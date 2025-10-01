use std::io;
use std::io::Write;
use std::process::{Command, Stdio};
use std::thread;
use std::time::Duration;
use arboard::Clipboard;
use crate::editor::fuma_state::FumaState;
use crate::log_debug;

impl FumaState {
    fn with_clipboard<F, R>(&self, f: F) -> io::Result<R>
    where
        F: FnOnce(&mut Clipboard) -> io::Result<R>,
    {
        let mut guard = self.clipboard.lock().map_err(|_| {
            io::Error::new(io::ErrorKind::Other, "Failed to lock clipboard mutex")
        })?;
        if guard.is_none() {
            *guard = Some(Clipboard::new().map_err(|e| {
                io::Error::new(io::ErrorKind::Other, format!("Clipboard::new failed: {}", e))
            })?);
        }
        let cb = guard.as_mut().unwrap();
        f(cb)
    }

    fn wl_copy_with_args(text: &str, args: &[&str]) -> io::Result<()> {
        let mut cmd = Command::new("wl-copy");
        for a in args {
            cmd.arg(a);
        }
        let mut child = cmd
            .stdin(Stdio::piped())
            .stderr(Stdio::null())
            .spawn()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("wl-copy spawn failed: {}", e)))?;
        if let Some(mut stdin) = child.stdin.take() {
            stdin.write_all(text.as_bytes())?;
        }
        let status = child
            .wait()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("wl-copy wait failed: {}", e)))?;
        if !status.success() {
            return Err(io::Error::new(io::ErrorKind::Other, "wl-copy exited with non-zero status"));
        }
        Ok(())
    }

    fn wl_copy_try_both(text: &str) -> io::Result<()> {
        if Self::wl_copy_with_args(text, &["--fork"]).is_ok() {
            return Ok(());
        }
        Self::wl_copy_with_args(text, &[])
    }

    fn wl_paste() -> io::Result<String> {
        let output = Command::new("wl-paste")
            .output()
            .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("wl-paste failed: {}", e)))?;
        if !output.status.success() {
            return Err(io::Error::new(io::ErrorKind::Other, "wl-paste returned non-zero"));
        }
        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    fn normalize_for_compare_owned(s: &str) -> String {
        s.replace("\r\n", "\n").trim_end_matches('\n').to_string()
    }

    pub fn copy_selection_to_clipboard(&mut self) -> io::Result<()> {
        if let Some(selection) = &self.selected_text {
            let target = selection.text.as_str();
            let mut success = false;

            if Self::wl_copy_try_both(target).is_ok() {
                thread::sleep(Duration::from_millis(40));
                if let Ok(pasted) = Self::wl_paste() {
                    let n_pasted = Self::normalize_for_compare_owned(&pasted);
                    let n_target = Self::normalize_for_compare_owned(target);
                    log_debug!("Verification wl-paste after wl-copy: read='{}' target='{}'", n_pasted, n_target);
                    if n_pasted == n_target {
                        success = true;
                    }
                }
            } else {
                log_debug!("wl-copy not available or failed, trying arboard");
            }

            if !success {
                let arboard_set_result = self.with_clipboard(|cb| {
                    cb.set_text(target)
                        .map_err(|e| io::Error::new(io::ErrorKind::Other, format!("{}", e)))
                });

                match arboard_set_result {
                    Ok(()) => {
                        thread::sleep(Duration::from_millis(30));
                        if let Ok(read_ar) = self.with_clipboard(|cb| {
                            cb.get_text().map_err(|e| io::Error::new(io::ErrorKind::Other, format!("{}", e)))
                        }) {
                            let n_read_ar = Self::normalize_for_compare_owned(&read_ar);
                            let n_target = Self::normalize_for_compare_owned(target);
                            log_debug!("Verification arboard get after set: read='{}' target='{}'", n_read_ar, n_target);
                            if n_read_ar == n_target {
                                success = true;
                            }
                        }
                        let _ = Self::wl_copy_try_both(target);
                        thread::sleep(Duration::from_millis(40));
                        if !success {
                            if let Ok(pasted) = Self::wl_paste() {
                                let n_pasted = Self::normalize_for_compare_owned(&pasted);
                                let n_target = Self::normalize_for_compare_owned(target);
                                log_debug!("Verification wl-paste after arboard+wl-copy: read='{}' target='{}'", n_pasted, n_target);
                                if n_pasted == n_target {
                                    success = true;
                                }
                            }
                        }
                    }
                    Err(e) => {
                        log_debug!("arboard set_text failed: {}", e);
                    }
                }
            }

            if success {
                log_debug!("Text copied to clipboard: {}", selection.text);
                Ok(())
            } else {
                Err(io::Error::new(io::ErrorKind::Other, "Failed to set clipboard (wl-copy and arboard attempts failed)"))
            }
        } else {
            Ok(())
        }
    }

    pub fn cut_selection_to_clipboard(&mut self) -> io::Result<()> {
        if self.selected_text.is_some() {
            self.copy_selection_to_clipboard()?;

            self.delete_selected_text()?;

            self.selected_text = None;
        }
        self.resize_console()?;
        Ok(())
    }


    pub fn paste_from_clipboard(&mut self) -> io::Result<()> {
        let mut clipboard_text = match Self::wl_paste() {
            Ok(t) => t,
            Err(_) => {
                let ar = self.with_clipboard(|cb| {
                    cb.get_text().map_err(|e| io::Error::new(io::ErrorKind::Other, format!("{}", e)))
                })?;
                ar
            }
        };

        if self.selected_text.is_some() {
            self.delete_selected_text()?;
            self.delete_selection();
        }

        clipboard_text = clipboard_text.trim_end().to_string();

        for c in clipboard_text.chars() {
            if c == '\n' {
                self.insert_newline()?;
            } else {
                self.insert_char(c)?;
            }
        }

        log_debug!("Text pasted from clipboard");
        Ok(())
    }

    fn delete_selected_text(&mut self) -> io::Result<()> {
        if let Some(selection) = &self.selected_text {

            let (start_row, start_col) = self.wrap_result.get_logical_position(selection.row_start, selection.col_start)?;
            let (end_row, end_col) = self.wrap_result.get_logical_position(selection.row_end, selection.col_end)?;

            self.buffer.delete_selected_text(
                start_row,
                start_col,
                end_row,
                end_col
            )?;
            self.cursor.x = selection.col_start;
            self.cursor.y = selection.row_start;

            self.cursor.ensure_visible()?;
        }
        Ok(())
    }
}
