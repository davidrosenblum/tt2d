use crate::models::map_code::MapCode;

#[derive(Clone, Copy)]
pub struct MapData {
  pub name: &'static str,
  pub map_code: MapCode,
  pub companions_enabled: bool,
}

pub const MAP_DATA_STORE: [MapData; 2] = [
  MapData {
    name: "Toontown Central: Playground",
    map_code: MapCode::TtcPlayground,
    companions_enabled: false,
  },
  MapData {
    name: "Toontown Central: Streets",
    map_code: MapCode::TtcStreets,
    companions_enabled: true,
  },
];