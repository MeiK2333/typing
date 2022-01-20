use std::{
    fmt::{Debug},
};
use serde::{Deserialize, Serialize};
use gloo::console;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::JsFuture;
use web_sys::{HtmlElement, HtmlInputElement, Request, RequestInit, RequestMode, Response};
use yew::prelude::*;
use crate::components::{typing_line::TypingLine};

#[derive(Debug, Serialize, Deserialize)]
pub struct Post {
    pub title: String,
    pub author: String,
    pub lines: Vec<String>,
}


pub enum Msg {
    Update,
    SetPost(JsValue),
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
    loading: bool,
    load_msg: String,
}

impl Component for Typing {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_future(async {
            let url = "https://poetrydb.org/random";
            let mut opts = RequestInit::new();
            opts.method("GET");
            opts.mode(RequestMode::Cors);
            let request = Request::new_with_str_and_init(url, &opts).unwrap();
            let window = web_sys::window().unwrap();
            let resp_value = JsFuture::from(window.fetch_with_request(&request)).await.unwrap();
            let resp: Response = resp_value.dyn_into().unwrap();
            Msg::SetPost(JsFuture::from(resp.json().unwrap()).await.unwrap())
        });
        Self {
            spans: vec![],
            lines: vec![],
            active_line: 0,
            loading: true,
            load_msg: "获取文章中".to_string(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Update => {
                true
            }
            Msg::SetPost(js_value) => {
                let mut posts: Vec<Post> = js_value.into_serde().unwrap();
                let post = posts.get_mut(0).unwrap();
                self.lines.clear();
                for line in &post.lines {
                    for char in line.chars() {
                        self.spans.push(char);
                    }
                    self.spans.push('\n');
                }
                self.load_msg = "加载文章中".to_string();
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
    if !self.loading {
        html! {
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
        }
    } else {
        html! {
            <div class="d-flex justify-content-center">
                <strong>{ self.load_msg.clone() }</strong>
                <div class="spinner-border" role="status">
                    <span class="visually-hidden">{ "Loading..." }</span>
                </div>
            </div>
        }
    }
}
{
    if self.spans.len() != 0 {
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

    fn rendered(&mut self, ctx: &Context<Self>, _first_render: bool) {
        if self.loading == false {
            if self.lines.len() > 0 {
                console::debug!(format!("focus line {}", self.active_line));
                self.lines.get_mut(self.active_line).unwrap()
                    .node_ref.cast::<HtmlElement>().unwrap()
                    .query_selector("input").unwrap().unwrap()
                    .dyn_into::<HtmlInputElement>().unwrap()
                    .focus().unwrap();
            }
            return;
        }
        if self.spans.len() != 0 {
            let document = web_sys::window().unwrap().document().unwrap();

            // 获取文章在页面上渲染的结果
            let spans = document
                .query_selector_all("#typing-hidden > span")
                .unwrap()
                .dyn_into::<web_sys::NodeList>().unwrap();

            let mut offset_top = 0;
            let mut pos = 0;
            for idx in 0..spans.length() {
                let elem = spans.item(idx).unwrap().dyn_into::<HtmlElement>().unwrap();
                // 对每个字符判断是否发生了换行
                if elem.offset_top() != offset_top {
                    if offset_top != 0 {
                        let mut left = pos;
                        let mut right = idx as usize;
                        // 忽略两端空白符号
                        while right > left && self.spans[left].is_whitespace() {
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
            self.spans.clear();
            self.loading = false;
            ctx.link().send_message(Msg::Update);
        }
    }
}