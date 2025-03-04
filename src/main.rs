use serde::{Deserialize, Serialize};
use tungstenite::{connect, Message};

#[derive(Debug, Serialize, Deserialize)]
struct BlueSkyMsg {
    commit: Option<Commit>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Commit {
    record: Option<Record>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Record {
    tags: Option<Vec<String>>,
    text: String,
}

fn main() {
    let (mut socket, response) = connect(
        "wss://jetstream2.us-east.bsky.network/subscribe?wantedCollections=app.bsky.feed.post",
    )
    .expect("Can't connect");

    for (header, _value) in response.headers() {
        println!("* {header}");
    }

    loop {
        match socket.read() {
            Ok(msg) => {
                println!("New msg!");
                match msg {
                    Message::Text(txt) => {
                        let msg: BlueSkyMsg = serde_json::from_str(&txt).unwrap();
                        println!("New msg: {:?}", msg);
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
}
