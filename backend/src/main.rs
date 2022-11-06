mod load;
mod passwords;
mod testing;

use load::*;
use passwords::*;
use testing::*;
use common::*;
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    //let (mut users, mut inventories, mut achievements, mut daily_quests, mut stats) = load();
    let mut users: Vec<User> = Vec::new();
    users.push(create_example_user_1());

    //create a route that receives a name, email, and plain password and adds it to the users vector
    warp::serve(warp::post()
        .and(warp::path("register"))
        .and(warp::body::json())
        .and_then(register));
    
    warp::serve(routes).run(([127, 0, 0, 1], 8000)).await;
}