use raylib::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct BlockDef {
    pub name: String,
    pub texture_path: String,
    pub walkable: bool,
    pub collider: bool,
    pub shader: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Block {
    pub name: String,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Block {
    pub fn new(name: String, x: f32, y: f32, z: f32) -> Self {
        Block { name, x, y, z }
    }
}

pub struct BlockRegistry {
    pub defs: HashMap<String, BlockDef>,
    pub textures: HashMap<String, Texture2D>,
}

impl BlockRegistry {
    pub fn new() -> Self {
        BlockRegistry {
            defs: HashMap::new(),
            textures: HashMap::new(),
        }
    }

    pub fn from_json(path: String) -> Self {
        let json_str = std::fs::read_to_string(path).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json_str).unwrap();

        let mut registry = BlockRegistry::new();
        if let Some(blocks) = parsed["blocks"].as_array() {
            for b in blocks {
                let def: BlockDef = serde_json::from_value(b.clone()).unwrap();
                registry.defs.insert(def.name.clone(), def);
            }
        }
        registry
    }

    pub fn load_textures(&mut self, rl: &mut RaylibHandle, thread: &RaylibThread) {
        for (id, def) in &self.defs {
            let texture = rl.load_texture(thread, &def.texture_path).unwrap();
            println!("Loaded Texture: {:?}", texture);
            let t = self.textures.insert(id.clone(), texture);
            if t.is_none() {
                println!("Could not load texture!");
            }
        }
        for (i, (name, texture)) in self.textures.iter().enumerate() {
            println!("TEXTURE {} as {}: {:?}", i, name, texture);
        }
    }

    pub fn get(&self, id: &str) -> Option<&BlockDef> {
        self.defs.get(id)
    }

    pub fn get_texture(&self, id: &str) -> Option<&Texture2D> {
        self.textures.get(id)
    }
}
