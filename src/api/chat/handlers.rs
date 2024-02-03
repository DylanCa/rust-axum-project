use log::info;
use serde_json::Value;
use socketioxide::extract::{AckSender, Bin, Data, SocketRef};
use crate::api::chat::models::{InMessage, OutMessage};

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
        |socket: SocketRef, Data::<InMessage>(mut data), Bin(bin)| {
            info!("Received event: {:?} {:?}", data, bin);

            let response = OutMessage {
                user_id: format!("user-{}", socket.id),
                datetime: chrono::Utc::now(),
                message: data.message,
            };

            let _ = socket.within(data.room).emit("message", response);
        },
    );
}
