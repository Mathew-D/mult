use macroquad::prelude::*;

pub async fn run() -> String {
    

    loop {
        clear_background(BLUE);
        draw_text("Screen 1", 20.0, 40.0, 30.0, WHITE);
      

        if is_key_pressed(KeyCode::Space) {
            return "screen2".to_string(); // Go to Screen 2
        }

        next_frame().await;
    }
}
