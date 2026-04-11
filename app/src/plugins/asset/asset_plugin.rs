use bevy::app::{Plugin, Update};
use bevy::asset::{AssetApp, AssetServer, Assets};
use bevy::ecs::schedule::IntoScheduleConfigs;
use bevy::ecs::system::{Commands, Res, ResMut};
use bevy::image::{Image, TextureAtlasLayout};
use bevy::state::condition::in_state;
use bevy::state::state::{NextState, OnEnter};

use crate::assets::aseprite_json::AsepriteJson;
use crate::assets::tiled_map_json::TiledMapJson;
use crate::loaders::aseprite_json::AsepriteJsonLoader;
use crate::loaders::tiled_map_json::TiledMapJsonLoader;
use crate::models::asset_cog_animation_code::AssetCogAnimationCode;
use crate::models::asset_cog_sprite_code::AssetCogSpriteCode;
use crate::models::asset_projectile_animation_code::AssetProjectileAnimationCode;
use crate::models::asset_projectile_sprite_code::AssetProjectileSpriteCode;
use crate::models::asset_toon_animation_code::AssetToonAnimationCode;
use crate::models::asset_toon_sprite_code::AssetToonSpriteCode;
use crate::models::asset_ui_animation_code::AssetUiAnimationCode;
use crate::models::asset_ui_sprite_code::AssetUiSpriteCode;
use crate::plugins::asset::asset_constants::{COGS_ANIMATION_PATH, COGS_IMAGE_PATH, GLOBAL_ANIMATION_PATHS, GLOBAL_IMAGE_PATHS, PROJECTILES_ANIMATION_PATH, PROJECTILES_IMAGE_PATH, TOONS_ANIMATION_PATH, TOONS_IMAGE_PATH, UI_ANIMATION_PATH, UI_IMAGE_PATH};
use crate::plugins::asset::asset_utils::{build_atlas_layout, build_sprite_map};
use crate::resources::cog_sprite_store::CogSpriteStore;
use crate::resources::global_asset_store::GlobalAssetStore;
use crate::resources::projectile_sprite_store::ProjectileSpriteStore;
use crate::resources::toon_sprite_store::ToonSpriteStore;
use crate::resources::ui_sprite_store::UiSpriteStore;
use crate::states::app_state::AppState;

pub struct AssetPlugin;

impl Plugin for AssetPlugin {
  fn build(&self, app: &mut bevy::app::App) {
    app.init_asset::<AsepriteJson>();
    app.init_asset::<TiledMapJson>();

    app.register_asset_loader(AsepriteJsonLoader);
    app.register_asset_loader(TiledMapJsonLoader);

    app.init_resource::<GlobalAssetStore>();

    app.add_systems(OnEnter(AppState::LoadingAssets), (load_global_images, load_global_animations));
    app.add_systems(Update, check_global_assets_loaded.run_if(in_state(AppState::LoadingAssets)));
    app.add_systems(
      OnEnter(AppState::ProcessingAssets),
      (
        process_ui_assets,
        process_cog_assets,
        process_projectile_assets,
        process_toon_assets,
        finish_processing_assets,
      ).chain(),
    );
  }
}

fn load_global_images(
  mut global_asset_store: ResMut<GlobalAssetStore>,
  asset_server: Res<AssetServer>,
) {
  // Load all global image files and retain their handles for entire app lifetime 
  for image_path in GLOBAL_IMAGE_PATHS {
    let image_handle = asset_server.load::<Image>(image_path);
    global_asset_store.image_handles.insert(image_path.to_string(), image_handle);
  }
}

fn load_global_animations(
  mut global_asset_store: ResMut<GlobalAssetStore>,
  asset_server: Res<AssetServer>,
) {
  // Load all global animation files and retain their handles for entire app lifetime 
  for animation_path in GLOBAL_ANIMATION_PATHS {
    let animation_handle = asset_server.load::<AsepriteJson>(animation_path);
    global_asset_store.animation_handles.insert(animation_path.to_string(), animation_handle);
  }
}

fn check_global_assets_loaded(
  global_asset_store: Res<GlobalAssetStore>,
  asset_server: Res<AssetServer>,
  mut next_state: ResMut<NextState<AppState>>,
) {
  let image_asset_handles = global_asset_store.image_handles.iter().map(|(_, v)| v.clone().untyped());
  let animation_asset_handles = global_asset_store.animation_handles.iter().map(|(_, v)| v.clone().untyped());
  let all_asset_handles = image_asset_handles
    .chain(animation_asset_handles)
    .collect::<Vec<_>>();

  for handle in all_asset_handles {
    if !asset_server.is_loaded(handle.id()) {
      // Something is not loaded yet
      return;
    }
  }

  // Done loading
  next_state.set(AppState::ProcessingAssets);
}

