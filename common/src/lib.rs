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
    pub days_logged_in: u8,
    pub days_logged_streaks: u8,
    pub trees_planted: u8,
    pub trash_cleaned: u8,
    pub coins_collected: u8,
    pub plastic_hero: u8,
    pub science_wizz: u8,
    pub tree_hugger: u8,
    pub nature_lover: bool,
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