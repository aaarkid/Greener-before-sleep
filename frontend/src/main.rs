mod components;

use components::{login::*, dashboard::*};
use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
    #[at("/dashboard")]
    Dashboard,
    #[not_found]
    #[at("/404")]
    NotFound,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! { 
            <Home />
        },
        Route::Login => html! {
            <Login />
        },
        Route::Dashboard => html! {
            <Dashboard />
        },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}

#[function_component(Home)]
fn home() -> Html {
    html! {
    <>
        <section>
            <div class="navbar">
                <ul style="margin-top: 0px">
                    <img src="images/logo.png" alt="" style="padding-left: 5px; padding-top: 5px; height: 40px;"/>
                    <li><a href="#">{"Home"}</a></li>
                    <li><a href="#">{"Login"}</a></li>
                    <li><a href="#">{"Contact"}</a></li>
                    <li><a href="#">{"About"}</a></li>
                </ul>
            </div>

        </section>

        <section>
            <div class="container">
                    <div class="item">
                        <img src="images/img2.png" alt=""/>
                        <div class="cover">
                            <div class="container">
                                <div class="header-content">
                                    <div class="line"></div>
                                    <h1>{"\""}</h1>
                                    <h1>{"Don't litter. It makes the world bitter."}</h1>
                                    <h1>{"\""}</h1>
                                </div>
                            </div>
                        </div>
                    </div>
            </div>

        </section>
    </>
    }
}

#[function_component(Main)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    yew::start_app::<Main>();
}