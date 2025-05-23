use futures_util::{SinkExt, StreamExt};
use std::error::Error;
use std::net::SocketAddr;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::broadcast::{channel, Sender};
use tokio_websockets::{Message, ServerBuilder, WebSocketStream};

async fn handle_connection(
    addr: SocketAddr,
    mut ws_stream: WebSocketStream<TcpStream>,
    bcast_tx: Sender<String>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut bcast_rx = bcast_tx.subscribe();
    let (mut sender, mut receiver) = ws_stream.split();

    // Menerima pesan dari klien dan broadcast ke semua
    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(message)) = receiver.next().await {
            if let Some(msg) = message.as_text() {
                let full_msg = format!("From {addr}: {msg}");
                let _ = bcast_tx.send(full_msg);
            }
        }
    });

    // Kirim pesan broadcast ke klien
    let mut send_task = tokio::spawn(async move {
        while let Ok(msg) = bcast_rx.recv().await {
            let _ = sender.send(Message::text(msg)).await;
        }
    });

    tokio::select! {
        _ = (&mut recv_task) => send_task.abort(),
        _ = (&mut send_task) => recv_task.abort(),
    }

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error + Send + Sync>> {
    let (bcast_tx, _) = channel(16);
    let listener = TcpListener::bind("127.0.0.1:2000").await?;
    println!("Server listening on ws://127.0.0.1:2000");

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("New connection from {addr}");

        let bcast_tx = bcast_tx.clone();
        tokio::spawn(async move {
            let (_req, ws_stream) = ServerBuilder::new().accept(socket).await.unwrap();
            handle_connection(addr, ws_stream, bcast_tx).await.unwrap();
        });
    }
}