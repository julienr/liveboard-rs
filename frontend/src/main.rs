use yew::prelude::*;
use reqwasm::http::Request;

enum Msg {
    AddOne,
}

struct App {
    value: i64,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            value: 0,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.value += 2;
                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        // This gives us a component's "`Scope`" which allows us to send messages, etc to the component.
        let link = ctx.link();
        html! {
            <div class="main">
                <button onclick={link.callback(|_| Msg::AddOne)}>{ "+1" }</button>
                <p>{ self.value }</p>
                <APITester />
            </div>
        }
    }
}

#[derive(Debug)]
enum Msg2 {
    Call
}

struct APITester {
}

impl Component for APITester {
    type Message = Msg2;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg2::Call => {
                log::info!("APITester {:?}", msg);
                // https://yew.rs/docs/tutorial
                wasm_bindgen_futures::spawn_local(async move {
                    let resp = Request::get("http://localhost:8000/test")
                        .send()
                        .await
                        .unwrap()
                        .json()
                        .await
                        .unwrap();
                    log::info!("resp: {:?}", resp);
                });
                return true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {
            <button onclick={link.callback(|_| Msg2::Call)}>{ "Test API" }</button>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}