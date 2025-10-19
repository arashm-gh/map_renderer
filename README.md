# Map Renderer
## Introduction
The `map_renderer` crate provides a Data Driven method to rendering isometric tiles and entities using `raylib_rs`.
## Download
```bash
cargo add simple_iso_renderer
```
## Usage
The crate works using five different structs:
1. BlockSettings
    1. Used to initalize the settings (width and height) of each sprite for calculation.
2. BlockRegistry
    1. Loads a json file (`assets/blocks.json`) that contains the definitions and descriptions of all the blocks in the world.
    2. Contains two Hashmaps, one for the definitions and one for the loaded textures for each sprite.
    3. Use functions `get` and `get_texture` to use.
3. BlockDef
    1. Contains definiton data for each type and sprite of block.
    2. Has values: `name`, `texture_path`, `collider`, `walkable`, and `shader`
4. Block
    1. Contains the position and name of the block to render
    2. Position is in 3D `x`, `y`, and `z` coordinates
5. Map
    1. Everything is handeld by the Map struct.
    2. Contains the settings, registry, and blocks.
    3. Use `Map::new(settings: BlockSettings, registry_path: String)` to create.
    4. Use `load_blocks(&mut self, path: &str)` to load a json file of the map.
    5. Use `y_sort(&mut self)` to Y-sort.

## Example
```bash
cargo run --example full
```

## Formatting
Json files must be formatted as follows:
**File for Block Definitions**
```json
{
  "blocks": [
    {
      "name": "grass",
      "texture_path": "assets/blocks/grass.png",
      "walkable": true,
      "collider": true
    },
    {
      "name": "dirt",
      "texture_path": "assets/blocks/dirt.png",
      "walkable": true,
      "collider": true
    },
    {
      "name": "stone",
      "texture_path": "assets/blocks/stone.png",
      "walkable": false,
      "collider": false
    }
  ]
}
```
**File for Map Layout**
```json
[
    { "name": "grass", "x": 0, "y": 0, "z": 0 },
    { "name": "dirt", "x": 1, "y": 0, "z": 0 },
    { "name": "stone", "x": 2, "y": 0, "z": 0 }
]
```