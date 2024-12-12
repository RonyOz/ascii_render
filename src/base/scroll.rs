// this module scroll a string in the terminal in a infinite loop

use std::{thread, time};
use scraper;
use super::ascii::asciify;

pub async fn scroll_text(text: &str, width: usize) -> String {
    let delay = time::Duration::from_millis(1000);  // scroll speed

    let ascii_text = asciify(text, "broadway").await.unwrap(); 

    // Assuming the asciify function returns an HTML string, we need to extract the body content
    let body_content = extract_body_content(&ascii_text);
    body_content
}

// Function to extract the body content from the HTML
fn extract_body_content(html: &str) -> String {
    let document = scraper::Html::parse_document(html);
    let selector = scraper::Selector::parse("pre").unwrap();
    let body_content = document.select(&selector).next().unwrap().inner_html();
    body_content
}