pub struct User {
    pub id: u32,
    pub name: String,
    pub email: String,
    pub password: String,
    pub inventory: Inventory,
    pub achievements: Achievements,
    pub daily: DailyQuest,
    pub stats: Stats,
}

pub struct Inventory {
    pub coins: u32,
    pub trees: Trees,
}

pub struct Trees {
    pub oak: u32,
    pub beech: u32,
    pub maple: u32,
}

pub struct Achievements {
    pub oak_planted: bool,
    pub beech_planted: bool,
    pub maple_planted: bool,
    pub days_logged_in: u8, // bronze medal: 7, silver: 30, gold: 90
    pub days_logged_streaks: u8, // bronze: 3, silver: 7, gold: 21
    pub trash_cleaned: u8, // bronze: 10, silver: 50, gold: 250
    pub coins_collected: u8, // bronze: 50, silver: 200, gold: 1000
    pub plastic_hero: u8, // bronze: 5, silver: 25, gold: 125
     // plastic_hero: achieved after specific number of plastic trash collected
    pub science_wizz: u8, // bronze: 1, silver: 3, gold: 10
    // science_wizz: achieved after specific number of chemical waste collected
    pub tree_hugger: u8, // bronze: 1, silver: 3, gold: 10
    // tree_hugger: achieved after specific number of trees planted
    pub nature_lover: bool, 
    // nature_lover: achieved when you are a beginner
}

pub struct DailyQuest {
    pub general: bool,
    pub specific: bool,
    pub wholesome: bool,
}

pub struct Stats {
    pub days_logged: u32,
    pub days_logged_streak: u32,
    pub trees_planted: u32,
    pub trash_cleaned: u32,
    pub coins_collected: u32,
}

pub enum Trash {
    PlasticPackage,
    PlasticBottle,
    PlasticBag,
    PlasticCup,
    CigaretteButt,
    PaperBag,
    GlassBottle,
    LeftoverFood,
    ChemicalWaste,
    BottleCap,
    PlasticStraw,
    Metals,
}

impl Into<u32> for Trash {
    fn into(self) -> u32 {
        match self {
            Trash::PlasticPackage => 2,
            Trash::PlasticBottle => 4,
            Trash::PlasticBag => 3,
            Trash::PlasticCup => 3,
            Trash::CigaretteButt => 2,
            Trash::PaperBag => 2,
            Trash::GlassBottle => 4,
            Trash::LeftoverFood => 3,
            Trash::ChemicalWaste => 8,
            Trash::BottleCap => 2,
            Trash::PlasticStraw => 2,
            Trash::Metals => 10,
        }
    }
}