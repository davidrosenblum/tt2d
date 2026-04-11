use strum::EnumString;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, EnumString)]
pub enum ToonNpcCode {
  // Ttc
  Flippy,
  ProfessorPete,
  StickyLou,
}