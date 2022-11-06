mod load;
mod passwords;
mod testing;

use load::*;
use passwords::*;
use testing::*;
use common::*;
use std::{collections::HashMap, sync::{atomic::AtomicU32, Arc}};
use warp::Filter;

#[tokio::main]
async fn main() {
    let mut day = Arc::new(AtomicU32::new(1));
    //read file quotes.txt and load the lines as strings into an array
    let file = std::fs::File::open("quotes.txt").unwrap();
    let mut contents = Vec::new();
    std::io::Read::read_to_end(&mut std::io::BufReader::new(file), &mut contents).unwrap();
    let quotes = std::str::from_utf8(&contents).unwrap().lines().map(|s| s.to_string()).collect::<Vec<String>>();
    
    //create a get route that gives the current day
    let get_date = warp::path!("date").map(|| day.load(std::sync::atomic::Ordering::Relaxed));

    let get_quote = warp::path!("quote").map(|| {
        let day = day.load(std::sync::atomic::Ordering::Relaxed);
        let quote = quotes[day as usize - 1].clone();
        quote
    });

    
    //create a post route that increments the day and calls a function
    let post_date = warp::path!("add")
        .and(warp::post())
        .map(move || {
            reqwe
            if (day.load(std::sync::atomic::Ordering::Relaxed) < 19) {
                day.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
            } else {
                day.store(1, std::sync::atomic::Ordering::Relaxed);
                day.load(std::sync::atomic::Ordering::Relaxed)
            }
        });
}