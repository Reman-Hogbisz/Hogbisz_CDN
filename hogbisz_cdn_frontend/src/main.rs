pub mod components;
pub mod pages;
pub mod route;

use crate::route::Route;
use yew::prelude::*;
use yew_router::prelude::*;

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Login => html! {
           <crate::pages::login::view::Login />
        },
        Route::Main => html! {
            <crate::pages::main_page::view::Main />
        },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}

fn main() {
    yew::start_app::<App>();
}
