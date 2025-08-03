use macroquad::prelude::*;

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

    Ball {
        pos: vec2(mouse_position().0, mouse_position().1),
        vel: vec2(
            rand::gen_range(-0.0, 0.0),
            rand::gen_range(-0.0, 0.0),
        ),
        radius: 15.0,
        color: WHITE,
    }
}

#[macroquad::main("Ball bouncing")]
async fn main() {
    
    let mut circle = Circle {
        pos: vec2(screen_width() / 2.0, screen_height() / 2.0),
        sides: 255,
        radius: 150.0,
        rotation: 0.0,
        thickness: 5.0,
        color: WHITE,
    };

    let mut balls: Vec<Ball> = vec![];

    let mut gravity = 0.8;

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
             if is_key_down(KeyCode::Up) {
            added_ball.vel *= 1.05;
             }
             if is_key_down(KeyCode::Down) {
            added_ball.vel *= 0.95;
             }
             if is_key_pressed(KeyCode::B) {
            added_ball.color = Color::from_rgba(
                                                rand::gen_range(0, 255),  
                                                rand::gen_range(0, 255),
                                                rand::gen_range(0, 255),
                                                255,
                                                );
             }
             
             
        
        }
         if is_key_pressed(KeyCode::G) {
            gravity += 0.1;
        }
        if is_key_pressed(KeyCode::F) {
            gravity -= 0.1;
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            balls.push(create_ball());
        }
        if is_mouse_button_pressed(MouseButton::Right) {
            balls.pop(); 
        }
        if is_key_down(KeyCode::D) {
            balls.clear(); 
        }

        if is_key_down(KeyCode::Left) {
            circle.radius -= 1.0;
        }
        if is_key_down(KeyCode::Right) {
            circle.radius += 1.0;
        }
      if is_key_pressed(KeyCode::C) {
            circle.color = Color::from_rgba(
                                                rand::gen_range(0, 255), 
                                                rand::gen_range(0, 255),
                                                rand::gen_range(0, 255),
                                                255,
                                                );
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

        let gravity_rounded = (gravity * 10.0).round() / 10.0;

        draw_text(&format!("Gravedad: {}", gravity_rounded), 20.0,   285.0,   30.0,    RED);

        draw_text(&format!("Bolas:  {}",balls.len()), 20.0, 250.0, 30.0, RED);
        //draw_text(&format!("Gravedad:  {}",gravity.round()as f32 / 10.0), 20.0, 285.0, 30.0, RED);


        draw_text("C: para cambiar el color del círculo", 20.0, 20.0, 20.0, WHITE);
        draw_text("B:  para cambiar el color de las bolas", 20.0, 40.0, 20.0, WHITE);
        draw_text(">: Aumentar tamaño de circulo", 20.0, 60.0, 20.0, WHITE);
        draw_text("<: Disminuir tamaño de circulo", 20.0, 80.0, 20.0, WHITE);
        draw_text("^: Aumentar velocidad ", 20.0, 100.0, 20.0, WHITE);
        draw_text("v: Disminuir velocidad", 20.0, 120.0, 20.0, WHITE);
        draw_text("G: Aumentar gravedad", 20.0, 140.0, 20.0, WHITE);
        draw_text("F: Disminuir gravedad", 20.0, 160.0, 20.0, WHITE);
      
        draw_text("Click: para crear una bola", screen_width()-250.0, 20.0, 20.0, WHITE);
        draw_text("Click derecho: para eliminar una bola", screen_width()-250.0, 40.0, 20.0, WHITE);
        draw_text("D: Borrar todo", screen_width()-250.0, 60.0, 20.0, WHITE);

        next_frame().await;
    }
}
