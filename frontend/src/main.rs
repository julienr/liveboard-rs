use log;
use reqwasm::http::Request;
use shared::datatypes::HealthResponse;
use std::{
    error::Error,
    fmt::{self, Debug, Display, Formatter},
};
use yew::prelude::*;

enum Msg {}

struct App {}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="main">
                <APIHealthChecker />
            </div>
        }
    }
}

#[derive(Debug)]
struct APIError {
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
enum Msg2 {
    Call,
    ReceiveResponse(HealthResponse),
    APIError(APIError),
}

struct APIHealthChecker {
    last_response: String,
}

impl Component for APIHealthChecker {
    type Message = Msg2;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            last_response: String::from("Not called yet"),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg2::Call => {
                ctx.link().send_future(async {
                    match fetch_health().await {
                        Ok(resp) => Msg2::ReceiveResponse(resp),
                        Err(err) => Msg2::APIError(err),
                    }
                });
                false
            }
            Msg2::ReceiveResponse(resp) => {
                log::info!("resp: {:?}", resp);
                self.last_response = String::from("value1: ") + &resp.value1;
                true
            }
            Msg2::APIError(err) => {
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
                <button onclick={link.callback(|_| Msg2::Call)}>{ "Check API health" }</button>
            </div>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
