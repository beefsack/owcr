use std::collections::{HashMap, HashSet};

use serde::Deserialize;

#[derive(Deserialize, PartialEq, Eq, Hash)]
pub enum Hero {
    Ana,
    Ashe,
    Baptiste,
    Bastion,
    Brigitte,
    DVa,
    Doomfist,
    Genji,
    Hanzo,
    Junkrat,
    Lucio,
    McCree,
    Mei,
    Mercy,
    Moira,
    Orisa,
    Pharah,
    Reaper,
    Reinhardt,
    Roadhog,
    Sigma,
    Soldier76,
    Sombra,
    Symmetra,
    Torbjorn,
    Tracer,
    Widowmaker,
    Winston,
    WreckingBall,
    Zarya,
    Zenyatta,
}

#[derive(Deserialize, PartialEq, Eq, Hash)]
pub enum Map {
    HanamuraAttack,
    HanamuraDefense,
    HorizonLunarColonyAttack,
    HorizonLunarColonyDefense,
    ParisAttack,
    ParisDefense,
    TempleOfAnubisAttack,
    TempleOfAnubisDefense,
    VolskayaIndustriesAttack,
    VolskayaIndustriesDefense,
    DoradoAttack,
    DoradoDefense,
    JunkertownAttack,
    JunkertownDefense,
    RialtoAttack,
    RialtoDefense,
    Route66Attack,
    Route66Defense,
    WatchpointGibraltarAttack,
    WatchpointGibraltarDefense,
    BlizzardWorldAttack,
    BlizzardWorldDefense,
    EichenwaldeAttack,
    EichenwaldeDefense,
    HollywoodAttack,
    HollywoodDefense,
    KingsRowAttack,
    KingsRowDefense,
    BusanSanctuary,
    BusanDowntown,
    BusanMEKABase,
    IliosLighthouse,
    IliosWell,
    IliosRuins,
    LijiangTowerNightMarket,
    LijiangTowerGarden,
    LigiangTowerControlCenter,
    NepalVillage,
    NepalShrine,
    NepalSanctum,
    OasisCityCenter,
    OasisGardens,
    OasisUniversity,
}

#[derive(Deserialize)]
pub struct Rule {
    pub comment: String,
    pub allies: HashSet<Hero>,
    pub enemies: Option<HashSet<Hero>>,
    pub maps: Option<HashSet<Map>>,
    pub score: isize,
}

pub fn from_slice<'de>(bytes: &'de [u8]) -> Result<HashMap<String, Rule>, toml::de::Error> {
    toml::from_slice(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn data_works() {
        let raw = include_bytes!("../../data/rules.toml");
        let _rules = from_slice(raw).expect("expected to read TOML file");
    }
}
