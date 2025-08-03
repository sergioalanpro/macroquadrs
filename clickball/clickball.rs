use macroquad::prelude::*;
use ::rand::Rng;       
use ::rand::thread_rng;  

pub struct Ball {
    pos: Vec2,
    vel: Vec2,
    radius: f32,
    color: Color,
}

struct Circle {
    pos: Vec2,
    sides: u8,
    radius: f32,
    rotation: f32,
    thickness: f32,
    color: Color,
}

pub fn create_ball() -> Ball {
    let mut rng = thread_rng(); 
    let radius = 15.0;
    let pos = (mouse_position().1, mouse_position().0);
    let vel = vec2(
        rng.gen_range(-10.0..10.0),
        rng.gen_range(-10.0..10.0),
    );
    Ball {
        pos: vec2(pos.1, pos.0),
        vel: vec2(vel.x, vel.y),
        radius: radius,
        color: WHITE,
    }
}

#[macroquad::main("Anim #2")]
async fn main() {
    
    let circle = Circle {
        pos: vec2(screen_width() / 2.0, screen_height() / 2.0),
        sides: 255,
        radius: 300.0,
        rotation: 0.0,
        thickness: 5.0,
        color: WHITE,
    };

    let mut balls: Vec<Ball> = vec![];

    let gravity = 0.1;


    loop {
        clear_background(BLACK);

        for added_ball in balls.iter_mut() {
            added_ball.vel.y += gravity;
            added_ball.pos += added_ball.vel;

            let dx = added_ball.pos.x - circle.pos.x;
            let dy = added_ball.pos.y - circle.pos.y;
            let distance = (dx * dx + dy * dy).sqrt();

            if distance >= circle.radius - added_ball.radius {
                let normal = vec2(dx / distance, dy / distance);
                let dot_product = added_ball.vel.x * normal.x + added_ball.vel.y * normal.y;
                added_ball.vel = added_ball.vel - 2.0 * dot_product * normal;
                added_ball.pos = circle.pos + normal * (circle.radius - added_ball.radius);
            }

            draw_circle(added_ball.pos.x, added_ball.pos.y, added_ball.radius, added_ball.color);
        
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            balls.push(Ball {
                pos: create_ball().pos,
                vel: create_ball().vel,
                radius: create_ball().radius,
                color: create_ball().color,
            });
        }
        if is_mouse_button_pressed(MouseButton::Right) {
            balls.pop(); 
        }
        if is_key_down(KeyCode::Backspace) {
            balls.clear(); 
        }

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
        draw_text(&format!("Bolas:  {}",balls.len()), 20.0, 40.0, 30.0, RED);

        next_frame().await;
    }
}
