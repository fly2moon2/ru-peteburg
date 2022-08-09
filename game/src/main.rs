use macroquad::prelude::*;

#[derive(Default)]
pub struct PlayerState {
    pub position: Vec2,
    pub rotation: f32,
}

pub struct Game {
    pub quit: bool,
    pub player_state: PlayerState,
    pub texture: Texture2D,
    pub game_speed:f32,
}

impl Game {
    pub async fn new() -> Self {
        let texture = load_texture("assets/plane.png").await.unwrap();
        //
       
        Self {
            player_state: PlayerState {
                position: Vec2::new(100f32, 100f32),
                rotation: 0f32,
            },
            texture,
            quit: false,
            game_speed: 0.6f32,
        }
    }

    pub fn update(&mut self) {
        if is_key_down(KeyCode::Escape) {
            self.quit = true;
        }
        const ROT_SPEED: f32 = 0.015;

        if is_key_down(KeyCode::Right) {
            self.player_state.rotation += ROT_SPEED;
        }
        if is_key_down(KeyCode::Left) {
            self.player_state.rotation -= ROT_SPEED;
        }

        if is_key_down(KeyCode::Up) {
            self.game_speed = self.game_speed + 0.6;
        }
        if is_key_down(KeyCode::Down) {
            self.game_speed = self.game_speed - 0.2;
        }

        const SPEED: f32 = 0.6;
        

        //self.player_state.position += vec2_from_angle(self.player_state.rotation) * SPEED;
        self.player_state.position += vec2_from_angle(self.player_state.rotation) * self.game_speed;

        if self.player_state.position.x > screen_width() {
            self.player_state.position.x = -self.texture.width();
        } else if self.player_state.position.x < -self.texture.width() {
            self.player_state.position.x = screen_width();
        }

        if self.player_state.position.y > screen_height() {
            self.player_state.position.y = -self.texture.height();
        } else if self.player_state.position.y < -self.texture.height() {
            self.player_state.position.y = screen_height();
        }
    }

    pub fn draw(&self) {
        //clear_background(color_u8!(255, 255, 255, 255));

        draw_texture_ex(
            self.texture,
            self.player_state.position.x,
            self.player_state.position.y,
            WHITE,
            DrawTextureParams {
                rotation: self.player_state.rotation,
                ..Default::default()
            },
        );

        draw_box(Vec2::new(250f32, 200f32), Vec2::new(15f32, 12f32));
    }
}

pub fn vec2_from_angle(angle: f32) -> Vec2 {
    let angle = angle - std::f32::consts::FRAC_PI_2;
    Vec2::new(angle.cos(), angle.sin())
}

fn draw_box(pos: Vec2, size: Vec2) {
    let dimension = size * 2.;
    let upper_left = pos - size;

    draw_rectangle(upper_left.x, upper_left.y, dimension.x, dimension.y, BLACK);
}

#[macroquad::main("game")]
async fn main() {
    let mut game = Game::new().await;
    //
    let rust_logo = load_texture("assets/plane.png").await.unwrap();

    loop {
        clear_background(LIGHTGRAY);

/*         set_camera(&Camera2D {
            zoom: vec2(1., screen_width() / screen_height()),
            ..Default::default()
        }); */
        // Going 3d!

        set_camera(&Camera3D {
            position: vec3(-20., 15., 0.),
            up: vec3(0., 1., 0.),
            target: vec3(0., 0., 0.),
            ..Default::default()
        });

        draw_grid(20, 1., BLACK, GRAY);

        draw_cube_wires(vec3(0., 1., -6.), vec3(2., 2., 2.), DARKGREEN);
        draw_cube_wires(vec3(0., 1., 6.), vec3(2., 2., 2.), DARKBLUE);
        draw_cube_wires(vec3(2., 1., 2.), vec3(2., 2., 2.), YELLOW);


        draw_cube(vec3(-5., 1., -2.), vec3(2., 2., 2.), rust_logo, WHITE);

        draw_cube(vec3(2., 0., -2.), vec3(0.4, 0.4, 0.4), None, BLACK);

        draw_sphere(vec3(-8., 0., 0.), 1., None, BLUE);

        draw_sphere(vec3(10., 10., 10.), 1., rust_logo, BLACK);

        // Back to screen space, render some text

        set_default_camera();
        draw_text("WELCOME TO 3D WORLD", 10.0, 20.0, 30.0, BLACK);
        draw_text("In an unsafe world", 20.0, 30.0, 30.0, BLUE);
        // 3d example ends
        game.update();
        game.draw();
        if game.quit {
            return;
        }
        next_frame().await
    }
}
