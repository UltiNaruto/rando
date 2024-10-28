use std::collections::{BTreeMap, HashMap};
use serde::{Deserialize, Serialize};

use crate::logic::{Ability as A, Drop as D, Check, Location, SPAWNS, CHECKS};
use crate::logic::Lock::Any;

const MAJOR_KEYS: [&'static str; 5] = [
    "Major Key - Empty Bailey",
    "Major Key - The Underbelly",
    "Major Key - Tower Remains",
    "Major Key - Sansa Keep",
    "Major Key - Twilight Theatre",
];

const PICKUP_LOCATIONS: [&'static str; 92] = [
    "",
    "Dilapidated Dungeon - Time Trial",
    "Dilapidated Dungeon - Dream Breaker",
    "Dilapidated Dungeon - Alcove Near Mirror",
    "Dilapidated Dungeon - Slide",
    "Dilapidated Dungeon - Dark Orbs",
    "Dilapidated Dungeon - Rafters",
    "Dilapidated Dungeon - Past Poles",
    "Dilapidated Dungeon - Strong Eyes",
    "",
    "Castle Sansa - Memento",
    "",
    "",
    "",
    "",
    "Castle Sansa - Indignation",
    "Castle Sansa - Platform In Main Halls",
    "",
    "Castle Sansa - Time Trial",
    "Castle Sansa - Corner Corridor",
    "Castle Sansa - Wheel Crawlers",
    "Castle Sansa - Alcove Near Scythe Corridor",
    "",
    "Castle Sansa - Near Theatre Front",
    "Castle Sansa - Tall Room Near Wheel Crawlers",
    "Castle Sansa - Alcove Near Dungeon",
    "",
    "Castle Sansa - Balcony",
    "",
    "",
    "Castle Sansa - Floater In Courtyard",
    "Castle Sansa - High Climb From Courtyard",
    "",
    "Listless Library - Time Trial",
    "Listless Library - Sun Greaves",
    "",
    "",
    "",
    "Listless Library - Upper Back",
    "Listless Library - Locked Door Across",
    "Listless Library - Locked Door Left",
    "",
    "",
    "",
    "Sansa Keep - Strikebreak",
    "Sansa Keep - Sunsetter",
    "Sansa Keep - Alcove Near Locked Door",
    "Sansa Keep - Levers Room",
    "Sansa Keep - Near Theatre",
    "Sansa Keep - Lonely Throne",
    "Sansa Keep - Time Trial",
    "",
    "",
    "Empty Bailey - Inside Building",
    "Empty Bailey - Guarded Hand",
    "Empty Bailey - Solar Wind",
    "Empty Bailey - Center Steeple",
    "Empty Bailey - Cheese Bell",
    "Empty Bailey - Time Trial",
    "The Underbelly - Rafters Near Keep",
    "The Underbelly - Surrounded By Holes",
    "The Underbelly - Ascendant Light",
    "The Underbelly - Alcove Near Light",
    "The Underbelly - Main Room",
    "",
    "The Underbelly - Strikebreak Wall",
    "The Underbelly - Time Trial",
    "The Underbelly - Locked Door",
    "",
    "",
    "The Underbelly - Building Near Little Guy",
    "Tower Remains - Cling Gem",
    "Tower Remains - Atop The Tower",
    "Tower Remains - Time Trial",
    "Twilight Theatre - Corner Beam",
    "Twilight Theatre - Time Trial",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "",
    "Twilight Theatre - Murderous Goat",
    "",
    "",
    "",
    "Twilight Theatre - Center Stage",
    "Twilight Theatre - Soul Cutter",
    "Twilight Theatre - Back Of Auditorium",
    "Twilight Theatre - Locked Door",
];

fn internal_name_from_level_name(level_name: &str) -> &'static str
{
    match level_name {
        "Dilapidated Dungeon" => "ZONE_Dungeon",
        "Castle Sansa" => "ZONE_LowerCastle",
        "Sansa Keep" => "Zone_Upper",
        "Listless Library" => "Zone_Library",
        "Twilight Theatre" => "Zone_Theatre",
        "Empty Bailey" => "ZONE_Exterior",
        "The Underbelly" => "Zone_Caves",
        "Tower Remains" => "Zone_Tower",
        _ => panic!("{} is not a valid level name", level_name),
    }
}

#[derive(Debug, Serialize)]
pub struct PatchConfig {
    pub starting_room: (&'static str, Location),

    pub split_kick: bool,
    pub split_cling: bool,

    pub progressive_breaker: bool,
    pub progressive_slide: bool,

    pub major_key_hints: [String; 5],

    pub checks: BTreeMap<&'static str, Vec<Check>>,
}

#[derive(Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct GameConfig {
    pub starting_room: Option<String>,

    pub split_kick: Option<bool>,
    pub split_cling: Option<bool>,

