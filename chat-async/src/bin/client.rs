use futures_util::{SinkExt, StreamExt};
use http::Uri;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio_websockets::{ClientBuilder, Message};

#[tokio::main]
async fn main() -> Result<(), tokio_websockets::Error> {
    let (mut ws_stream, _) =
        ClientBuilder::from_uri(Uri::from_static("ws://127.0.0.1:2000"))
            .connect()
            .await?;

    let (mut sender, mut receiver) = ws_stream.split();
    let stdin = BufReader::new(tokio::io::stdin());
    let mut lines = stdin.lines();

    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(message)) = receiver.next().await {
            if let Some(msg) = message.as_text() {
                println!("{}", msg);
            }
        }
    });

    let mut send_task = tokio::spawn(async move {
        while let Ok(Some(line)) = lines.next_line().await {
            if sender.send(Message::text(line)).await.is_err() {
                break;
            }
        }
    });

    tokio::select! {
        _ = (&mut recv_task) => send_task.abort(),
        _ = (&mut send_task) => recv_task.abort(),
    }

    Ok(())
}