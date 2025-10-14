use raylib::prelude::*;
pub mod block;
pub use block::*;

pub struct BlockSettings {
    pub block_width: f32,
    pub block_height: f32,
}

impl BlockSettings {
    pub fn new(block_width: f32, block_height: f32) -> Self {
        BlockSettings {
            block_width,
            block_height,
        }
    }
}

pub struct Map {
    pub settings: BlockSettings,
    pub registry: BlockRegistry,
    pub blocks: Vec<Block>,
}

impl Map {
    pub fn new(settings: BlockSettings, registry_path: String) -> Self {
        Map {
            settings,
            registry: BlockRegistry::from_json(registry_path),
            blocks: vec![],
        }
    }

    pub fn load_blocks(&mut self, path: &str) {
        let data = std::fs::read_to_string(path).unwrap();
        let loaded: Vec<Block> = serde_json::from_str(&data).unwrap();
        self.blocks = loaded;
    }

    pub fn y_sort(&mut self) {
        self.blocks.sort_by(|a, b| {
            let da = a.x + a.y + a.z;
            let db = b.x + b.y + b.z;
            da.partial_cmp(&db).unwrap_or(std::cmp::Ordering::Equal)
        });
    }

    pub fn draw_map(&self, d: &mut RaylibDrawHandle) {
        for block in self.blocks.iter() {
            println!("{}", block.id);
            println!("{:?}", self.registry.get_texture(&block.id));
            if let Some(tex) = self.registry.get_texture(&block.id) {
                let (sx, sy) = Self::iso_to_screen(block.x, block.y, block.z, &self.settings);
                d.draw_texture(tex, sx as i32, sy as i32, Color::WHITE);
                println!("Drawing: {:?}", block.id);
            }
        }
    }

    pub fn iso_to_screen(x: f32, y: f32, z: f32, settings: &BlockSettings) -> (f32, f32) {
        let bw = settings.block_width;
        let bh = settings.block_height;
        let sx = (x - y) * (bw / 2.0);
        let sy = (x + y) * (bh / 4.0) - z * bh;
        (sx, sy)
    }
}
