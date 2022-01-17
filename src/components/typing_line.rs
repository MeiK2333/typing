use wasm_bindgen::JsCast;
use web_sys::{EventTarget, HtmlInputElement};
use yew::prelude::*;
use gloo::console;


pub enum Msg {
    InputValue(String),
    Input(u32),
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub line_number: usize,
    pub line: Vec<char>,
    pub disabled: bool,
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

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::InputValue(value) => {
                self.input.clear();
                self.input.append(&mut value.chars().collect());
                true
            }
            Msg::Input(code) => {
                // 如果此时该行输入已经够而且输入空格时，完成该行输入
                if self.input.len() >= ctx.props().line.len() {
                    if code == 32 {
                        // TODO: 移动至下一行
                        console::debug!(format!("line: {} done", ctx.props().line_number));
                        return true
                    }
                }
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let oninput = ctx.link().callback(|e: InputEvent| {
            let target: EventTarget = e
                .target().unwrap();
            Msg::InputValue(target.unchecked_into::<HtmlInputElement>().value())
        });
        let onkeypress = ctx.link().callback(|e: KeyboardEvent| {
            Msg::Input(e.char_code())
        });
        return html! {
<div class={classes!("card", ctx.props().disabled.then(|| "typing-form-disabled"))} style="padding: 18px;">
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
        <input type="text" class="typing-input" {oninput} {onkeypress} disabled={ctx.props().disabled} />
    </div>
</div>
        };
    }
}