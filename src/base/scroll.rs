// this module scroll a string in the terminal in a infinite loop

use std::{thread, time};

use super::ascii::asciify; // This imports the thread and time modules from the standard library.


pub async fn scroll_text(text: &str, width: usize) -> String {
    let delay = time::Duration::from_millis(1000);  // scroll speed

    let ascii_text = asciify(text, "broadway").await.unwrap();

    print!("{}", " ".repeat(width));
    loop {
        for i in 0..ascii_text.len() - width {
            print!("\r{}", &ascii_text[i..i + width]);
            thread::sleep(delay);
        }
    }
}