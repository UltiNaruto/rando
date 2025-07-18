use serde::Serialize;
/*
To-Do:
split areas further to ensure proper logic is available.
Add Nodes for each lever.
Re-name current nodes.
i.e. split Late prison into the save crystal's, In the rafters room, and after the underbelly wall.
*/
#[derive(Debug, Clone, Copy, PartialEq, strum::EnumIter, strum::EnumCount, strum::Display, Serialize)]
pub enum Location {
    // Prison
    EarlyPrison,
    LatePrison,
    StrongEyes,
    PEntryCastle,
    PEntryUnderBelly,
    PEntryTheatre,
    // Castle
    CsOldSoftlockRoom,
    CsKeepClimbEntrance,
    CsMain,
    CsTheatreEntrance,
    CsPrisonEntry,
    CsLibraryEntry,
    CsTheatreEntryNearPrison,
    CsKeepEntryMain,
    CsKeepEntryRamp,
    CsBaileyEntry,
    // Library
    MainLibrary,
    Restricted,
    LibSaveNearGreaves,
    // Keep
    SkCastleRampEntry,
    SkCastleMainEntry,
    SkCastleClimbEntry,
    SkUnderbellyEntry,
    SkTheatreEntry,
    SansaKeep,
    Sunsetter,
    // Bailey
    EmptyBailey,
    EbEntryUnderBelly,
    EbEntryRuins,
    EbEntryTheatre,
    EbEntryCastle,
    // Ruins
    TowerRuinsEntrance,
    TowerRuinsKeep,
    // Underbelly
    SansaHole,
    BaileyHole,
    PrisonHole,
    MainUnderbelly,
    VAscendantLight,
    HpSave,
    // Theatre
    ThCastleEntryPillar,
    ThCastleEntryMain,
    ThBaileyEntry,
    ThKeepEntry,
    ThDungeonEntry,
    PillarRoom,
    TheatreEntrance,
    OtherTheatrePath,
    MainTheatre,
    // Final
    FinalBoss,
}

use Location as L;
impl Location {
    pub const fn file(&self) -> &'static str {
        match self {
            L::PEntryTheatre
            | L::PEntryCastle
            | L::PEntryUnderBelly
            | L::LatePrison
            | L::EarlyPrison
            | L::StrongEyes => "ZONE_Dungeon",
            L::CsBaileyEntry
            | L::CsPrisonEntry
            | L::CsLibraryEntry
            | L::CsTheatreEntryNearPrison
            | L::CsKeepEntryMain
            | L::CsKeepEntryRamp
            | L::CsOldSoftlockRoom
            | L::CsKeepClimbEntrance
            | L::CsMain
            | L::CsTheatreEntrance => "ZONE_LowerCastle",
            L::LibSaveNearGreaves | L::MainLibrary | L::Restricted => "Zone_Library",
            L::SkTheatreEntry
            | L::SkUnderbellyEntry
            | L::SkCastleClimbEntry
            | L::SkCastleMainEntry
            | L::SkCastleRampEntry
            | L::SansaKeep
            | L::Sunsetter => "Zone_Upper",
            L::EbEntryCastle
            | L::EbEntryRuins
            | L::EbEntryTheatre
            | L::EbEntryUnderBelly
            | L::EmptyBailey => "ZONE_Exterior",
            L::TowerRuinsEntrance | L::TowerRuinsKeep => "Zone_Tower",
            L::VAscendantLight
            | L::HpSave
            | L::SansaHole
            | L::PrisonHole
            | L::BaileyHole
            | L::MainUnderbelly => "Zone_Caves",
            L::ThDungeonEntry
            | L::ThBaileyEntry
            | L::ThCastleEntryMain
            | L::ThCastleEntryPillar
            | L::ThKeepEntry
            | L::PillarRoom
            | L::TheatreEntrance
            | L::OtherTheatrePath
            | L::MainTheatre => "Zone_Theatre",
            L::FinalBoss => "Zone_PrincessChambers",
        }
    }
    pub const fn name(&self) -> &'static str {
        match self {
            L::PEntryTheatre
            | L::PEntryCastle
            | L::PEntryUnderBelly
            | L::LatePrison
            | L::EarlyPrison
            | L::StrongEyes => "Dilapidated Dungeon",
            L::CsBaileyEntry
            | L::CsPrisonEntry
            | L::CsLibraryEntry
            | L::CsTheatreEntryNearPrison
            | L::CsKeepEntryMain
            | L::CsKeepEntryRamp
            | L::CsOldSoftlockRoom
            | L::CsKeepClimbEntrance
            | L::CsMain
            | L::CsTheatreEntrance => "Castle Sansa",
            L::LibSaveNearGreaves | L::MainLibrary | L::Restricted => "Listless Library",
            L::SkTheatreEntry
            | L::SkUnderbellyEntry
            | L::SkCastleClimbEntry
            | L::SkCastleMainEntry
            | L::SkCastleRampEntry
            | L::SansaKeep
            | L::Sunsetter => "Sansa Keep",
            L::EbEntryCastle
            | L::EbEntryRuins
            | L::EbEntryTheatre
            | L::EbEntryUnderBelly
            | L::EmptyBailey => "Empty Bailey",
            L::TowerRuinsEntrance | L::TowerRuinsKeep => "Tower Ruins",
            L::VAscendantLight
            | L::HpSave
            | L::SansaHole
            | L::PrisonHole
            | L::BaileyHole
            | L::MainUnderbelly => "Underbelly",
            L::ThDungeonEntry
            | L::ThBaileyEntry
            | L::ThCastleEntryMain
            | L::ThCastleEntryPillar
            | L::ThKeepEntry
            | L::PillarRoom
            | L::TheatreEntrance
            | L::OtherTheatrePath
            | L::MainTheatre => "Twilight Theatre",
            L::FinalBoss => "Princess",
        }
    }
}
