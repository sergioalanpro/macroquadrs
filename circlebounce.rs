use macroquad::prelude::*;
//use macroquad::math;


struct Ball {
    pos: Vec2,
    vel: Vec2,
    radius: f32,
}

struct Circle {
    pos: Vec2,
    sides: u8,
    radius: f32,
    rotation: f32,
    thickness: f32,
    color: Color,
}


#[macroquad::main("Anim #2")]
async fn main() {
    let mut ball = Ball {
        pos:vec2(screen_width()/2.0, screen_height()/2.0),
        vel: vec2(10.0,-10.0),
        radius: 15.0,
    };

    let circle = Circle {
        pos: vec2(screen_width()/2.0, screen_height()/2.0),
        sides: 255,
        radius: 200.0,
        rotation: 0.0,
        thickness: 5.0,
        color: WHITE,
    };


    let gravity = 1.0;


    loop {
        ball.vel.y += gravity;
        ball.pos += ball.vel;

        let dx = ball.pos.x - circle.pos.x;
        let dy = ball.pos.y - circle.pos.y;
        let distance = (dx * dx + dy * dy).sqrt();

        
        if distance >= circle.radius - ball.radius {
    
            let normal = vec2(dx / distance, dy / distance);

            let dot_product = ball.vel.x * normal.x + ball.vel.y * normal.y;

            ball.vel = ball.vel - 2.0 * dot_product * normal;

            ball.pos = circle.pos + normal * (circle.radius - ball.radius);
        }

        clear_background(BLACK);
        draw_circle(ball.pos.x, ball.pos.y, ball.radius, RED);

        draw_poly_lines(
            circle.pos.x,
            circle.pos.y,
            circle.sides,
            circle.radius,
            circle.rotation,
            circle.thickness,
            circle.color,
        );



        draw_text("Circle Bounce", 20.0, 20.0, 40.0, WHITE);

        
        next_frame().await;
    }
}
