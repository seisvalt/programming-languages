use rust_socketio::{SocketBuilder, Payload};

fn main() {
    let socket = SocketBuilder::new("http://localhost:3000")
        .on("connect", |_| {
            println!("Connected to server");
        })
        .on("message", |payload: Payload, _| {
            if let Payload::String(data) = payload {
                println!("Message from server: {}", data);
            }
        })
        .on("disconnect", |_| {
            println!("Disconnected from server");
        })
        .connect()
        .unwrap();

    socket.emit("message", "Hello Server").unwrap();

    // Keep the client running
    loop {}
}
