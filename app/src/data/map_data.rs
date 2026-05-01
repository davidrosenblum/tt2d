use crate::models::map_code::MapCode;
use crate::models::map_hub_code::MapHubCode;

#[derive(Clone, Copy)]
pub struct MapData {
  pub name: &'static str,
  pub map_code: MapCode,
  pub map_hub_code: Option<MapHubCode>,
  pub companions_enabled: bool,
}

pub const MAP_DATA_STORE: [MapData; 32] = [
  // TTC
  MapData {
    name: "Toontown Central: Playground",
    map_code: MapCode::TtcPlayground,
    map_hub_code: Some(MapHubCode::Ttc),
    companions_enabled: false,
  },
  MapData {
    name: "Toontown Central: Streets",
    map_code: MapCode::TtcStreet,
    map_hub_code: Some(MapHubCode::Ttc),
    companions_enabled: true,
  },

  // DD
  MapData {
    name: "Donald's Dock: Playground",
    map_code: MapCode::DdPlayground,
    map_hub_code: Some(MapHubCode::Dd),
    companions_enabled: false,
  },
  MapData {
    name: "Donald's Dock: Streets",
    map_code: MapCode::DdStreet,
    map_hub_code: Some(MapHubCode::Dd),
    companions_enabled: true,
  },

  // DG
  MapData {
    name: "Daisy Gardens: Playground",
    map_code: MapCode::DgPlayground,
    map_hub_code: Some(MapHubCode::Dg),
    companions_enabled: false,
  },
  MapData {
    name: "Daisy Gardens: Streets",
    map_code: MapCode::DgStreet,
    map_hub_code: Some(MapHubCode::Dg),
    companions_enabled: true,
  },

  // MML
  MapData {
    name: "Minnie's Melodyland: Playground",
    map_code: MapCode::MmlPlayground,
    map_hub_code: Some(MapHubCode::Mml),
    companions_enabled: false,
  },
  MapData {
    name: "Minnie's Melodyland: Streets",
    map_code: MapCode::MmlStreet,
    map_hub_code: Some(MapHubCode::Mml),
    companions_enabled: true,
  },

  // BR
  MapData {
    name: "The Brrrgh: Playground",
    map_code: MapCode::BrPlayground,
    map_hub_code: Some(MapHubCode::Br),
    companions_enabled: false,
  },
  MapData {
    name: "The Brrrgh: Streets",
    map_code: MapCode::BrStreet,
    map_hub_code: Some(MapHubCode::Br),
    companions_enabled: true,
  },

  // DDL
  MapData {
    name: "Donald's Dreamland: Playground",
    map_code: MapCode::DdlPlayground,
    map_hub_code: Some(MapHubCode::Ddl),
    companions_enabled: false,
  },
  MapData {
    name: "Donald's Dreamland: Streets",
    map_code: MapCode::DdlStreet,
    map_hub_code: Some(MapHubCode::Ddl),
    companions_enabled: true,
  },

  // Cog buildings
  MapData {
    name: "Bossbot, Inc.",
    map_code: MapCode::CogBuildingBb,
    map_hub_code: None,
    companions_enabled: true,
  },
  MapData {
    name: "Lawbot, Inc.",
    map_code: MapCode::CogBuildingLb,
    map_hub_code: None,
    companions_enabled: true,
  },
  MapData {
    name: "Cashbot, Inc.",
    map_code: MapCode::CogBuildingCb,
    map_hub_code: None,
    companions_enabled: true,
  },
  MapData {
    name: "Sellbot, Inc.",
    map_code: MapCode::CogBuildingSb,
    map_hub_code: None,
    companions_enabled: true,
  },

  // SBHQ
  MapData {
    name: "Sellbot HQ: Courtyard",
    map_code: MapCode::SbhqCourtyard,
    map_hub_code: Some(MapHubCode::Sbhq),
    companions_enabled: true,
  },
  MapData {
    name: "Sellbot HQ: Factory",
    map_code: MapCode::SbhqFactory,
    map_hub_code: Some(MapHubCode::Sbhq),
    companions_enabled: true,
  },
  MapData {
    name: "Sellbot HQ: Sellbot Towers Lobby",
    map_code: MapCode::SbhqLobby,
    map_hub_code: Some(MapHubCode::Sbhq),
    companions_enabled: true,
  },
  MapData {
    name: "Sellbot HQ: Sellbot Towers",
    map_code: MapCode::SbhqVp,
    map_hub_code: Some(MapHubCode::Sbhq),
    companions_enabled: true,
  },

  // CBHQ
  MapData {
    name: "Cashbot HQ: Courtyard",
    map_code: MapCode::CbhqCourtyard,
    map_hub_code: Some(MapHubCode::Cbhq),
    companions_enabled: true,
  },
  MapData {
    name: "Cashbot HQ: Mint",
    map_code: MapCode::CbhqMint,
    map_hub_code: Some(MapHubCode::Cbhq),
    companions_enabled: true,
  },
  MapData {
    name: "Cashbot HQ: Vault Lobby",
    map_code: MapCode::CbhqLobby,
    map_hub_code: Some(MapHubCode::Cbhq),
    companions_enabled: true,
  },
  MapData {
    name: "Cashbot HQ: Vault",
    map_code: MapCode::CbhqCfo,
    map_hub_code: Some(MapHubCode::Cbhq),
    companions_enabled: true,
  },

  // LBHQ
  MapData {
    name: "Lawbot HQ: Courtyard",
    map_code: MapCode::LbhqCourtyard,
    map_hub_code: Some(MapHubCode::Lbhq),
    companions_enabled: true,
  },
  MapData {
    name: "Lawbot HQ: Mint",
    map_code: MapCode::LbhqDaOffice,
    map_hub_code: Some(MapHubCode::Lbhq),
    companions_enabled: true,
  },
  MapData {
    name: "Lawbot HQ: Courthouse Lobby",
    map_code: MapCode::LbhqLobby,
    map_hub_code: Some(MapHubCode::Lbhq),
    companions_enabled: true,
  },
  MapData {
    name: "Lawbot HQ: Courthouse",
    map_code: MapCode::LbhqCj,
    map_hub_code: Some(MapHubCode::Lbhq),
    companions_enabled: true,
  },

   // BBHQ
  MapData {
    name: "Bossbot HQ: Courtyard",
    map_code: MapCode::BbhqCourtyard,
    map_hub_code: Some(MapHubCode::Bbhq),
    companions_enabled: true,
  },
  MapData {
    name: "Bossbot HQ: Golf Course",
    map_code: MapCode::BbhqGolfCourse,
    map_hub_code: Some(MapHubCode::Bbhq),
    companions_enabled: true,
  },
  MapData {
    name: "Bossbot HQ: Clubhouse Lobby",
    map_code: MapCode::BbhqLobby,
    map_hub_code: Some(MapHubCode::Bbhq),
    companions_enabled: true,
  },
  MapData {
    name: "Bossbot HQ: Clubhouse",
    map_code: MapCode::BbhqCeo,
    map_hub_code: Some(MapHubCode::Bbhq),
    companions_enabled: true,
  },
];