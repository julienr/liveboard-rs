use futures::channel::mpsc::UnboundedSender;
use futures::{future, pin_mut};
use futures::{SinkExt, StreamExt};
use log;
use reqwasm::websocket::{futures::WebSocket, Message as WsMessage};
use yew::prelude::*;

#[derive(Debug)]
pub enum Msg {
    ButtonClicked,
    SendMessage,
    MessageSent,
    MessageReceived(String),
}

type WsSender = UnboundedSender<WsMessage>;

pub struct WSTester {
    sender: WsSender,
}

impl Component for WSTester {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let window = web_sys::window().unwrap();
        let location = window.location();
        let url = format!(
            "ws://{}:{}/ws/",
            location.hostname().unwrap(),
            location.port().unwrap(),
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

        let scope = ctx.link().clone();
        wasm_bindgen_futures::spawn_local(async move {
            while let Some(m) = read_rx.next().await {
                match m {
                    Ok(WsMessage::Text(value)) => {
                        scope.send_message(Msg::MessageReceived(value));
                    }
                    Ok(WsMessage::Bytes(_value)) => {
                        log::info!("Bytes message");
                    }
                    Err(err) => {
                        log::info!("Error: {}", err);
                    }
                }
            }
        });

        ctx.link().send_message(Msg::SendMessage);

        Self { sender: write_tx }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ButtonClicked => {
                let mut sender2 = self.sender.clone();
                ctx.link().send_future(async move {
                    sender2
                        .send(WsMessage::Text(String::from("msg1")))
                        .await
                        .unwrap();
                    Msg::MessageSent
                });
                false
            }
            Msg::SendMessage => false,
            Msg::MessageSent => {
                log::info!("MessageSent");
                false
            }
            Msg::MessageReceived(value) => {
                // TODO: Add to list of received message and display in the DOM
                log::info!("MessageReceived: {}", value);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {
            <div style="margin-top: 10px;">
                <button onclick={link.callback(|_| Msg::ButtonClicked)}>{ "Test websocket" }</button>
            </div>
        }
    }
}
