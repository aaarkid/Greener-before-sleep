use std::{fmt, str::FromStr};

use yew::prelude::*;
use yew_router::prelude::*;

use super::super::Route;

#[derive(Clone, PartialEq, Debug)]
pub enum DashSection {
    Collect,
    DailyTasks,
    Achievements,
    Plant,
}

impl Default for DashSection {
    fn default() -> Self {
        Self::Collect
    }
}

impl fmt::Display for DashSection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Collect => write!(f, "collect"),
            Self::DailyTasks => write!(f, "daily-tasks"),
            Self::Achievements => write!(f, "achievements"),
            Self::Plant => write!(f, "plant"),
        }
    }
}

impl FromStr for DashSection {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "collect" => Ok(Self::Collect),
            "daily-tasks" => Ok(Self::DailyTasks),
            "achievements" => Ok(Self::Achievements),
            "plant" => Ok(Self::Plant),
            _ => Err(()),
        }
    }
}

#[derive(Clone, PartialEq, Debug, Properties)]
pub struct DashProps {
    pub section: DashSection,
}

#[function_component(Dashboard)]
pub fn dashboard() -> Html {
    html! {
<>
    <section>
        <div class="navbar">
            <ul style="margin-top: 0px; margin-bottom: 0px;">
                //number of icons and a coin icon
                <img src="images/logo.png" alt="" style="padding-left: 5px; padding-top: 5px; height: 40px;"/>
                <li><Link<Route> to={Route::Home}>{"Logout"}</Link<Route>></li>
                <li style="
                color: white;
                text-align: center;
                text-decoration: none;
                padding: 14px 16px;
                "><img src="images/icons/coin.png" alt="coin" style="width: 15px; height: 15px;"/>{"80 Coins"}</li>
            </ul>
        </div>
    </section>

    //create a pane in the left with 4 buttons: Collect, Daily Tasks, Achievments, Plant
    <section>
        <div class="pane">
            <ul style="background:none;">
                //each button has a big icon a title under
                <li><Link<Route> to={Route::Collect}><img src="images/icons/collect.png" alt="collect" style="width: 60px; height: 60px; margin-top: 5px;"/></Link<Route>>{"Collect"}</li>
                <li><Link<Route> to={Route::DailyTasks}><img src="images/icons/daily.png" alt="daily" style="width: 60px; height: 60px; margin-top: 5px;"/></Link<Route>>{"Daily Tasks"}</li>
                <li><Link<Route> to={Route::Achievements}><img src="images/icons/achievments.png" alt="achievments" style="width: 60px; height: 60px; margin-top: 5px;"/></Link<Route>>{"Achievements"}</li>
                <li><Link<Route> to={Route::Plant}><img src="images/icons/quest.png" alt="plant" style="width: 60px; height: 60px; margin-top: 5px;"/></Link<Route>>{"Plant"}</li>
            </ul>
        </div>
    </section>
</>
    }
}

#[function_component(Collect)]
pub fn collect() -> Html {
    html! {
    <>
    <Dashboard />
        <div class="rowcollect">
            <div class="columncollect">
            <img src="images/bottlecap.png"/>
            <img src="images/chemicalwaste.png"/>
            </div>
            <div class="columncollect">
            <img src="images/cigarette butt1.png"/>
            <img src="images/glass bottle.png"/>
            </div>
            <div class="columncollect">
            <img src="images/leftoverfood.png"/>
            <img src="images/metals.png"/>
            </div>
            <div class="columncollect">
            <img src="images/paperbag.png"/>
            <img src="images/plasticbag.png"/>
            </div>
            <div class="columncollect">
            <img src="images/plasticbottle.png"/>
            <img src="images/plasticcup.png"/>
            </div>
            <div class="columncollect">
            <img src="images/plasticpackage.png"/>
            <img src="images/plasticstraw.png"/>
            </div>
        </div>
    </>
    }
}

#[function_component(DailyTasks)]
pub fn daily_tasks() -> Html {
    html! {
    <>
    <Dashboard />
        <div class="rowdaily">
            <div class="columndaily">
            <img src="images/bg/garbage.png"/><h3>{"Paper Champion"}</h3>{"Collect 2 paper trash."}
            </div>
            <div class="columndaily">
            <img src="images/bg/group_of_people.png"/><h3>{"Wzzz Bizz"}</h3>{"Collect a battery or chemical waste."}
            </div>
            <div class="columndaily">
            <img src="images/bg/wholesome.png"/><h3>{"Grateful Hero"}</h3>{"Hug your loved ones."}
            </div>
        </div>
    </>
    }
}

#[function_component(Achievements)]
pub fn achievments() -> Html {
    html! {
    <>
    <Dashboard />
        <div class="rowachiev">
            <div class="columnachiev">
            <img src="images/icons/bronze trash cleaned.png"/>
            <img src="images/icons/silver trash cleaned.png"/>
            <img style="filter: grayscale(1.0);" src="images/icons/gold trash cleaned.png"/>
            <br/>
            <img src="images/icons/bronze plastic hero.png"/>
            <img src="images/icons/silver plastic hero.png"/>
            <img style="filter: grayscale(1.0);" src="images/icons/gold plastic hero.png"/>
            <br/>
            <img src="images/icons/bronze science wizz.png"/>
            <img style="filter: grayscale(1.0);" src="images/icons/silver science wizz.png"/>
            <img style="filter: grayscale(1.0);" src="images/icons/gold science wizz.png"/>
            </div>
            <div class="columnachiev">
            <img style="filter: grayscale(1.0);" src="images/icons/bronze beech tree.png"/>
            <img style="filter: grayscale(1.0);" src="images/icons/silver beech tree.png"/>
            <img style="filter: grayscale(1.0);" src="images/icons/gold beech tree.png"/>
            <br/>
            <img src="images/icons/bronze oak tree.png"/>
            <img style="filter: grayscale(1.0);" src="images/icons/silver oak tree.png"/>
            <img style="filter: grayscale(1.0);" src="images/icons/gold oak tree.png"/>
            <br/>
            <img style="filter: grayscale(1.0);" src="images/icons/bronze maple tree.png"/>
            <img style="filter: grayscale(1.0);" src="images/icons/silver maple tree.png"/>
            <img style="filter: grayscale(1.0);" src="images/icons/gold maple tree.png"/>
            </div>
            <div class="columnachiev">
            <img src="images/icons/bronze coin.png"/>
            <img src="images/icons/silver coin.png"/>
            <img src="images/icons/gold coin.png"/>
            <br/>
            <img src="images/icons/bronze tree hugger.png"/>
            <img style="filter: grayscale(1.0);" src="images/icons/silver tree hugger.png"/>
            <img style="filter: grayscale(1.0);" src="images/icons/gold tree hugger.png"/>
            <br/>
            <img style="filter: grayscale(1.0);" src="images/icons/nature lover.png"/>
            </div>
        </div>
    </>
    }
}

#[function_component(Plant)]
pub fn plant() -> Html {
    html! {
    <>
    <Dashboard />
        <h2 style="text-align:center; color: #eeeeee;">{"Plant a tree with your coins."}</h2>
        <div class="rowdaily">
            <div class="columndaily">
            <img src="images/oak.png"/>{"Oak Tree\n(600 coins)"}
            </div>
            <div class="columndaily">
            <img src="images/beech.png"/>{"Beech Tree\n(800 coins)"}
            </div>
            <div class="columndaily">
            <img src="images/maple.png"/>{"Maple Tree\n(900 coins)"}
            </div>
        </div>
    </>
    }
}