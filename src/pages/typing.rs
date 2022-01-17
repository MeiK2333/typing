use gloo::console;
use wasm_bindgen::JsCast;
use web_sys::{HtmlElement};
use yew::prelude::*;
use crate::components::{typing_line::TypingLine};

pub enum Msg {
    Update
}

pub struct Typing {
    spans: Vec<char>,
    lines: Vec<Vec<char>>,
}

impl Component for Typing {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            spans: include_str!("../data/1.txt").chars().collect(),
            lines: vec![],
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Update => {
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        console::debug!("review");
        return html! {
<>
{
    for self.lines.iter().enumerate().map(|(idx, line)| {
        html! {
            <TypingLine line_number={idx} line={line.clone()}></TypingLine>
        }
    })
}
{
    if self.lines.len() == 0 {
        html!{
            <div class="card typing-hidden" style="padding: 18px;">
                <div class="mb-3 typing-form typing-hidden">
                    <div class="typing-label" id="typing-hidden">
                        {
                            self.spans.iter().enumerate().map(|(idx, chr)| {
                                if *chr != '\n' {
                                    html!{<span key={idx} id={ format!("typing-text-{}", idx) }>{ format!("{}", chr) }</span>}
                                } else {
                                    html!{ <span key={idx} id={ format!("typing-text-{}", idx) }><br /></span> }
                                }
                            }).collect::<Html>()
                        }
                    </div>
                </div>
            </div>
        }
    } else {
        html! {}
    }
}
</>
        };
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if !first_render {
            return;
        }
        let document = web_sys::window().unwrap().document().unwrap();

        let spans = document
            .query_selector_all("#typing-hidden > span")
            .unwrap()
            .dyn_into::<web_sys::NodeList>().unwrap();

        self.lines.clear();

        let mut offset_top = 0;
        let mut pos = 0;
        for idx in 0..spans.length() {
            let elem = spans.item(idx).unwrap().dyn_into::<HtmlElement>().unwrap();
            if elem.offset_top() != offset_top {
                if offset_top != 0 {
                    let line = self.spans[pos as usize..idx as usize].to_vec();
                    pos = idx;
                    if line.clone().into_iter().any(|x| x != '\n') {
                        self.lines.push(line);
                    }
                }
                offset_top = elem.offset_top();
            }
        }

        ctx.link().send_message(Msg::Update);
    }
}