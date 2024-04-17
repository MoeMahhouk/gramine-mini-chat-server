use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::Mutex as TokioMutex;

// Define a simple struct to represent a chat message
#[derive(Debug)]
struct ChatMessage {
    sender: String,
    message: String,
}

// Function to handle client connections
async fn handle_client(mut stream: TcpStream, chat_messages: Arc<TokioMutex<Vec<ChatMessage>>>) {
    println!("Client connected: {:?}", stream.peer_addr().unwrap());

    loop {
        let mut buffer = [0; 1024];
        match stream.read(&mut buffer).await {
            Ok(bytes_read) => {
                if bytes_read == 0 {
                    println!("Client disconnected: {:?}", stream.peer_addr().unwrap());
                    break;
                }
                let message = String::from_utf8_lossy(&buffer[..bytes_read]).to_string();
                let sender = stream.peer_addr().unwrap().to_string();
                let chat_message = ChatMessage { sender, message: message.clone() };

                let mut messages = chat_messages.lock().await;
                messages.push(chat_message);

                // Broadcast message to all connected clients
                for message in messages.iter() {
                    if let Err(_) = stream.write_all(format!("{}: {}\n", message.sender, message.message).as_bytes()).await {
                        break;
                    }
                }
            }
            Err(_) => {
                println!("Error reading from client");
                break;
            }
        }
    }
}

#[tokio::main]
async fn main() {
    let addr = "127.0.0.1:8080";
    let chat_messages = Arc::new(TokioMutex::new(Vec::new()));

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("Server listening on {}", addr);

    loop {
        match listener.accept().await {
            Ok((stream, _)) => {
                let chat_messages = Arc::clone(&chat_messages);
                tokio::spawn(async move {
                    handle_client(stream, chat_messages).await;
                });
            }
            Err(e) => {
                println!("Connection failed: {}", e);
            }
        }
    }
}
