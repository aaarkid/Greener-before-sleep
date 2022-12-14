mod components;

use components::{login::*, dashboard::*, home::*};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq  )]
enum Route {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
    #[at("/collect")]
    Collect,
    #[at("/quests")]
    DailyTasks,
    #[at("/achievements")]
    Achievements,
    #[at("/plant")]
    Plant,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! { 
            <Home />
        },
        Route::Login => html! {
            <Login />
        },
        Route::Collect  => html! {
            <Collect />
        },
        Route::DailyTasks => html! {
            <DailyTasks />
        },
        Route::Achievements => html! {
            <Achievements />
        },
        Route::Plant => html! {
            <Plant />
        },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[function_component(Main)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)} />
        </BrowserRouter>
    }
}

fn main() {
    yew::start_app::<Main>();
}