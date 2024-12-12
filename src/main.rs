mod base {
    pub mod scroll;
    pub mod ascii; // This declares a module named ascii in the base module.
}

use warp::{filters::ws::{Message, WebSocket}, Filter}; // warp is a web server framework in Rust, and Filter is a trait that defines the core functionality of a filter.
use futures::sink::SinkExt; // Import the SinkExt trait to use the send method on WebSocket.

#[tokio::main] // This is an attribute that tells the Rust compiler to generate the main function that uses the Tokio runtime.
async fn main() { 

    let ws_route = warp::path!(String)
        .and(warp::ws())
        .map(|text: String, ws: warp::ws::Ws| {
            ws.on_upgrade(move |socket| scroll_text(socket, text))
        });

    warp::serve(ws_route) 
        .run(([127, 0, 0, 1], 3030)) 
        .await; // wait for the server to finish running.

}
async fn scroll_text(mut socket: WebSocket, text: String) {
    let mut scrolling_text = text;

    loop {
        clean_screen();
        if socket.send(Message::text(&scrolling_text)).await.is_err() {
            break;
        }
    }
}

fn clean_screen() {
    print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
}
