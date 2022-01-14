use gloo::console;
use yew::{html, Component, Context, Html};
use yew_router::prelude::*;


mod route;
mod pages;
mod components;

use components::{header::Header};

pub enum Msg {}

pub struct App {}

impl App {}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        console::debug!("Hello World!");

        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        return html! {
<BrowserRouter>
    <Header></Header>
    <div class="container" style="margin-top: 35px; background-color: #f8f9fa; padding: 0;">
        <main>
            <Switch<route::Route> render={Switch::render(route::switch)} />
        </main>
    </div>
</BrowserRouter>
        };
    }
}


fn main() {
    yew::start_app::<App>();
}
