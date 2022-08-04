use yew_router::prelude::*;

#[derive(Debug, Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/boards")]
    BoardsList,
    #[at("/boards/:id")]
    BoardView { id: String },
}
