use yew::prelude::*;
use yew_router::prelude::*;
use super::super::Route;

#[function_component(Home)]
pub fn home() -> Html {
    // //read from file quote.txt and load the text as string
    // let file = std::fs::File::open("quote.txt").unwrap();
    // let mut contents = Vec::new();
    // std::io::Read::read_to_end(&mut std::io::BufReader::new(file), &mut contents).unwrap();
    // let quote = std::str::from_utf8(&contents).unwrap().to_owned();
    html! {
    <>
        <section>
            <div class="navbar">
                <ul style="margin-top: 0px">
                    <img src="images/logo.png" alt="" style="padding-left: 5px; padding-top: 5px; height: 40px;"/>
                    <li><a href="#">{"Home"}</a></li>
                    <li><Link<Route> to={Route::Login}>{"Login"}</Link<Route>></li>
                    <li><a href="#">{"Contact"}</a></li>
                    <li><a href="#">{"About"}</a></li>
                </ul>
            </div>
        </section>

        <section>
            <div class="container">
                    <div class="item">
                        <img src="images/bg/img.png" alt=""/>
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