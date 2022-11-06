use yew::prelude::*;

#[function_component(Login)]
pub fn login() -> Html {
    html! {
        <form>
            <label for="username">{"Username"}</label>
            <input type="text" id="username" name="username" />
            <label for="password">{"Password"}</label>
            <input type="password" id="password" name="password" />
            <input type="submit" value="Login" />
        </form>
    }
}