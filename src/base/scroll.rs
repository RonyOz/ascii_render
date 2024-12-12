pub async fn scroll_text(text: &str, width: usize) -> String {
    
    // insert spaces in ascii art

    let text = text.to_string();

    let text = text
        .lines()
        .map(|line| {
            let line = line.trim_end();
            let spaces = " ".repeat(width);
            format!("{}{}", spaces, line)
        })
        .collect::<Vec<String>>()
        .join("\n");

    text
}