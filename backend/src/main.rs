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

}