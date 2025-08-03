use macroquad::prelude::*;
use macroquad::audio;

struct Ball {
    pos: Vec2,
    vel: Vec2,
    radius: f32,
}

#[macroquad::main("Mi Juego")]
async fn main() {
    let sound1 = audio::load_sound("assets/jump.wav").await.unwrap();
    let sound2 = audio::load_sound("assets/death.wav").await.unwrap();

    let gravity = 0.1;
    let platform = Rect::new(0.0, 500.0, screen_width(), 20.0);
    let obstacle = Rect::new(screen_width() / 2.0, screen_height() - 120.0, 20.0, 20.0);

    let mut ball = Ball {
        pos: vec2(platform.x + 41.0, platform.y - 50.0),
        vel: vec2(0.0, 0.0),
        radius: 20.0,
    };

    let mut game_over = false;

    loop {
        if game_over == false {

            audio::stop_sound(sound2);
            ball.vel.y += gravity;
            ball.pos += ball.vel;

            if is_key_down(KeyCode::Right) {
                ball.pos.x += 5.0;
            }
            if is_key_down(KeyCode::Left) {
                ball.pos.x -= 5.0;
            }

            if is_key_down(KeyCode::Up) && platform.y - ball.radius < ball.pos.y {
                ball.pos.y -= 4.0;
                ball.vel.y -= 4.0;
                audio::play_sound_once(sound1);
            }

            if ball.pos.x > screen_width() {
                ball.pos.x = 0.0;
            }
            if ball.pos.x < 0.0 {
                ball.pos.x = screen_width();
            }

            if ball.pos.x > platform.x && ball.pos.x < platform.x + platform.w &&
                ball.pos.y + ball.radius > platform.y && ball.pos.y < platform.y + platform.h {
                ball.pos.y = platform.y - ball.radius;
                ball.vel.y *= -0.5;
            }


            let closest_x = ball.pos.x.clamp(obstacle.x, obstacle.x + obstacle.w);
            let closest_y = ball.pos.y.clamp(obstacle.y, obstacle.y + obstacle.h);
            let dist_x = ball.pos.x - closest_x;
            let dist_y = ball.pos.y - closest_y;
            let distance_sq = dist_x * dist_x + dist_y * dist_y;

            if distance_sq < ball.radius * ball.radius {
                game_over = true;
                audio::play_sound_once(sound2);
                
            }
        }

        clear_background(BLACK);

        draw_rectangle(platform.x, platform.y, platform.w, platform.h, DARKGRAY);
        draw_rectangle(obstacle.x, obstacle.y, obstacle.w, obstacle.h, RED);
        draw_circle(ball.pos.x, ball.pos.y, ball.radius, BLUE);
        draw_text("GamePRO", 20.0, 20.0, 30.0, WHITE);
        draw_text(&format!("Vel: {}", ball.vel), 20.0, 60.0, 22.0, WHITE);

        if game_over == true {
    
            draw_text("YOU DIED!", screen_width() / 2.0 - 200.0, screen_height() / 2.0, 100.0, RED);
            draw_text("Presiona ESPACIO para reiniciar", screen_width() / 2.0 - 250.0, screen_height() / 2.0 + 120.0, 40.0, WHITE);

            if is_key_pressed(KeyCode::Space) {
                ball.pos = vec2(platform.x + 41.0, platform.y - 50.0);
                ball.vel = vec2(0.0, 0.0);
                game_over = false;
            }
        }

        next_frame().await;
    }
}

