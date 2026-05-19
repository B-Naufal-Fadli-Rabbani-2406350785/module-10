use futures_util::{SinkExt, StreamExt};
use gloo_net::websocket::{futures::WebSocket, Message};
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
struct ChatState {
    messages: Vec<String>,
}

enum ChatAction {
    AddMessage(String),
}

impl Reducible for ChatState {
    type Action = ChatAction;

    fn reduce(
        self: std::rc::Rc<Self>,
        action: Self::Action,
    ) -> std::rc::Rc<Self> {
        match action {
            ChatAction::AddMessage(msg) => {
                let mut messages =
                    self.messages.clone();

                messages.push(msg);

                ChatState { messages }.into()
            }
        }
    }
}

#[function_component(App)]
fn app() -> Html {
    let state = use_reducer(|| ChatState {
        messages: vec![],
    });

    let input_text = use_state(|| "".to_string());

    let writer = use_mut_ref(|| None);

    {
        let state = state.clone();
        let writer = writer.clone();

        use_effect_with((), move |_| {
            spawn_local(async move {
                let ws = WebSocket::open(
                    "ws://127.0.0.1:8080"
                )
                .unwrap();

                let (write, mut read) =
                    ws.split();

                *writer.borrow_mut() = Some(write);

                while let Some(msg) = read.next().await {
                    if let Ok(Message::Text(text)) = msg {
                        state.dispatch(
                            ChatAction::AddMessage(text),
                        );
                    }
                }
            });

            || ()
        });
    }

    let oninput = {
        let input_text = input_text.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement =
                e.target_unchecked_into();

            input_text.set(input.value());
        })
    };

    let onclick = {
        let writer = writer.clone();
        let input_text = input_text.clone();

        Callback::from(move |_| {
            let msg = (*input_text).clone();

            if msg.is_empty() {
                return;
            }

            let writer = writer.clone();

            spawn_local(async move {
                if let Some(w) =
                    writer.borrow_mut().as_mut()
                {
                    w.send(Message::Text(msg))
                        .await
                        .unwrap();
                }
            });

            input_text.set(String::new());
        })
    };

    html! {
        <div style="
            background: linear-gradient(to right, #141e30, #243b55);
            min-height: 100vh;
            color: white;
            font-family: Arial, sans-serif;
            display: flex;
            justify-content: center;
            align-items: center;
        ">
            <div style="
                width: 500px;
                background: #1e1e1e;
                border-radius: 20px;
                padding: 20px;
                box-shadow: 0px 0px 20px rgba(0,0,0,0.5);
            ">

                <h1>{ "YewChat" }</h1>

                <div style="
                    background:#2d2d2d;
                    padding:15px;
                    border-radius:15px;
                    height:300px;
                    overflow-y:auto;
                    margin-bottom:20px;
                ">
                    {
                        for state.messages.iter().map(|msg| html!{
                            <div style="
                                background:#3b82f6;
                                padding:10px;
                                border-radius:12px;
                                margin-bottom:10px;
                            ">
                                { msg }
                            </div>
                        })
                    }
                </div>

                <div style="
                    display:flex;
                    gap:10px;
                ">
                    <input
                        type="text"
                        placeholder="Type message..."
                        value={(*input_text).clone()}
                        oninput={oninput}
                        style="
                            flex:1;
                            padding:12px;
                            border:none;
                            border-radius:10px;
                            background:#2d2d2d;
                            color:white;
                        "
                    />

                    <button
                        onclick={onclick}
                        style="
                            padding:12px 20px;
                            border:none;
                            border-radius:10px;
                            background:#4CAF50;
                            color:white;
                            cursor:pointer;
                        "
                    >
                        { "Send" }
                    </button>
                </div>
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}