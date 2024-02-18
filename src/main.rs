use axum::{
    routing::get,
    Router,
};
use std::process::Command;
use std::net::SocketAddr;

async fn play_video() -> &'static str {
    let video_path = "videos/test/video.mp4";

    match Command::new("vlc")
        .args(["--fullscreen", video_path])
        .spawn() {
            Ok(_) => "Attempting to play video...",
            Err(_) => "Failed to launch VLC. Is it installed and in your PATH?",
        }
}


#[tokio::main]
async fn main() {
    // Define the app routes
    let app = Router::new().route("/play", get(play_video));

    // Specify the address to run the server on
    let addr: SocketAddr = SocketAddr::from(([127, 0, 0, 1], 1337));
    println!("Listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}