use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;


pub enum Msg {
    InputValue(String),
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub line_number: usize,
    pub line: Vec<char>,
}

pub struct TypingLine {
    input: Vec<char>,
}

impl Component for TypingLine {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            input: vec![],
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::InputValue(value) => {
                self.input.clear();
                self.input.append(&mut value.chars().collect());
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let oninput = ctx.link().callback(|e: InputEvent| {
            let target: EventTarget = e
                .target().unwrap();
            Msg::InputValue(target.unchecked_into::<HtmlInputElement>().value())
        });
        return html! {
<div class={classes!("card")} style="padding: 18px;">
    <div class="mb-3 typing-form">
        <div class="typing-label">
            {
                for ctx.props().line.iter().enumerate().map(|(idx, chr)| {
                    html! {
                        <span key={idx} class={
                            if self.input.len() > idx {
                                if self.input.get(idx).unwrap() == chr {
                                    "span-true"
                                } else {
                                    "span-false"
                                }
                            } else {
                                ""
                            }
                        }>{ chr }</span>
                    }
                })
            }
        </div>
        <input type="text" class="typing-input" {oninput} />
    </div>
</div>
        };
    }
}