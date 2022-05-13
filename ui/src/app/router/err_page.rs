
pub mod err_page {
    use yew::{Component, html};
    use yew_router::prelude::*;

    use crate::app::router::router::Route;

    pub enum Msg {
    }

    pub struct ErrPage {

    }

    impl Component for ErrPage {
        type Message = Msg;

        type Properties = ();

        fn create(_ctx: &yew::Context<Self>) -> Self {
            ErrPage {  }
        }

        fn view(&self, _ctx: &yew::Context<Self>) -> yew::Html {
            html! {
                <div class="bg-gray-800 flex w-screen"> 
                    <h2>{ " -404- NOT FOUND" }</h2>
                    <form class="m-4 flex">
                        <Link<Route> to={Route::Login}>
                            <button>
                                {"Go to Login Page"}
                            </button>
                        </Link<Route>>
                    </form>
                </div>
            }
        }
    }
}