    pub progressive_breaker: Option<bool>,
    pub progressive_slide: Option<bool>,

    pub major_key_hints: HashMap<String, String>,
}

#[derive(Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct PickupConfig {
    pub index: usize,

    #[serde(alias = "type")]
    pub pickup_type: String,

    // used with offworld pickup to tell if the pickup is major
    pub major: Option<bool>,

    // used with offworld pickup to set the description when picked up
    pub description: Option<String>,
}

#[derive(Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct LevelData {
    #[serde(default)]
    pickups: HashMap<String, PickupConfig>,
}

#[derive(Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
struct PatchConfigPrivate {
    #[serde(default)]
    game_config: GameConfig,

    #[serde(default)]
    level_data: HashMap<String, LevelData>,
}

impl PatchConfigPrivate {
    pub fn from_json(json: &str) -> Result<Self, String>
    {
        serde_json::from_str(json).map_err(|e| format!("Error while parsing config: {}", e.to_string()))
    }
}

impl PatchConfig {
    pub fn from_json<'a>(json: &str) -> Result<Self, String>
    {
        let cfg = Box::leak(Box::new(PatchConfigPrivate::from_json(json)?));

        // filling starting room according to config
        let starting_room_name = cfg.game_config.starting_room.clone().unwrap_or("gameStart".to_string());
        let starting_room_idx = SPAWNS.iter().position(|(sp_name, _)| *sp_name == starting_room_name).unwrap();
        let starting_room = SPAWNS[starting_room_idx];

        // filling major key hints array
        let mut major_key_hints = [const { String::new() }; 5];
        for (n, h) in cfg.game_config.major_key_hints.iter() {
            let major_key_idx = MAJOR_KEYS.iter().position(|k| *k == *n);
            if major_key_idx.is_some() {
                major_key_hints[major_key_idx.unwrap()] = h.clone();
            }
        }

        let mut checks: BTreeMap<&'static str, Vec<Check>> = BTreeMap::new();
        for (level_name, level_data) in cfg.level_data.iter() {
            let mut level_checks: Vec<Check> = Vec::new();
            for (location_name, pickup_cfg) in level_data.pickups.iter() {
                let loc_name = format!("{} - {}", level_name, location_name);
                let check_idx = PICKUP_LOCATIONS.iter().position(|c| *c == loc_name);
                let mut check = if check_idx.is_none() {
                    match loc_name.as_str() {
                        "Listless Library - Sun Greaves 1" => Check {
                            description: "where sun greaves normally is",
                            location: Location::MainLibrary,
                            index: 1679,
                            drop: D::Ability(A::HeliacalPower),
                            trial: None,
                            locks: Any(&[]),
                        },
                        "Listless Library - Sun Greaves 2" => Check {
                            description: "where sun greaves normally is",
                            location: Location::MainLibrary,
                            index: 1685,
                            drop: D::Ability(A::HeliacalPower),
                            trial: None,
                            locks: Any(&[]),
                        },
                        "Listless Library - Sun Greaves 3" => Check {
                            description: "where sun greaves normally is",
                            location: Location::MainLibrary,
                            index: 1691,
                            drop: D::Ability(A::HeliacalPower),
                            trial: None,
                            locks: Any(&[]),
                        },
                        _ => continue,
                    }
                } else {
                    CHECKS[check_idx.unwrap()].clone()
                };

                // retrieve major key number from its name
                let major_key_idx = MAJOR_KEYS.iter().position(|k| *k == pickup_cfg.pickup_type);

                // set drop to the one in the config
                check.drop = match pickup_cfg.pickup_type.as_ref() {
                    "Health Piece" => D::Health,
                    "Small Key" => D::SmallKey,
                    "Offworld Item" => D::Ability(A::OffworldItem(pickup_cfg.major.unwrap_or(false), pickup_cfg.index, pickup_cfg.description.as_ref())),
                    _ => if pickup_cfg.pickup_type.starts_with("Major Key") && major_key_idx.is_some() {
                        D::BigKey((major_key_idx.unwrap() + 1) as i32)
                    } else {
                        D::Ability(A::from_name(pickup_cfg.pickup_type.as_ref(), match pickup_cfg.pickup_type.as_ref() {
                            "Cling Gem" => Some(6),
                            _ => None,
                        }))
                    },
                };
                level_checks.push(check);
            }
            checks.insert(internal_name_from_level_name(level_name), level_checks);
        }

        Ok(Self {
            starting_room,
            split_kick: cfg.game_config.split_kick.unwrap_or(false),
            split_cling: cfg.game_config.split_cling.unwrap_or(false),
            progressive_breaker: cfg.game_config.progressive_breaker.unwrap_or(false),
            progressive_slide: cfg.game_config.progressive_slide.unwrap_or(false),
            major_key_hints,
            checks,
        })
    }
}