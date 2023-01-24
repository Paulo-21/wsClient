use async_tungstenite::{tokio::connect_async, tungstenite::Message};
use futures::prelude::*;
use std::io;
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = "ws://127.0.0.1:8082/websocketChess/Friend/?user_name=Paulo";
    //let url = "ws://127.0.0.1:8082/websocketChess/Paul/?name=Paulo";

    let (mut _ws_stream, _) = connect_async(url).await?;
    //let (mut sender, mut receiver) = _ws_stream.split();
    
    if let Some(Ok(Message::Text(text))) = _ws_stream.next().await {
        _ws_stream.close(None).await.unwrap();
        
        let url = format!("ws://127.0.0.1:8082/websocketChess/Game/{}/{}", text, "?user_name=Paulo");
        let (mut _ws_stream, _) = connect_async(url).await?;
        let (mut sender, mut receiver) = _ws_stream.split();
    
        let mut send_task = tokio::spawn(async move {
            loop {
                let mut buffer = String::new();
                io::stdin().read_line(&mut buffer).unwrap();
                
                if sender.send(Message::Text(buffer)).await.is_err() {
                    break;
                }
            }
        });
        let mut recv_task = tokio::spawn(async move {
            while let Some(Ok(Message::Text(text))) = receiver.next().await {
                // Add username before message.
                println!("{}", text);
            }
        });

        tokio::select! {
            _ = (&mut send_task) => recv_task.abort(),
            _ = (&mut recv_task) => send_task.abort(),
        };
    }
    //s.send(Message::text(text)).await?;

    //let msg = r.next().await.ok_or("didn't receive anything").unwrap().unwrap();

    //println!("Received: {:?}", msg);

    Ok(())
}

