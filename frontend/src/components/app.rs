use super::api_health_checker::APIHealthChecker;
use super::board::Board;
use yew::prelude::*;

pub enum Msg {}

pub struct App {}

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
                <Board />
            </div>
        }
    }
}
