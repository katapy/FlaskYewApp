
pub mod chat;
pub mod err_page;

pub mod router {
    use web_sys::HtmlInputElement;
    use yew_router::prelude::*;
    use yew::prelude::*;

    use crate::app::user_inner::user_inner::User;
    use crate::app::router::chat::chat::Chat;
    use crate::app::router::err_page::err_page::ErrPage;

    #[derive(Clone, Routable, PartialEq)]
    pub enum Route {
        #[at("/")]
        Login,
        #[at("/secure")]
        Secure,
        #[at("/chat")]
        Chat,
        #[not_found]
        #[at("/404")]
        NotFound,
    }

    #[function_component(Secure)]
    fn secure() -> Html {
        let history = use_history().unwrap();

        let onclick = Callback::once(move |_| history.push(Route::Login));
        html! {
            <div>
                <h2>{ "Secure" }</h2>
                <button {onclick}>{ "Go Home" }</button>
            </div>
        }
    }

    #[function_component(Login)]
    pub fn login() -> Html {
        let username = use_state(|| String::new());
        let user = use_context::<User>().expect("No context found.");

        let oninput = {
            let current_username = username.clone();
    
            Callback::from(move |e: InputEvent| {
                let input: HtmlInputElement = e.target_unchecked_into();
                current_username.set(input.value());
            })
        };

        let onclick = {
            let username = username.clone();
            let user = user.clone();
            Callback::from(move |_| *user.username.borrow_mut() = (*username).clone())
        };
        
        html! {
            <div class="bg-gray-800 flex w-screen">
                <form class="m-4 flex">
                    <input {oninput} placeholder="Username"/>
                    <Link<Route> to={Route::Chat}> 
                        <button {onclick} disabled={username.len()<1} >
                            {"Go Chatting!"}
                        </button>
                    </Link<Route>>
                </form>
            </div>
        }
    }

    pub fn switch(routes: &Route) -> Html {
        match routes {
            Route::Login => html! { 
                <Login />
            },
            Route::Secure => html! {
                <Secure />
            },
            Route::Chat => html! {<Chat/>},
            Route::NotFound => html! {<ErrPage/>}
        }
    }
}
