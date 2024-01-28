use crate::api::chat::handlers::on_connect;
use socketioxide::layer::SocketIoLayer;
use socketioxide::SocketIo;

pub fn get_chat_layer() -> SocketIoLayer {
    let (socket_layer, io) = SocketIo::new_layer();

    io.ns("/ws", on_connect);

    socket_layer
}
