pub struct WrapResult {
    pub wrapped_text: String,
    pub wrap_ids: Vec<usize>,
}
pub fn wrap_content(content: &str, width: usize) -> WrapResult {
    let effective_width = width.saturating_sub(2).max(1); // Aseguramos al menos 1

    let mut result = Vec::new();
    let mut wrap_ids = Vec::new();

    for (logical_idx, line) in content.lines().enumerate() {
        let mut remaining = line;

        while !remaining.is_empty() {
            let chunk: String = remaining.chars().take(effective_width).collect();
            let byte_len = chunk.len();
            remaining = &remaining[byte_len..];

            result.push(chunk);
            wrap_ids.push(logical_idx);
        }

        if line.is_empty() {
            result.push(String::new());
            wrap_ids.push(logical_idx);
        }
    }

    WrapResult {
        wrapped_text: result.join("\n"),
        wrap_ids,
    }
}