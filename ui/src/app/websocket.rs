
pub mod websocket {
    // use reqwasm::websocket::{futures::WebSocket, Message};
    use futures::{channel::mpsc::Sender, SinkExt, StreamExt};

    pub struct WebSocketService {
        pub tx: Sender<String>,
    }

    impl WebSocketService {
        /*
        pub fn new() -> Self {
            let mut ws = WebSocket::open("ws://127.0.0.1:8000/ws").unwrap();
            let (in_tx, mut in_rx) = futures::channel::mpsc::channel::<String>(1000);
            Self{
                tx: in_tx,
            }
        }
        */
    }
}
