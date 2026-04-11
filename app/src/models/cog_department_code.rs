use strum::EnumString;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, EnumString)]
pub enum CogDepartmentCode {
  Bossbot,
  Lawbot,
  Cashbot,
  Sellbot, 
}