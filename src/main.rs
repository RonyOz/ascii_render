
fn pixel_to_ascii(value:u8) -> char {
    
    match value {
        0..=50 => '@',
        51..=100 => '#',
        101..=150 => '*',
        151..=200 => '+',
        _ => ' ',
    }

}



fn main() {
   let pixels = vec![0, 55, 120, 180, 220];
   for pixel in pixels {
        print!("{}",pixel_to_ascii(pixel));
   }  
}
