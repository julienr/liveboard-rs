use futures::channel::mpsc::UnboundedSender;
use futures::{future, pin_mut};
use futures::{SinkExt, StreamExt};
use gloo_net::websocket::{futures::WebSocket, Message as WsMessage};

type WsSender = UnboundedSender<WsMessage>;

pub struct WSClient {
    pub sender: WsSender,
}

pub fn new_ws_client<F>(board_id: i32, handle_message: F) -> WSClient
where
    F: 'static + Fn(WsMessage),
{
    let window = web_sys::window().unwrap();
    let location = window.location();
    let url = format!(
        "ws://{}:{}/api/boards/{}/ws",
        location.hostname().unwrap(),
        location.port().unwrap(),
        board_id,
    );
    log::info!("url= {:?}", url);
    let ws = WebSocket::open(&url).unwrap();
    let (write, mut read) = ws.split();

    let (mut read_tx, mut read_rx) = futures::channel::mpsc::unbounded();
    let (write_tx, write_rx) = futures::channel::mpsc::unbounded();

    let fwd_writes = write_rx.map(Ok).forward(write);
    let fwd_reads = async move {
        while let Some(m) = read.next().await {
            read_tx.send(m).await.unwrap()
        }
    };
    wasm_bindgen_futures::spawn_local(async move {
        pin_mut!(fwd_writes, fwd_reads);
        future::select(fwd_writes, fwd_reads).await;
    });

    wasm_bindgen_futures::spawn_local(async move {
        while let Some(m) = read_rx.next().await {
            match m {
                Ok(WsMessage::Text(value)) => {
                    handle_message(WsMessage::Text(value));
                }
                Ok(WsMessage::Bytes(value)) => {
                    handle_message(WsMessage::Bytes(value));
                }
                Err(err) => {
                    log::info!("Error: {}", err);
                }
            }
        }
    });

    WSClient { sender: write_tx }
}

impl WSClient {}

impl Clone for WSClient {
    fn clone(&self) -> Self {
        WSClient {
            sender: self.sender.clone(),
        }
    }
}
