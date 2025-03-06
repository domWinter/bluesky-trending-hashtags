use std::time::Duration;

use anyhow::Result;
use axum::{
    extract::{
        ws::{Message, WebSocket},
        State, WebSocketUpgrade,
    },
    http::StatusCode,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
use tokio::time::sleep;
mod config;
mod model;
use config::CONFIG;
use tokio::sync::watch;
use tokio::sync::watch::Receiver;
use tungstenite::{connect, Message as WSMessage};
use model::BlueSkyMsg;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let (mut socket, _response) = connect(
        "wss://jetstream2.us-east.bsky.network/subscribe?wantedCollections=app.bsky.feed.post",
    )
    .expect("Can't connect");

    let (tx, rx) = watch::channel(String::from(""));

    tokio::spawn(async move {
        loop {
            let mut buffer: Vec<BlueSkyMsg> = vec![];
            match socket.read() {
                Ok(msg) => {
                    match msg {
                        WSMessage::Text(txt) => {
                            println!("New message!");
                            let msg: BlueSkyMsg = serde_json::from_str(&txt).unwrap();
                            buffer.push(msg);
                            if buffer.len() >= 10 {
                                tx.send(serde_json::to_string(&buffer).unwrap()).unwrap();
                                buffer.clear();
                            }
                            
                        }
                        _ => {
                            println!("Connection closed");
                            break;
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error reading from socket: {:?}", e);
                    break;
                }
            }
        }

    });
   



    let app = Router::new()
        .route("/", get(root_get))
        .route("/ws/v1", get(ws_stream))
        .with_state(rx);

    log::info!(
        "Starting Webserver on port: {}:{}",
        CONFIG.server_addr,
        CONFIG.server_port
    );
    let listener =
        tokio::net::TcpListener::bind(format!("{}:{}", CONFIG.server_addr, CONFIG.server_port))
            .await?;
    axum::serve(listener, app).await?;

    Ok(())
}

async fn root_get() -> impl IntoResponse {
    let markup = match tokio::fs::read_to_string("src/index.html").await {
        Ok(m) => m,
        Err(err) => {
            log::error!("{}", err);
            return (StatusCode::INTERNAL_SERVER_ERROR, "").into_response();
        }
    };

    Html(markup).into_response()
}

async fn ws_stream(ws: WebSocketUpgrade, State(rx): State<Receiver<String>>) -> impl IntoResponse {
    ws.on_upgrade(|ws: WebSocket| async {
        let _ = get_trending(ws, rx).await;
    })
}

async fn get_trending(mut ws: WebSocket, mut rx:Receiver<String>) -> Result<()> {
    loop {
        let val = rx.borrow_and_update().clone();
        ws.send(Message::Text(val.into())).await?;
    }
}

