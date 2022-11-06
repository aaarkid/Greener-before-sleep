use yew::prelude::*;

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
                <button class="form__button" type="submit">{"Continue"}</button>
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
            <p>{
r#"Some text..."#}
            </p>

        </div>
    </div>
    }
}