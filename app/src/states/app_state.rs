use bevy::state::state::States;

#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, States)]
pub enum AppState {
  #[default]
  LoadingAssets,
  ProcessingAssets,
  LoadingProfile,
  MainMenu,
  LoadingMap,
  InGame,
}