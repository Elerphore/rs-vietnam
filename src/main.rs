use macroquad::prelude::*;

#[macroquad::main("The thingy.")]
async fn main() {

    let vietnam_map = load_texture("img/small_asia.png").await.unwrap();

    loop {
        clear_background(LIGHTGRAY);

        draw_texture(vietnam_map,
                     screen_width() / 2. - vietnam_map.width() / 2.,
                     screen_height() / 2. - vietnam_map.height() / 2.,
                     RED);

        draw_text(&*format!("{}, {}", mouse_position().0, mouse_position().1), 20.0, 20.0, 20.0, DARKGRAY);
        next_frame().await
    }
}