fn process_ui_assets(
  mut commands: Commands,
  aseprite_json_assets: Res<Assets<AsepriteJson>>,
  mut atlas_layout_assets: ResMut<Assets<TextureAtlasLayout>>,
  asset_server: Res<AssetServer>,
) {
  let image_handle = asset_server.get_handle(UI_IMAGE_PATH)
    .expect("UI image handle not found");

  let aseprite_json_handle = asset_server.get_handle::<AsepriteJson>(UI_ANIMATION_PATH)
    .expect("UI animation handle not found");

  let aseprite_json = aseprite_json_assets.get(aseprite_json_handle.id())
    .expect("UI animation data not found");

  let atlas_layout = build_atlas_layout(&aseprite_json);
  let atlas_layout_handle = atlas_layout_assets.add(atlas_layout);

  let sprite_map = build_sprite_map::<AssetUiSpriteCode, AssetUiAnimationCode>(&aseprite_json);

  let ui_sprite_store = UiSpriteStore {
    image_handle,
    atlas_layout_handle,
    sprite_map,
  };
  commands.insert_resource(ui_sprite_store);
}

fn process_cog_assets(
  mut commands: Commands,
  aseprite_json_assets: Res<Assets<AsepriteJson>>,
  mut atlas_layout_assets: ResMut<Assets<TextureAtlasLayout>>,
  asset_server: Res<AssetServer>,
) {
  let image_handle = asset_server.get_handle::<Image>(COGS_IMAGE_PATH)
    .expect("Cogs image handle not found");

  let aseprite_json_handle = asset_server.get_handle::<AsepriteJson>(COGS_ANIMATION_PATH)
    .expect("Cogs animation handle not found");

  let aseprite_json = aseprite_json_assets.get(aseprite_json_handle.id())
    .expect("Cogs animation data not found");

  let atlas_layout = build_atlas_layout(aseprite_json);
  let atlas_layout_handle = atlas_layout_assets.add(atlas_layout);

  let sprite_map = build_sprite_map::<AssetCogSpriteCode, AssetCogAnimationCode>(aseprite_json);

  let cog_asset_store = CogSpriteStore {
    image_handle,
    atlas_layout_handle,
    sprite_map,
  };
  commands.insert_resource(cog_asset_store);
}

fn process_projectile_assets(
  mut commands: Commands,
  aseprite_json_assets: Res<Assets<AsepriteJson>>,
  mut atlas_layout_assets: ResMut<Assets<TextureAtlasLayout>>,
  asset_server: Res<AssetServer>,
) {
  let image_handle = asset_server.get_handle::<Image>(PROJECTILES_IMAGE_PATH)
    .expect("Projectile image handle not found");

  let aseprite_json_handle = asset_server.get_handle::<AsepriteJson>(PROJECTILES_ANIMATION_PATH)
    .expect("Projectile animation handle not found");

  let aseprite_json = aseprite_json_assets.get(aseprite_json_handle.id())
    .expect("Projectile animation data not found");

  let atlas_layout = build_atlas_layout(aseprite_json);
  let atlas_layout_handle = atlas_layout_assets.add(atlas_layout);

  let sprite_map = build_sprite_map::<AssetProjectileSpriteCode, AssetProjectileAnimationCode>(aseprite_json);

  let projectile_sprite_store = ProjectileSpriteStore {
    image_handle,
    atlas_layout_handle,
    sprite_map,
  };
  commands.insert_resource(projectile_sprite_store);
}

fn process_toon_assets(
  mut commands: Commands,
  aseprite_json_assets: Res<Assets<AsepriteJson>>,
  mut atlas_layout_assets: ResMut<Assets<TextureAtlasLayout>>,
  asset_server: Res<AssetServer>,
) {
  let image_handle = asset_server.get_handle::<Image>(TOONS_IMAGE_PATH)
    .expect("Toons image handle not found");

  let aseprite_json_handle = asset_server.get_handle::<AsepriteJson>(TOONS_ANIMATION_PATH)
    .expect("Toons animation handle not found");

  let aseprite_json = aseprite_json_assets.get(aseprite_json_handle.id())
    .expect("Toons animation data not found");

  let atlas_layout = build_atlas_layout(aseprite_json);
  let atlas_layout_handle = atlas_layout_assets.add(atlas_layout);

  let sprite_map = build_sprite_map::<AssetToonSpriteCode, AssetToonAnimationCode>(aseprite_json);

  let toon_sprite_store = ToonSpriteStore {
    image_handle,
    atlas_layout_handle,
    sprite_map,
  };
  commands.insert_resource(toon_sprite_store);
}

fn finish_processing_assets(
  mut next_state: ResMut<NextState<AppState>>,
) {
  // Everything is loaded, proceed to next loading screen
  next_state.set(AppState::LoadingProfile);
}