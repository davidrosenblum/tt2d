#[derive(Debug)]
pub enum TilemapSpawnError {
  /** Map file does not have sufficient tilesets defined, used for first guid. */
  TilesetsLength,
  /** Map file is missing or has invalid terrain layer. */
  MissingInvalidTerrainLayer,
  /** Map file is missing or has invalid terrain transition layer. */
  MissingInvalidTerrainTransitionLayer,
  /** Map file is missing or has invalid structure bottom layer. */
  MissingInvalidStructureBottomLayer,
  /** Map file is missing or has invalid structure top layer. */
  MissingInvalidStructureTopLayer,
}