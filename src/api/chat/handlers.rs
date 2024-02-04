use crate::api::chat::models::{ChatMessage, IncomingMessage};
use serde_json::Value;
use socketioxide::extract::{Bin, Data, SocketRef};
use tracing::info;

pub fn on_connect(socket: SocketRef, Data(data): Data<Value>) {
    info!("Socket.IO connected: {:?} {:?}", socket.ns(), socket.id);
    socket.emit("auth", data).ok();

    socket.on("join", |socket: SocketRef, Data::<String>(room)| {
        info!("Received join: {:?}", room);
        let _ = socket.leave_all();
        let _ = socket.join(room);
    });

    socket.on(
        "message",
        |socket: SocketRef, Data::<IncomingMessage>(data), Bin(bin)| {
            info!("Received event: {:?} {:?}", data, bin);

            let response = ChatMessage::new(data.room_id, socket.id.to_string(), data.message);
            let _ = socket
                .within(response.clone().room_id())
                .emit("message", response);
        },
    );
}
