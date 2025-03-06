/*
By: <Your Name Here>
Date: 2025-03-06
Program Details: <Program Description Here>
*/
mod screen1;
mod screen2;
use macroquad::prelude::*;

#[macroquad::main("Screen Manager")]
async fn main() {
    let mut current_screen = "screen1".to_string();
    let mut last_switch = get_time(); // Use get_time() to get the current time

    loop {
        if get_time() - last_switch > 0.01 { // 10ms cooldown
            current_screen = match current_screen.as_str() {
                "screen1" => screen1::run().await,
                "screen2" => screen2::run().await,
                _ => break,
            };
            last_switch = get_time(); // Reset cooldown timer
        }
        next_frame().await;
    }
}
