use std::{collections::HashMap, io::BufReader};
use common::*;
use serde_json::from_reader;

pub fn load () -> (Vec<User>, HashMap<User, Inventory>, HashMap<User, Achievements>, HashMap<User, DailyQuest>, HashMap<User, Stats>) {
    if std::path::Path::new("users.json").exists() {
        let file = std::fs::File::open("users.json").unwrap();
        let reader = BufReader::new(file);
        let users = from_reader(reader).unwrap();
        let file = std::fs::File::open("inventories.json").unwrap();
        let reader = BufReader::new(file);
        let inventories = from_reader(reader).unwrap();
        let file = std::fs::File::open("achievements.json").unwrap();
        let reader = BufReader::new(file);
        let achievements = from_reader(reader).unwrap();
        let file = std::fs::File::open("daily_quests.json").unwrap();
        let reader = BufReader::new(file);
        let daily_quests = from_reader(reader).unwrap();
        let file = std::fs::File::open("stats.json").unwrap();
        let reader = BufReader::new(file);
        let stats = from_reader(reader).unwrap();
        (users, inventories, achievements, daily_quests, stats)
    } else {
        let file = std::fs::File::create("users.json").unwrap();
        let writer = std::io::BufWriter::new(file);
        let users = Vec::new();
        let file = std::fs::File::create("inventories.json").unwrap();
        let writer = std::io::BufWriter::new(file);
        let inventories = HashMap::new();
        let file = std::fs::File::create("achievements.json").unwrap();
        let writer = std::io::BufWriter::new(file);
        let achievements = HashMap::new();
        let file = std::fs::File::create("daily_quests.json").unwrap();
        let writer = std::io::BufWriter::new(file);
        let daily_quests = HashMap::new();
        let file = std::fs::File::create("stats.json").unwrap();
        let writer = std::io::BufWriter::new(file);
        let stats = HashMap::new();
        (users, inventories, achievements, daily_quests, stats)
    }
}