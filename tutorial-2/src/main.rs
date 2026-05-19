use futures_util::{SinkExt, StreamExt};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use tokio::{
    net::{TcpListener, TcpStream},
    sync::mpsc,
};
use tokio_websockets::{Message, ServerBuilder};

type Tx = mpsc::UnboundedSender<String>;
type PeerMap = Arc<Mutex<HashMap<usize, Tx>>>;

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:8080";

    let listener = TcpListener::bind(addr)
        .await
        .expect("Cannot bind");

    println!("WebSocket server running on ws://{}", addr);

    let peers: PeerMap = Arc::new(Mutex::new(HashMap::new()));

    let mut id_counter = 0;

    loop {
        let (stream, _) = listener.accept().await.unwrap();

        id_counter += 1;

        let peer_id = id_counter;

        let peers = peers.clone();

        tokio::spawn(async move {
            handle_connection(peer_id, peers, stream).await;
        });
    }
}

async fn handle_connection(
    id: usize,
    peers: PeerMap,
    stream: TcpStream,
) {
    let ws_stream = ServerBuilder::new()
        .accept(stream)
        .await
        .unwrap();

    println!("Client {} connected", id);

    let (mut ws_sender, mut ws_receiver) = ws_stream.split();

    let (tx, mut rx) = mpsc::unbounded_channel::<String>();

    peers.lock().unwrap().insert(id, tx);

    let peers_for_broadcast = peers.clone();

    let receive_task = tokio::spawn(async move {
        while let Some(msg_result) = ws_receiver.next().await {
            let msg = msg_result.unwrap();

            if msg.is_text() {
                let text = msg.as_text().unwrap();

                let message = format!("Client {}: {}", id, text);

                println!("{}", message);

                let peers = peers_for_broadcast.lock().unwrap();

                for (&peer_id, tx) in peers.iter() {
                    if peer_id != id {
                        let _ = tx.send(message.clone());
                    }
                }
            }
        }
    });

    let send_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            ws_sender
                .send(Message::text(msg))
                .await
                .unwrap();
        }
    });

    tokio::select! {
        _ = receive_task => (),
        _ = send_task => (),
    }

    peers.lock().unwrap().remove(&id);

    println!("Client {} disconnected", id);
}