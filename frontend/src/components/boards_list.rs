use crate::api::{create_board, fetch_boards, APIError};
use crate::routes::Route;
use shared::datatypes::Board;
use web_sys::{HtmlFormElement, HtmlInputElement};
use yew::html::Scope;
use yew::prelude::*;
use yew_router::components::Link;

pub struct BoardsList {
    loading: bool,
    boards: Vec<Board>,
    create_form_ref: NodeRef,
    create_form_name_ref: NodeRef,
}

#[derive(Debug)]
pub enum Msg {
    BoardsRefreshed(Vec<Board>),
    APIError(APIError),
    CreateBoard(String),
    RefreshBoards,
}

impl Component for BoardsList {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(Msg::RefreshBoards);
        Self {
            loading: true,
            boards: vec![],
            create_form_ref: NodeRef::default(),
            create_form_name_ref: NodeRef::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        log::info!("Msg::{:?}", msg);
        match msg {
            Msg::BoardsRefreshed(resp) => {
                self.boards = resp;
                self.loading = false;
                true
            }
            Msg::APIError(err) => {
                log::error!("error: {:?}", err);
                true
            }
            Msg::CreateBoard(name) => {
                let form = self.create_form_ref.cast::<HtmlFormElement>().unwrap();
                form.reset();
                ctx.link().send_future(async move {
                    create_board(name).await.unwrap();
                    Msg::RefreshBoards
                });
                false
            }
            Msg::RefreshBoards => {
                self.loading = true;
                ctx.link().send_future(async {
                    match fetch_boards().await {
                        Ok(resp) => Msg::BoardsRefreshed(resp),
                        Err(err) => Msg::APIError(err),
                    }
                });
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
          <main class="container" aria-busy={ if self.loading { "true" } else { "false" } }>
            { self.view_boards(&self.boards) }
            { self.view_new_board_form(ctx.link()) }
          </main>
        }
    }
}

impl BoardsList {
    fn view_board_card(&self, board: &Board) -> Html {
        html! {
          <Link<Route> to={Route::BoardView { id: board.id.to_string() }}>
            <div class="card center">
              <p>{&board.name}</p>
              <p>{&board.id}</p>
            </div>
          </Link<Route>>
        }
    }
    fn view_boards(&self, boards: &[Board]) -> Html {
        html! {
          <article>
            <h3>{"Boards"}</h3>
            <div class="cards-container">
            { boards.iter().map(|b| self.view_board_card(b)).collect::<Html>() }
            </div>
          </article>
        }
    }
    fn view_new_board_form(&self, link: &Scope<Self>) -> Html {
        let name_ref = self.create_form_name_ref.clone();
        let onclick = link.callback(move |_e: MouseEvent| {
            let name_input = name_ref.cast::<HtmlInputElement>().unwrap();
            let name = name_input.value();
            Msg::CreateBoard(name)
        });
        html! {
          <article>
            <h3>{"Create new board"}</h3>
            <form ref={self.create_form_ref.clone()}>
              <label for="name">{"Name"}</label>
              <input ref={self.create_form_name_ref.clone()} id="name" type="text" name="name" /><br/>
              <input {onclick} type="submit" value="Create" />
            </form>
          </article>
        }
    }
}
