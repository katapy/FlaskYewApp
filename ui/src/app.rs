
pub mod router;
pub mod user_inner;
pub mod websocket;

pub mod app{
    use yew::prelude::*;

    use yew_router::BrowserRouter;
    use yew_router::Switch;

    use crate::app::router::router::switch;
    use crate::app::router::router::Route;
    use crate::app::user_inner::user_inner::{User, get_user_state};

    #[function_component(App)]
    pub fn app() -> Html  {
        let ctx = get_user_state();

        html! {
            <ContextProvider<User> context={(*ctx).clone()}>
                <BrowserRouter>
                    <div class="flex w-screen h-screen">
                        <h1> { "OurFractal" } </h1>
                        <Switch<Route> render={Switch::render(switch)}/>
                    </div>
                </BrowserRouter>
            </ContextProvider<User>>
        }
    }
}
