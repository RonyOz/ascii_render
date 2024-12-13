mod base {
    pub mod scroll;
    pub mod ascii; // This declares a module named ascii in the base module.
}

use base::{ascii::asciify, scroll};
use warp::Filter; // warp is a web server framework in Rust, and Filter is a trait that defines the core functionality of a filter.
use tokio::time::{Duration, sleep};

#[tokio::main] // This is an attribute that tells the Rust compiler to generate the main function that uses the Tokio runtime.
async fn main() { 

    const MIN_SCREEN_WIDTH: usize = 100;

    let stream_route = warp::path!(String)
        .map(|text: String|{
            let stream = warp::sse::reply(warp::sse::keep_alive().stream(async_stream::stream!{
                let mut scrolling_text = match asciify(&text, "broadway").await {
                    Ok(text) => text,
                    Err(_) => return,
                };

                let screen_width = if text.len()*20 > MIN_SCREEN_WIDTH {
                    text.len()*20 + 20
                } else {
                    MIN_SCREEN_WIDTH
                };

                loop {

                    // Send a screen cleaner
                    yield Ok::<_, warp::Error>(warp::sse::Event::default().data("\x1B[2J\x1B[1;1H"));

                    scrolling_text = scroll::scroll_text(&scrolling_text, screen_width).await;

                    yield Ok::<_, warp::Error>(warp::sse::Event::default().data(scrolling_text.clone()));
                    sleep(Duration::from_millis(50)).await;
                } 
            }));

            stream
        });

    warp::serve(stream_route) 
        .run(([127, 0, 0, 1], 3030)) 
        .await; // wait for the server to finish running.

}

