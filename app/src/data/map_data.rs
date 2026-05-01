use crate::models::map_code::MapCode;

#[derive(Clone, Copy)]
pub struct MapData {
  pub name: &'static str,
  pub map_code: MapCode,
  pub companions_enabled: bool,
}

pub const MAP_DATA_STORE: [MapData; 32] = [
  // TTC
  MapData {
    name: "Toontown Central: Playground",
    map_code: MapCode::TtcPlayground,
    companions_enabled: false,
  },
  MapData {
    name: "Toontown Central: Streets",
    map_code: MapCode::TtcStreet,
    companions_enabled: true,
  },

  // DD
  MapData {
    name: "Donald's Dock: Playground",
    map_code: MapCode::DdPlayground,
    companions_enabled: false,
  },
  MapData {
    name: "Donald's Dock: Streets",
    map_code: MapCode::DdStreet,
    companions_enabled: true,
  },

  // DG
  MapData {
    name: "Daisy Gardens: Playground",
    map_code: MapCode::DgPlayground,
    companions_enabled: false,
  },
  MapData {
    name: "Daisy Gardens: Streets",
    map_code: MapCode::DgStreet,
    companions_enabled: true,
  },

  // MML
  MapData {
    name: "Minnie's Melodyland: Playground",
    map_code: MapCode::MmlPlayground,
    companions_enabled: false,
  },
  MapData {
    name: "Minnie's Melodyland: Streets",
    map_code: MapCode::MmlStreet,
    companions_enabled: true,
  },

  // BR
  MapData {
    name: "The Brrrgh: Playground",
    map_code: MapCode::BrPlayground,
    companions_enabled: false,
  },
  MapData {
    name: "The Brrrgh: Streets",
    map_code: MapCode::BrStreet,
    companions_enabled: true,
  },

  // DDL
  MapData {
    name: "Donald's Dreamland: Playground",
    map_code: MapCode::DdlPlayground,
    companions_enabled: false,
  },
  MapData {
    name: "Donald's Dreamland: Streets",
    map_code: MapCode::DdlStreet,
    companions_enabled: true,
  },

  // Cog buildings
  MapData {
    name: "Bossbot, Inc.",
    map_code: MapCode::CogBuildingBb,
    companions_enabled: true,
  },
  MapData {
    name: "Lawbot, Inc.",
    map_code: MapCode::CogBuildingLb,
    companions_enabled: true,
  },
  MapData {
    name: "Cashbot, Inc.",
    map_code: MapCode::CogBuildingCb,
    companions_enabled: true,
  },
  MapData {
    name: "Sellbot, Inc.",
    map_code: MapCode::CogBuildingSb,
    companions_enabled: true,
  },

  // SBHQ
  MapData {
    name: "Sellbot HQ: Courtyard",
    map_code: MapCode::SbhqCourtyard,
    companions_enabled: true,
  },
  MapData {
    name: "Sellbot HQ: Factory",
    map_code: MapCode::SbhqFactory,
    companions_enabled: true,
  },
  MapData {
    name: "Sellbot HQ: Sellbot Towers Lobby",
    map_code: MapCode::SbhqLobby,
    companions_enabled: true,
  },
  MapData {
    name: "Sellbot HQ: Sellbot Towers",
    map_code: MapCode::SbhqVp,
    companions_enabled: true,
  },

  // CBHQ
  MapData {
    name: "Cashbot HQ: Courtyard",
    map_code: MapCode::CbhqCourtyard,
    companions_enabled: true,
  },
  MapData {
    name: "Cashbot HQ: Mint",
    map_code: MapCode::CbhqMint,
    companions_enabled: true,
  },
  MapData {
    name: "Cashbot HQ: Vault Lobby",
    map_code: MapCode::CbhqLobby,
    companions_enabled: true,
  },
  MapData {
    name: "Cashbot HQ: Vault",
    map_code: MapCode::CbhqCfo,
    companions_enabled: true,
  },

  // LBHQ
  MapData {
    name: "Lawbot HQ: Courtyard",
    map_code: MapCode::LbhqCourtyard,
    companions_enabled: true,
  },
  MapData {
    name: "Lawbot HQ: Mint",
    map_code: MapCode::LbhqDaOffice,
    companions_enabled: true,
  },
  MapData {
    name: "Lawbot HQ: Courthouse Lobby",
    map_code: MapCode::LbhqLobby,
    companions_enabled: true,
  },
  MapData {
    name: "Lawbot HQ: Courthouse",
    map_code: MapCode::LbhqCj,
    companions_enabled: true,
  },

   // BBHQ
  MapData {
    name: "Bossbot HQ: Courtyard",
    map_code: MapCode::BbhqCourtyard,
    companions_enabled: true,
  },
  MapData {
    name: "Bossbot HQ: Golf Course",
    map_code: MapCode::BbhqGolfCourse,
    companions_enabled: true,
  },
  MapData {
    name: "Bossbot HQ: Clubhouse Lobby",
    map_code: MapCode::BbhqLobby,
    companions_enabled: true,
  },
  MapData {
    name: "Bossbot HQ: Clubhouse",
    map_code: MapCode::BbhqCeo,
    companions_enabled: true,
  },
];