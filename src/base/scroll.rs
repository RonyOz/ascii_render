const SKIP: usize = 1;

pub async fn scroll_text(current_text: &str, width: usize) -> String {
    let text = current_text.to_string();
    let text = text
        .lines()
        .map(|line| {
            let line = line.trim_end();
            let line_lenght = line.len();
            
            if (line_lenght + SKIP) > width { // if the text is out of the screen | Overflow
                let overflow = &line[line_lenght-SKIP..]; // save overflow
                format!("{}{}", overflow, &line[..line_lenght-SKIP].to_string()) // update text frame

            } else {
                let spaces = " ".repeat(SKIP);
                format!("{}{}", spaces, line)
            }

        })
        .collect::<Vec<String>>()
        .join("\n");

    text
}