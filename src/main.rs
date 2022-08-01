use macroquad::prelude::*;

#[macroquad::main("The thingy.")]
async fn main() {

    let vietnam_map = load_texture("img/small_asia.png").await.unwrap();
    let image_itself = vietnam_map.get_texture_data();

    loop {
        clear_background(LIGHTGRAY);

        let (mouse_x, mouse_y) = (mouse_position().0, mouse_position().1);
        let pixel = image_itself.get_pixel(mouse_x as u32, mouse_y as u32);

        draw_texture(vietnam_map,
                     screen_width() / 2. - vietnam_map.width() / 2.,
                     screen_height() / 2. - vietnam_map.height() / 2.,
                     WHITE);

        draw_text(&*format!("r:{}, g: {}, b: {}", pixel.r, pixel.b, pixel.b), 20.0, 50.0, 20.0, RED);
        draw_text(&*format!("{:?}", Color::from_rgba(pixel.r as u8, pixel.g as u8, pixel.b as u8, pixel.a as u8)), 20.0, 70.0, 20.0, RED);
        draw_text(&*format!("{}, {}", mouse_position().0 as u32, mouse_position().1 as u32), 20.0, 20.0, 20.0, RED);

        // eprintln!("{:?}", Color::from_rgba(pixel.r as u8, pixel.g as u8, pixel.b as u8, pixel.a as u8));

        next_frame().await
    }

    fn round(x: f32, decimals: u32) -> f32 {
        let y = 10i32.pow(decimals) as f32;
        (x * y).floor() / y
    }

}
