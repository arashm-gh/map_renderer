use raylib::prelude::*;
use simple_iso_renderer::*;

fn main() -> Result<(), ()> {
    let (mut rl, thread) = raylib::init()
        .size(1280, 720)
        .title("Full Map Loader Example")
        .build();

    let mut map: Map = simple_iso_renderer::Map::new(
        BlockSettings {
            block_width: 32.0,
            block_height: 32.0,
        },
        "assets/blocks.json".to_string(),
    );
    map.load_blocks("assets/map.json");
    map.y_sort();

    map.registry.load_textures(&mut rl, &thread);

    rl.set_target_fps(12);
    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread); // use mode2d for camera
        d.clear_background(Color::DARKGRAY);
        map.draw_map(&mut d);
    }

    Ok(())
}
