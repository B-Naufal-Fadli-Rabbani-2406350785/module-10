use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
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

                <div style="
                    display:flex;
                    justify-content:space-between;
                    align-items:center;
                    margin-bottom:20px;
                ">
                    <div>
                        <h1 style="margin:0;">
                            { "YewChat" }
                        </h1>

                        <p style="
                            margin:0;
                            color:lightgray;
                        ">
                            { "Async Programming Tutorial" }
                        </p>
                    </div>

                    <div style="
                        background:#4CAF50;
                        width:12px;
                        height:12px;
                        border-radius:50%;
                    ">
                    </div>
                </div>

                <div style="
                    background:#2d2d2d;
                    padding:15px;
                    border-radius:15px;
                    height:300px;
                    overflow-y:auto;
                ">

                    <div style="
                        background:#3b82f6;
                        padding:10px;
                        border-radius:12px;
                        width:fit-content;
                        margin-bottom:10px;
                    ">
                        { "Alice: Hello everyone!" }
                    </div>

                    <div style="
                        background:#4CAF50;
                        padding:10px;
                        border-radius:12px;
                        width:fit-content;
                        margin-left:auto;
                        margin-bottom:10px;
                    ">
                        { "You: Hi Alice!" }
                    </div>

                    <div style="
                        background:#3b82f6;
                        padding:10px;
                        border-radius:12px;
                        width:fit-content;
                    ">
                        { "Alice: Welcome to YewChat!" }
                    </div>
                </div>

                <div style="
                    display:flex;
                    margin-top:20px;
                    gap:10px;
                ">
                    <input
                        type="text"
                        placeholder="Type your message..."
                        style="
                            flex:1;
                            padding:12px;
                            border:none;
                            border-radius:10px;
                            background:#2d2d2d;
                            color:white;
                        "
                    />

                    <button style="
                        padding:12px 20px;
                        border:none;
                        border-radius:10px;
                        background:linear-gradient(to right,#4CAF50,#45a049);
                        color:white;
                        cursor:pointer;
                        font-weight:bold;
                    ">
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