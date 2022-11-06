use yew::prelude::*;
use yew_router::prelude::*;
use super::dashboard::DashSection;

use super::super::Route;

#[function_component(Login)]
pub fn login() -> Html {
    html! {
    <div class="content">
        <div class="column" id="left">
            <div class="form_container">

            <form class="form" id="login">
                <h1 class="form__title">{"Login"}</h1>
                <div class="form__message form__message--error"></div>
                <div class="form__input-group">
                    <input type="text" class="form__input" autofocus=true placeholder="Username or email"/>
                    <div class="form__input-error-message"></div>
                </div>
                <div class="form__input-group">
                    <input type="password" class="form__input" autofocus=true placeholder="Password"/>
                    <div class="form__input-error-message"></div>
                </div>
                <Link<Route> to={Route::Dashboard {section:{DashSection::Collect}}}><button class="form__button" type="submit">{"Continue"}</button></Link<Route>>
                <p class="form__text">
                    <a class="form__link" href="./" id="linkCreateAccount">{"Don't have an account? Create account"}</a>
                </p>
            </form>

            
            </div>

        </div>
        <script src="main.js"></script>

        <div class="column" id="right">
            <h2>{
r#"Column 2"#}
            </h2>
            <div class="para">
            <p>{
r#"It all starts with a morning notification....
It's a scientifically proven fact that people can be motivated very easily,
when devices that are at their fingertips at all times give them this ounce 
of inspiration. With this type of innovation, you can help build a better 
and greener future for yourself and the community around you just 
by doing simple mundane tasks everyday. Believe it, because it's possible! 
Even the smallest contribution can eventually result in being a beautiful 
planted tree someday."#} </p>
            </div>

        </div>
    </div>
    }
}