use std::time::Duration;

use futures_util::{StreamExt, SinkExt};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use rand::prelude::*;

/* 
    Esse é o agente, e sua função é enviar mensagens para o notificador

    Mensage: Números Aleatórios a cada 5 segundos
*/

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    match connect_async("ws://127.0.0.1:8888").await{
        Ok((ws_stream, _)) => {
           
           let (mut sink, _stream) = ws_stream.split();

           loop {
                let number = gen_number().await?;
                let message = Message::Text(number.to_string().into());
            
                if let Err(e) = sink.send(message).await {
                    println!("Error sending message: {e}");
                    break;
                }

                tokio::time::sleep(Duration::from_secs(5)).await;
            }
            
        },
        Err(e) => println!("{}", e)

    }

    Ok(())
}

async fn gen_number() -> Result<i32, Box<dyn std::error::Error>> {
    let mut rng = rand::rng();

    let num = rng.random_range(-100..100);

    Ok(num)
}