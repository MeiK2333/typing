use gloo::console;
use yew::prelude::*;
use yew_router::prelude::*;
use yew_router::scope_ext::HistoryHandle;

use crate::route::Route;

pub enum Msg {
    UpdateRoute(AnyHistory),
}

pub struct Header {
    _listener: HistoryHandle,
}

impl Component for Header {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let listener = ctx.link()
            .add_history_listener(ctx.link().callback(Msg::UpdateRoute))
            .unwrap();
        Self {
            _listener: listener
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateRoute(history) => {
                console::debug!(history.location().pathname());
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let path = ctx.link().location().unwrap().pathname();
        return html! {
<nav class="navbar navbar-expand navbar-light bg-light fix-top">
    <div class="container">
        <Link<Route> classes={classes!("navbar-brand")} to={Route::Home}>{"Home"}</Link<Route>>
        <div class="collapse navbar-collapse">
            <ul class="navbar-nav mr-auto">
                <li class="nav-item">
                    <Link<Route> classes={classes!("nav-link", (path == "/typing").then(|| Some("active")))} to={Route::Typing}>{"Typing"}</Link<Route>>
                </li>
                <li class="nav-item">
                    <Link<Route> classes={classes!("nav-link", (path == "/404").then(|| Some("active")))} to={Route::NotFound}>{"NotFound"}</Link<Route>>
                </li>
            </ul>
        </div>
    </div>
</nav>
        };
    }
}