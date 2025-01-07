mod error;
mod whisper;

use axum::{
    extract::State,
    routing::post,
    Router,
    http::StatusCode,
    serve,
};
use bytes::Bytes;
use std::net::SocketAddr;
use whisper::WhisperTranscriber;
use std::sync::Arc;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), error::Error> {
    let transcriber = Arc::new(WhisperTranscriber::new()?);
    
    let app = Router::new()
        .route("/transcribe", post(transcribe))
        .with_state(transcriber);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("Listening on {}", addr);
    
    serve(listener, app).await.unwrap();

    Ok(())
}

async fn transcribe(
    State(transcriber): State<Arc<WhisperTranscriber>>,
    audio_data: Bytes,
) -> Result<String, (StatusCode, String)> {
    transcriber
        .transcribe(&audio_data)
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))
}
