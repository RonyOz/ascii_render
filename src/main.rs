mod base {
    pub mod scroll;
    pub mod ascii; // This declares a module named ascii in the base module.
}

use base::scroll;
use warp::Filter; // warp is a web server framework in Rust, and Filter is a trait that defines the core functionality of a filter.

#[tokio::main] // This is an attribute that tells the Rust compiler to generate the main function that uses the Tokio runtime.
async fn main() { // The main function is an asynchronous function that returns a future.

    let scroll_text = warp::path!("scroll" / String) // This creates a filter that matches the /scroll/:text/:width path.
        .map(|text: String| { // This maps the filter to a closure that takes the text and width parameters.
            scroll::scroll_text(&text, 80); // This calls the scroll_text function from the base::scroll module.
        });

    warp::serve(scroll_text) 
        .run(([127, 0, 0, 1], 3030)) 
        .await; // wait for the server to finish running.

}
