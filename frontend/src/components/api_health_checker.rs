use crate::api::{fetch_health, APIError};
use log;
use shared::datatypes::HealthResponse;
use std::fmt::Debug;
use yew::prelude::*;

#[derive(Debug)]
pub enum Msg {
    ButtonClicked,
    ReceiveResponse(HealthResponse),
    APIError(APIError),
}

pub struct APIHealthChecker {
    last_response: String,
}

impl Component for APIHealthChecker {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            last_response: String::from("Not called yet"),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::ButtonClicked => {
                ctx.link().send_future(async {
                    match fetch_health().await {
                        Ok(resp) => Msg::ReceiveResponse(resp),
                        Err(err) => Msg::APIError(err),
                    }
                });
                false
            }
            Msg::ReceiveResponse(resp) => {
                log::info!("resp: {:?}", resp);
                self.last_response = String::from("value1: ") + &resp.value1;
                true
            }
            Msg::APIError(err) => {
                log::error!("error: {:?}", err);
                self.last_response = String::from("ERROR: ") + &err.message;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {
            <div class="overlay">
                <p>{ &self.last_response }</p>
                <button onclick={link.callback(|_| Msg::ButtonClicked)}>{ "Check API health" }</button>
            </div>
        }
    }
}
