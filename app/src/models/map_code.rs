use serde::{Deserialize, Serialize};
use strum::EnumString;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize, EnumString)]
pub enum MapCode {
  // Toontown Central
  #[strum(serialize = "TTC_Playground")]
  TtcPlayground,
  #[strum(serialize = "TTC_Street")]
  TtcStreet,

  // Donald's Dock
  #[strum(serialize = "DD_Playground")]
  DdPlayground,
  #[strum(serialize = "DD_Street")]
  DdStreet,

  // Daisy's Gardens
  #[strum(serialize = "DG_Playground")]
  DgPlayground,
  #[strum(serialize = "DG_Street")]
  DgStreet,

  // Minnie's Melodyland
  #[strum(serialize = "MML_Playground")]
  MmlPlayground,
  #[strum(serialize = "MML_Street")]
  MmlStreet,

  // The Brrrgh
  #[strum(serialize = "BR_Playground")]
  BrPlayground,
  #[strum(serialize = "BR_Street")]
  BrStreet,

  // Donald's Dreamland
  #[strum(serialize = "DDL_Playground")]
  DdlPlayground,
  #[strum(serialize = "DDL_Street")]
  DdlStreet,

  // Cog buildings
  #[strum(serialize = "CogBuilding_BB")]
  CogBuildingBb,
  #[strum(serialize = "CogBuilding_LB")]
  CogBuildingLb,
  #[strum(serialize = "CogBuilding_CB")]
  CogBuildingCb,
  #[strum(serialize = "CogBuilding_SB")]
  CogBuildingSb,

  // Sellbot HQ
  #[strum(serialize = "SBHQ_Courtyard")]
  SbhqCourtyard,
  #[strum(serialize = "SBHQ_Factory")]
  SbhqFactory,
  #[strum(serialize = "SBHQ_Lobby")]
  SbhqLobby,
  #[strum(serialize = "SBHQ_VP")]
  SbhqVp,

  // Cashbot HQ
  #[strum(serialize = "CBHQ_Courtyard")]
  CbhqCourtyard,
  #[strum(serialize = "CBHQ_Mint")]
  CbhqMint,
  #[strum(serialize = "CBHQ_Lobby")]
  CbhqLobby,
   #[strum(serialize = "CBHQ_CFO")]
  CbhqCfo,

  // Lawbot HQ
  #[strum(serialize = "LBHQ_Courtyard")]
  LbhqCourtyard,
  #[strum(serialize = "BBHQ_Da_Office")]
  LbhqDaOffice,
  #[strum(serialize = "LBHQ_Lobby")]
  LbhqLobby,
  #[strum(serialize = "LBJQ_CJ")]
  LbhqCj,

  // Bossbot HQ
  #[strum(serialize = "BBHQ_Courtyard")]
  BbhqCourtyard,
  #[strum(serialize = "BBHQ_GolfCourse")]
  BbhqGolfCourse,
  #[strum(serialize = "BBHQ_Lobby")]
  BbhqLobby,
  #[strum(serialize = "BBHQ_CEO")]
  BbhqCeo,
}