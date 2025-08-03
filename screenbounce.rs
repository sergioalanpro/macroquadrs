use macroquad::prelude::*;


struct Ball {
    pos: Vec2,
    vel: Vec2,
    radius: f32,
}


#[macroquad::main("Anim #1")]
async fn main() {
    let mut ball = Ball {
        pos:vec2(screen_width()/2.0, screen_height()/2.0),
        vel: vec2(5.0,-5.0),
        radius: 15.0,
    };


    let gravity = 0.3;


    loop {
        ball.vel.y += gravity;
        ball.pos += ball.vel;


        if ball.pos.x - ball.radius <= 0.0 {
            ball.pos.x = ball.radius;
            ball.vel.x *= -0.95; 
        } else if ball.pos.x + ball.radius >= screen_width() {
            ball.pos.x = screen_width() - ball.radius;
            ball.vel.x *= -0.95;
        }
        if ball.pos.y - ball.radius <= 0.0 {
            ball.pos.y = ball.radius;
            ball.vel.y *= -0.95;
        } else if ball.pos.y + ball.radius >= screen_height() {
            ball.pos.y = screen_height() - ball.radius;
            ball.vel.y *= -0.95; 
            }
      

        clear_background(BLACK);
        draw_circle(ball.pos.x, ball.pos.y, ball.radius, RED);
        draw_text("Screen bounce", 20.0, 20.0, 40.0, WHITE);

        
        next_frame().await;
    }
}
