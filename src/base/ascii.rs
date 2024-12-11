pub fn text_to_ascii_art(text: &str) -> String { 
    let mut result = String::new(); // This creates a new empty string to store the result.

    for c in text.chars() { 
        let ascii_value = c as u32; // This converts the character to its ASCII value.
        let ascii_art = format!("{} ", ascii_value); // This formats the ASCII value as a string.
        result.push_str(&ascii_art); // This appends the ASCII art to the result string.
    
    }
    result // return
}