pub struct User {
    pub id: u32,
    pub name: String,
    pub email: String,
    pub password: String,
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

}

pub struct DailyQuest {
    pub general: bool,
    pub specific: bool,
    pub wholesome: bool,
}

pub enum Trash {
    PlasticPackage,
    PlasticBottle,
    PlasticBag,
    PlasticCup,
    CigaretteButt,
    PaperBag,
    GlassBottle,
    
}
