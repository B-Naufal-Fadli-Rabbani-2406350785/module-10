use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div style="
            background:#1e1e1e;
            color:white;
            min-height:100vh;
            padding:20px;
            font-family:Arial;
        ">
            <h1>{ "Yew WebChat" }</h1>

            <p>
                { "Tutorial 3 Async Programming" }
            </p>

            <div style="
                margin-top:20px;
                background:#2d2d2d;
                padding:15px;
                border-radius:10px;
            ">
                <p>{ "Alice: Hello!" }</p>
                <p>{ "Bob: Hi there!" }</p>
            </div>

            <div style="margin-top:20px;">
                <input
                    type="text"
                    placeholder="Type message..."
                    style="
                        padding:10px;
                        width:300px;
                        border-radius:8px;
                        border:none;
                    "
                />

                <button style="
                    margin-left:10px;
                    padding:10px 20px;
                    border:none;
                    border-radius:8px;
                    background:#4CAF50;
                    color:white;
                ">
                    { "Send" }
                </button>
            </div>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}