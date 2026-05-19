use futures_util::{SinkExt, StreamExt};
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast;
use tokio_websockets::{
    Message,
    ServerBuilder,
};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind(
        "127.0.0.1:8080"
    )
    .await
    .unwrap();

    println!(
        "WebSocket server running on ws://127.0.0.1:8080"
    );

    let (tx, _) =
        broadcast::channel::<String>(100);

    loop {
        let (stream, _) =
            listener.accept().await.unwrap();

        let tx = tx.clone();

        let rx = tx.subscribe();

        tokio::spawn(async move {
            handle_connection(stream, tx, rx).await;
        });
    }
}

async fn handle_connection(
    stream: TcpStream,
    tx: broadcast::Sender<String>,
    mut rx: broadcast::Receiver<String>,
) {
    let ws_stream = ServerBuilder::new()
        .accept(stream)
        .await
        .unwrap();

    let (mut ws_sender, mut ws_receiver) =
        ws_stream.split();

    let tx_clone = tx.clone();

    tokio::spawn(async move {
        while let Some(msg) =
            ws_receiver.next().await
        {
            if let Ok(msg) = msg {
                if let Some(text) = msg.as_text() {
                    tx_clone
                        .send(text.to_string())
                        .unwrap();
                }
            }
        }
    });

    while let Ok(msg) = rx.recv().await {
        ws_sender
            .send(Message::text(msg))
            .await
            .unwrap();
    }
}