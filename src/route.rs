use yew::{html, Html};
use yew_router::prelude::*;

use crate::pages::{page_not_found::PageNotFound, home::Home, typing::Typing};

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/typing")]
    Typing,
    #[not_found]
    #[at("/404")]
    NotFound,
}


pub fn switch(routes: &Route) -> Html {
    let ret;
    match routes.clone() {
        Route::Home => {
            ret = html! { <Home /> }
        }
        Route::Typing => {
            ret = html! { <Typing /> }
        }
        Route::NotFound => {
            ret = html! { <PageNotFound /> }
        }
    }
    return ret;
}
