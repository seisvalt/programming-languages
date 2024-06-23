use rust_socketio::{Socket, Payload};
use std::time::Duration;

fn main() {
    let server = Socket::builder()
        .on("message", |payload: Payload, socket| {
            match payload {
                Payload::String(data) => {
                    println!("Message from client: {}", data);
                    socket.emit("message", format!("Echo: {}", data)).unwrap();
                }
                Payload::Binary(_) => println!("Binary data"),
            }
        })
        .on("connect", |_, socket| {
            println!("Client connected: {}", socket.id());
        })
        .on("disconnect", |_, _| {
            println!("Client disconnected");
        })
        .build()
        .unwrap();

    server.listen("127.0.0.1:3000").unwrap();
}
