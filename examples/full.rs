use map_renderer::*;
use raylib::prelude::*;

fn main() -> Result<(), ()> {
    let (mut rl, thread) = raylib::init()
        .size(800, 600)
        .title("Full Map Loader Example")
        .build();

    // Load definitions (metadata)
    let mut registry = map_renderer::BlockRegistry::from_json(String::from("assets/blocks.json"));
    registry.load_textures(&mut rl, &thread);
    
    // Load map (instances)
    let mut map: Map = map_renderer::Map::new(
        BlockSettings {
            block_width: 64.0,
            block_height: 64.0,
        },
        "assets/blocks.json".to_string(),
    );
    map.load_blocks("assets/map.json");
    map.y_sort();
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::GRAY);
        map.draw_map(&mut d);
    }

    Ok(())
}
