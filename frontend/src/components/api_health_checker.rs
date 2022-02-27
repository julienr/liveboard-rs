use log;
use reqwasm::http::Request;
use shared::datatypes::HealthResponse;
use std::{
    error::Error,
    fmt::{self, Debug, Display, Formatter},
};
use yew::prelude::*;

#[derive(Debug)]
pub struct APIError {
    message: String,
}
impl Display for APIError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        Debug::fmt(&self.message, f)
    }
}
impl Error for APIError {}

// https://github.com/yewstack/yew/blob/master/examples/futures/src/main.rs
async fn fetch_health() -> Result<HealthResponse, APIError> {
    let resp = Request::get("/api/health")
        .send()
        .await
        .unwrap()
        .json::<HealthResponse>()
        .await
        .unwrap();

    Ok(resp)
}

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
            <div>
                <p>{ &self.last_response }</p>
                <button onclick={link.callback(|_| Msg::ButtonClicked)}>{ "Check API health" }</button>
            </div>
        }
    }
}
