
pub mod chat {
    // use reqwasm::websocket::futures::WebSocket;
    use yew::prelude::*;

    // use crate::app::websocket::websocket::WebSocketService;


    // use websocket::{Client, Message};
    // use websocket::client::request::Url;
    use web_sys::{ErrorEvent, MessageEvent, WebSocket};

    pub enum Msg {
        Add
    }

    pub struct Chat {
        count: i32,
        messages: Vec<String>,
    }

    impl Component for Chat {
        type Message = Msg;
        type Properties = ();

        fn create(ctx: &yew::Context<Self>) -> Self {
            // let task = self.wss.connect("ws://127.0.0.1:500/ws/", cbout, cbnot.into());
            // let mut ws = WebSocket::open("ws://127.0.0.1:5000/ws").unwrap();
            // let ws = WebSocket::new("ws://127.0.0.1:5000/ws/").unwrap();
            // let url = Url::parse("ws://127.0.0.1:5000/ws").unwrap(); // Get the URL
            // let request = Client::connect(url).unwrap(); // Connect to the server
            Self {
                count: 0,
                messages: vec![],
            }
        }

        fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
            match msg {
                Msg::Add => {
                    self.count += 1;
                    self.messages.push(format!("count: {}", self.count));
                },
            }
            true
        }

        fn view(&self, ctx: &yew::Context<Self>) -> yew::Html {
            let link = ctx.link();
            html! {
                <div class="flex w-screen">
                    <button onclick={link.callback(|_| Msg::Add)} class="btn btn-primary">{"+"}</button>
                    <span class="font-bold mx-3">{&format!(" {} ", self.count)}</span>    
                    <table class="table table-striped">
                        <thead>
                            <tr>
                                <th> { "Title cell" }</th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr>
                                { 
                                    self.messages.clone().iter().map(|message| {
                                        html!{ <tr><td> {{ message }} </td></tr> }
                                    }).collect::<Html>()
                                }
                            </tr>
                        </tbody>
                    </table>
                </div>
            }
        }
    }
}
