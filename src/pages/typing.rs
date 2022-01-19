use gloo::console;
use wasm_bindgen::JsCast;
use web_sys::{HtmlElement, HtmlInputElement};
use yew::prelude::*;
use crate::components::{typing_line::TypingLine};

pub enum Msg {
    Update,
    LineDone(usize, Vec<char>),
}

struct LineState {
    text: Vec<char>,
    disabled: bool,
    node_ref: NodeRef,
    line_number: usize,
    value: Vec<char>,
}

pub struct Typing {
    spans: Vec<char>,
    lines: Vec<LineState>,
    active_line: usize,
}

impl Component for Typing {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            spans: include_str!("../data/1.txt").chars().collect(),
            lines: vec![],
            active_line: 0,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Update => {
                true
            }
            Msg::LineDone(line, value) => {
                let state = self.lines.get_mut(line).unwrap();
                state.value = value;
                state.disabled = true;

                // 判断是否有下一行
                if line < self.lines.len() {
                    self.active_line = line + 1;
                    let next_line = self.lines.get_mut(line + 1).unwrap();
                    next_line.disabled = false;
                }
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_done = ctx.link().callback(|(number_line, value)| {
            Msg::LineDone(number_line, value)
        });
        return html! {
<>
{
    for self.lines.iter().enumerate().map(|(idx, line)| {
        html! {
            <TypingLine
                key={idx}
                ref={line.node_ref.clone()}
                disabled={line.disabled}
                line_number={line.line_number}
                line={line.text.clone()}
                on_done={on_done.clone()}
            ></TypingLine>
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
            console::debug!(format!("focus line {}", self.active_line));
            self.lines.get_mut(self.active_line).unwrap()
                .node_ref.cast::<HtmlElement>().unwrap()
                .query_selector("input").unwrap().unwrap()
                .dyn_into::<HtmlInputElement>().unwrap()
                .focus().unwrap();
            return;
        }
        let document = web_sys::window().unwrap().document().unwrap();

        let spans = document
            .query_selector_all("#typing-hidden > span")
            .unwrap()
            .dyn_into::<web_sys::NodeList>().unwrap();

        self.lines.clear();

        // TODO: 此处需要单独抽离出来作为模块
        let mut offset_top = 0;
        let mut pos = 0;
        for idx in 0..spans.length() {
            let elem = spans.item(idx).unwrap().dyn_into::<HtmlElement>().unwrap();
            if elem.offset_top() != offset_top {
                if offset_top != 0 {
                    let mut left = pos;
                    let mut right = idx as usize;
                    while self.spans[left].is_whitespace() {
                        left += 1;
                    }
                    while right > left && self.spans[right - 1].is_whitespace() {
                        right -= 1;
                    }
                    let line = self.spans[left as usize..right as usize].to_vec();
                    pos = idx as usize;
                    if line.clone().into_iter().any(|x| x != '\n') {
                        self.lines.push(LineState {
                            text: line,
                            disabled: self.lines.len() != 0,
                            node_ref: Default::default(),
                            line_number: self.lines.len(),
                            value: vec![],
                        });
                    }
                }
                offset_top = elem.offset_top();
            }
        }

        ctx.link().send_message(Msg::Update);
    }
}