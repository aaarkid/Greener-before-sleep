use yew::prelude::*;
use yew_router::prelude::*;

use super::super::Route;

#[function_component(Dashboard)]
pub fn dashboard() -> Html {
    html! {
<>
    <section>
        <div class="navbar">
            <ul style="margin-top: 0px">
                //number of icons and a coin icon
                <img src="images/logo.png" alt="" style="padding-left: 5px; padding-top: 5px; height: 40px;"/>
                <li><Link<Route> to={Route::Home}>{"Logout"}</Link<Route>></li>
                <li style="
                color: white;
                text-align: center;
                padding: 14px 16px;
                text-decoration: none;
                "><img src="images/icons/coin.png" alt="coin" style="width: 20px; height: 20px; padding-right
                "/>{"80 Coins"}</li>
            </ul>
        </div>
    </section>

    //create a pane in the left with 4 buttons: Collect, Daily Tasks, Achievments, Plant
    <section>
        <div class="pane">
            <ul>
                //each button has a big icon a title under
                <li><img src="images/collect.png" alt="collect" style="width: 50; height: 50; margin-top: 5px;"/>{"Collect"}</li>
                <li><img src="images/daily.png" alt="daily" style="width: 50; height: 50; margin-top: 5px;"/>{"Daily Tasks"}</li>
                <li><img src="images/achievments.png" alt="achievments" style="width: 50; height: 50; margin-top: 5px;"/>{"Achievments"}</li>
                <li><img src="images/plant.png" alt="plant" style="width: 50; height: 50; margin-top: 5px;"/>{"Plant"}</li>
            </ul>
        </div>
    </section>
</>
    }
}