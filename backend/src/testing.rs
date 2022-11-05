use passwords::*;
use common::*;

pub fn create_example_user_1() -> User {
    User {
        id: 0,
        name: String::from("Arkid"),
        email: String::from("kaleciarkid@gmail.com"),
        password: generate_hash(String::from("1234567890")),
        inventory: Inventory {
            coins: 126,
            oak: 0,
            beech: 0,
            maple: 0,
        },
        achievements: Achievements {
            oak_planted: false,
            beech_planted: false,
            maple_planted: false,
            days_logged_in: 8,
            days_logged_streaks: 3,
            trees_planted: 0,
            trash_cleaned: 38,
            coins_collected: 126,
            plastic_hero: 0,
            science_wizz: 0,
            tree_hugger: 0,
            nature_lover: false,
        },
        daily_quests: DailyQuest {
            general: true,
            specific: false,
            wholesome: false,
        },
        stats: Stats {
            trees_planted: 0,
            trash_cleaned: 38,
            coins_collected: 126,
            days_logged: 8,
            days_logged_streak: 3,
            
        },
    }
}