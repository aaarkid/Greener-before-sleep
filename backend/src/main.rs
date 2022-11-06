mod load;
mod passwords;
mod testing;

use load::*;
use passwords::*;
use testing::*;
use common::*;
use std::{collections::HashMap, sync::{atomic::AtomicU32, Arc}, io::Write};
use warp::Filter;
use reqwest::Client;

#[tokio::main]
async fn main() {
    let mut day: Arc<AtomicU32> = Arc::new(AtomicU32::new(1));
    //read file quotes.txt and load the lines as strings into an array
    let file = std::fs::File::open("quotes.txt").unwrap();
    let mut contents = Vec::new();
    std::io::Read::read_to_end(&mut std::io::BufReader::new(file), &mut contents).unwrap();
    let quotes = std::str::from_utf8(&contents).unwrap().lines().map(|s| s.to_string()).collect::<Vec<String>>();
    
    //create a get route that gives the current day
    let get_date = warp::path!("date").map({
        let shared_day = Arc::clone(&day); 
        move || 
        warp::reply::json(&shared_day.load(std::sync::atomic::Ordering::Relaxed))
    });

    let get_quote = warp::path!("quote").map({ 
        let shared_day = Arc::clone(&day);
        move || {
            let day = shared_day.load(std::sync::atomic::Ordering::Relaxed);
            let quote = quotes[day as usize - 1].as_str();
            warp::reply::json(&quote)
        }
    });

    
    //create a post route that increments the day and calls a function
    let post_date = warp::path!("add")
        .and(warp::post())
        .map( move || {
            //make a post request to the link https://api.twilio.com/2010-04-01/Accounts/AC2aca0b0375acdff08df264a951dc993e/Messages.json with data
            let client = Client::new();
            let _ = client.post("https://api.twilio.com/2010-04-01/Accounts/AC2aca0b0375acdff08df264a951dc993e/Messages.json")
                .basic_auth("AC2aca0b0375acdff08df264a951dc993e", Some("8f169004f8dc6c39df637578c575f117"))
                .form(&[("To", "+355699751977"), ("MessagingServiceSid", "MGe662c305172f061ad61d99e309a9cb64"), ("Body", "Don't forget to log in GBS today and feel good about your effort to make our planet greener! www.sustaining.tech/greener")])
                .send();

            let _ = std::fs::copy(format!("../frontend/images/bg/img{}.png", &day.load(std::sync::atomic::Ordering::Relaxed) + 1), "../frontend/images/bg/img.png");
            // //create a quote.txt file and write the quote of the day to it
            // let mut file = std::fs::File::create("../frontend/quote.txt").unwrap();
            // let _ = file.write_all(quotes[day.load(std::sync::atomic::Ordering::Relaxed) as usize - 1].as_bytes());
            if (day.load(std::sync::atomic::Ordering::Relaxed) < 19) {
                warp::reply::json(&day.fetch_add(1, std::sync::atomic::Ordering::Relaxed))
            } else {
                day.store(1, std::sync::atomic::Ordering::Relaxed);
                warp::reply::json(&day.load(std::sync::atomic::Ordering::Relaxed))
            }
        });

    //serve the routes
    warp::serve(get_date.or(post_date).or(get_quote)).run(([0, 0, 0, 0], 3030)).await;
}