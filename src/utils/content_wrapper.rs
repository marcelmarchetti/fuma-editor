pub struct WrapResult {
    pub wrapped_text: String,
    pub wrap_ids: Vec<usize>,
}

pub fn wrap_content(content: &str, width: usize) -> WrapResult {
    let effective_width = width.saturating_sub(2).max(1);

    let mut wrapped_text = String::new();
    let mut wrap_ids = Vec::new();

    for (logical_idx, line) in content.lines().enumerate() {
        if line.is_empty() {
            wrapped_text.push('\n');
            wrap_ids.push(logical_idx);
            continue;
        }

        let mut start = 0;
        let mut count = 0;

        for (i, ch) in line.char_indices() {
            count += 1;

            if count == effective_width {
                wrapped_text.push_str(&line[start..=i]);
                wrapped_text.push('\n');
                wrap_ids.push(logical_idx);

                start = i + ch.len_utf8();
                count = 0;
            }
        }

        if start < line.len() {
            wrapped_text.push_str(&line[start..]);
            wrapped_text.push('\n');
            wrap_ids.push(logical_idx);
        }
    }

    if wrapped_text.ends_with('\n') {
        wrapped_text.pop();
    }

    WrapResult {
        wrapped_text,
        wrap_ids,
    }
}
