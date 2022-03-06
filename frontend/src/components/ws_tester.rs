use super::ws_client::{new_ws_client, WSClient};
use futures::SinkExt;
use log;
use reqwasm::websocket::Message as WsMessage;
use yew::prelude::*;

#[derive(Debug)]
pub enum Msg {
    ButtonClicked,
    MessageSent,
    MessageReceived(String),
}

pub struct WSTester {
    client: WSClient,
}

impl Component for WSTester {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let scope = ctx.link().clone();
        let client = new_ws_client(move |message: WsMessage| match message {
            WsMessage::Text(value) => {
                scope.send_message(Msg::MessageReceived(value));
            }
            WsMessage::Bytes(_value) => {
                log::info!("Bytes message");
            }
        });
        Self { client: client }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ButtonClicked => {
                let mut client = self.client.clone();
                ctx.link().send_future(async move {
                    client
                        .sender
                        .send(WsMessage::Text(String::from("msg1")))
                        .await
                        .unwrap();
                    Msg::MessageSent
                });
                false
            }
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
