mod components;
use crate::components::board::Board;
use crate::components::boards_list::BoardsList;
use yew_router::prelude::{BrowserRouter, Redirect, Switch};
mod api;
mod live_cursor;
mod routes;
mod utils;
use routes::Route;
use yew::prelude::*;

fn switch(route: &Route) -> Html {
    log::info!("Switch route: {:?}", route);
    match route {
        Route::Home => {
            return html! {
                <Redirect<Route> to={Route::BoardsList} />
            }
        }
        Route::BoardsList => {
            return html! {
                <BoardsList />
            }
        }
        Route::BoardView { id } => {
            return html! {
                <Board />
            }
        }
    }
}

#[function_component(Main)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    log::info!("Hello from rust!");
    yew::start_app::<Main>();
}
