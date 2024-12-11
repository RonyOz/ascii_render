mod base {
    pub mod ascii; // This declares a module named ascii in the base module.
}

use warp::Filter; // warp is a web server framework in Rust, and Filter is a trait that defines the core functionality of a filter.

#[tokio::main] // This is an attribute that tells the Rust compiler to generate the main function that uses the Tokio runtime.
async fn main() { // The main function is an asynchronous function that returns a future.

    let ascii_route = warp::path!("ascii" / String) // The path! macro is used to create a filter that matches the specified path.
        .map(|input:String| {
            let ascii_art = base::ascii::text_to_ascii_art(&input); // This calls the text_to_ascii_art function from the ascii module.
            warp::reply::html(ascii_art) // This creates an HTML response with the ASCII art.
        } );

    warp::serve(ascii_route) 
        .run(([127, 0, 0, 1], 3030)) 
        .await; // wait for the server to finish running.

}
