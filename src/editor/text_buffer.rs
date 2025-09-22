pub struct TextBuffer {
    pub lines: Vec<String>,
}

impl TextBuffer {
    pub fn from_string(text: String) -> Self {
        let lines = text.lines().map(|l| l.to_string()).collect();
        Self { lines }
    }

    pub fn to_string(&self) -> String {
        self.lines.join("\n")
    }

    pub fn insert_char(&mut self, line: usize, col: usize, c: char) {
        if let Some(l) = self.lines.get_mut(line) {
            if col <= l.len() {
                l.insert(col, c);
            }
        }
    }

    pub fn remove_char(&mut self, line: usize, col: usize) {
        if let Some(l) = self.lines.get_mut(line) {
            if col < l.len() {
                l.remove(col);
            }
        }
    }

    pub fn line_count(&self) -> usize {
        self.lines.len()
    }

    pub fn get_line(&self, idx: usize) -> Option<&str> {
        self.lines.get(idx).map(|s| s.as_str())
    }
    pub fn insert_newline(&mut self, line: usize, col: usize) {
        if line < self.lines.len() {
            let current = self.lines[line].clone();
            let (before, after) = current.split_at(col);
            self.lines[line] = before.to_string();
            self.lines.insert(line + 1, after.to_string());
        } else {
            self.lines.push(String::new());
        }
    }

    pub fn backspace(&mut self, line: usize, col: usize) {
        if line < self.lines.len() {
            if col > 0 {
                self.lines[line].remove(col - 1);
            } else if line > 0 {
                let removed = self.lines.remove(line);
                let prev_line = &mut self.lines[line - 1];
                prev_line.push_str(&removed);
            }
        }
    }

    pub fn delete(&mut self, line: usize, col: usize) {
        if line < self.lines.len() {
            if col < self.lines[line].len() {
                self.lines[line].remove(col);
            } else if line + 1 < self.lines.len() {
                let next = self.lines.remove(line + 1);
                self.lines[line].push_str(&next);
            }
        }
    }
}
