use std::hash::Hash;
use std::str::FromStr;

use bevy::image::TextureAtlasLayout;
use bevy::log::warn;
use bevy::math::{URect, UVec2};
use bevy::platform::collections::HashMap;

use crate::assets::aseprite_json::AsepriteJson;
use crate::errors::asset_parse_filename_error::AssetParseFilenameError;
use crate::models::sprite_animation_frame_data::SpriteAnimationFrameData;
use crate::models::sprite_map::SpriteMap;

pub fn parse_filename<S: FromStr, A: FromStr>(filename: &str) -> Result<(S, A, usize), AssetParseFilenameError> {
  // Expected format is "{layer}:{tag}:{tagframe}"
  // Tagframe is required to not have colliding names when an animation has multiple frames

  // Must have both nested variables
  let sections = filename.split(":").collect::<Vec<_>>();
  if sections.len() != 3 {
    return Err(AssetParseFilenameError::Malformed);
  }

  // Parse and validate the {layer} to spriteCode
  let layer = sections[0];
  let sprite_code = layer.parse::<S>()
    .map_err(|_| AssetParseFilenameError::InvalidSpriteCode)?;

  // Parse and validate the {tag} to animationCode
  let tag = sections[1];
  let animation_code = tag.parse::<A>()
    .map_err(|_| AssetParseFilenameError::InvalidAnimationCode)?;

  let tagframe = sections[2];
  let index = tagframe.parse::<usize>()
    .map_err(|_| AssetParseFilenameError::InvalidIndex)?;

  // Success
  Ok((sprite_code, animation_code, index))
}


pub fn build_sprite_map<S: FromStr + Eq + Hash, A: FromStr + Eq + Hash>(
  aseprite_json: &AsepriteJson,
) -> SpriteMap<S, A> {
  // Empty map of sprite => animations[] => frames[]
  let mut map = HashMap::<S, HashMap<A, Vec<SpriteAnimationFrameData>>>::new();

  for (index, (filename, frame)) in aseprite_json.frames.iter().enumerate() {
    // Parse the filename, skip if invalid
    let Ok((sprite_code, animation_code, _)) = parse_filename::<S, A>(filename) else {
      warn!("Failed to parse filename: {}", filename);
      continue;
    };

    // Create the frame data
    let animation_frame_data = SpriteAnimationFrameData {
      duration: frame.duration,
      layout_index: index,
    };
 
    // Create or get the nested animation map
    let animation_map = map.entry(sprite_code).or_default();

    // Create new empty array or get the populated array of animation frames
    let animation_frames = animation_map.entry(animation_code).or_default();

    // Push the latest frame
    animation_frames.push(animation_frame_data);
  }

  // Done
  SpriteMap::new(map)
}

pub fn build_atlas_layout(aseprite_json: &AsepriteJson) -> TextureAtlasLayout {
  // Texture size is in json metadata
  let dimensions = UVec2::new(
    aseprite_json.meta.size.w,
    aseprite_json.meta.size.h,
  );
  let mut atlas_layout = TextureAtlasLayout::new_empty(dimensions);

  // Define where each sprite is in the texture
  for frame in aseprite_json.frames.values() {
    let rect = URect::new(
      frame.frame.x,
      frame.frame.y,
      frame.frame.x + frame.frame.w,
      frame.frame.y + frame.frame.h,
    );
    atlas_layout.add_texture(rect);
  }

  atlas_layout
}