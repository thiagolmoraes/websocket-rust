use futures_util::{TryStreamExt, StreamExt};
use tokio::net::TcpListener;
use tokio_tungstenite::accept_async;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "127.0.0.1:8888";

    let tcp_listener = TcpListener::bind(addr).await?;

    while let Ok((tcp_stream, addr)) = tcp_listener.accept().await {
        println!("Incoming TCP connection from: {}", addr);

        match accept_async(tcp_stream).await {
            Ok(ws_stream) => {
                println!("WebSocket connection established: {}", addr);

                let (_send, recv) = ws_stream.split();

                recv.try_for_each(|msg| async move {
                    println!("{msg:#?}");
                    Ok(())
                }).await?;

            },
            Err(e) => println!("Error: {e}"),
        }
        
    } 

    Ok(())
}