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
    #[at("/dashboard/:section")]
    Dashboard { section: DashSection },
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
        Route::Dashboard {section} => html! {
            match section {
                DashSection::Collect => html! {
                    <Dashboard section={DashSection::Collect} />
                },
                DashSection::DailyTasks => html! {
                    <Dashboard section={DashSection::DailyTasks} />
                },
                DashSection::Achievements => html! {
                    <Dashboard section={DashSection::Achievements} />
                },
                DashSection::Plant => html! {
                    <Dashboard section={DashSection::Plant} />
                },
            }
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