use macroquad::prelude::*;

//use macroquad_font_renderer::Fonts;
//use macroquad_tiled as tiled;
/* use macroquad::{

    experimental::{
        collections::storage,
        coroutines::start_coroutine,

    },
    ui,
}; */


//const NOTO_SANS: &[u8] = include_bytes!("../assets/fonts/NotoSans-Regular.ttf");

//use std::mem::size_of;

static B:[u8;4]=[34,56,123,23];
static C:[u8;7]=[3,33,54,231,34,45,98];


pub fn init() {
    let a:usize=42;
    let b:&[u8;4]=&B;
    let c:Box<[u8]>=Box::new(C);

    println!("a is: {}",a);
    println!("location: {:p}",&a);


}

#[derive(Default)]
pub struct PlayerState {
    pub position: Vec2,
    pub rotation: f32,
}

pub struct Game {
    pub quit: bool,
    pub player_state: PlayerState,
    pub texture: Texture2D,
    pub texture2: Texture2D,
    pub game_speed:f32,
    pub opacity:f32
}

impl Game {
    pub async fn new() -> Self {
        let texture = load_texture("assets/plane.png").await.unwrap();
        let texture2 = load_texture("assets/starship1.png").await.unwrap();
        //
       


        Self {
            player_state: PlayerState {
                position: Vec2::new(100f32, 100f32),
                rotation: 0f32,
            },
            texture,
            texture2,
            quit: false,
            game_speed: 0.6f32,
            opacity: 1.0f32,
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

        // add/minus opacity
        if is_key_down(KeyCode::A) && (self.opacity < 1.0) {
            self.opacity = self.opacity + 0.02;
        }
        if is_key_down(KeyCode::D) && (self.opacity > 0.0) {
            self.opacity = self.opacity - 0.02;
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

        // texture
        draw_texture_ex(
            self.texture,
            self.player_state.position.x,
            self.player_state.position.y,
            //WHITE,
            Color::new(1.00,1.00,1.00,self.opacity),
            DrawTextureParams {
                rotation: self.player_state.rotation,
                ..Default::default()
            },
        );

        //texture2
        draw_texture_ex(
            self.texture2,
            self.player_state.position.x+100f32,
            self.player_state.position.y+100f32,
            //WHITE,
            Color::new(1.00,1.00,1.00,self.opacity),
            DrawTextureParams {
                rotation: self.player_state.rotation,
                ..Default::default()
            },
        );

        draw_box(Vec2::new(self.player_state.position.x + 100f32, self.player_state.position.y + 50f32), Vec2::new(15f32, 12f32));
    

        draw_text_ex("st. petersburg", self.player_state.position.x + 150f32, 120.0, TextParams::default());

        
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

//https://github.com/heroiclabs/fishgame-macroquad/blob/master/src/main.rs
/* struct Resources {

    tiled_map: tiled::Map,
    stone:Texture2D,

}
 */
/* impl Resources {
    async fn new() -> Result<Resources, macroquad::prelude::FileError> {
        let tileset = load_texture("assets/tileset.png").await?;
        tileset.set_filter(FilterMode::Nearest);

        let stone = load_texture("assets/stone.png").await?;
        stone.set_filter(FilterMode::Nearest);

    let tiled_map_json = load_string("assets/map.json").await.unwrap();
    let tiled_map = tiled::load_map(
        &tiled_map_json,
        &[("tileset.png", tileset), ("stone.png", stone)],
        &[],
    )
    .unwrap();

    Ok(Resources {

        tiled_map,
        stone,

    })
    }
} */

/* pub fn tiled(){

    let resources_loading = start_coroutine(async move {
        let resources = Resources::new().await.unwrap();
    });
    while resources_loading.is_done() == false {
    }
    let resources = storage::get::<Resources>();
    for object in &resources.tiled_map.layers["stone"].objects {
        scene::add_node(Decoration::new(
            vec2(object.world_x, object.world_y),
            object.gid.unwrap(),
        ));
    }
    drop(resources);

} */

#[macroquad::main("game")]
async fn main() {
    let mut game = Game::new().await;
    //
    let rust_logo = load_texture("assets/plane.png").await.unwrap();

    //
    //init()
    init();
    //tiled();

    //static mut FONT_SIZE: f32 = 32.0;

    let font = load_ttf_font("./assets/fonts/RobotoCondensed-LightItalic.ttf").await.unwrap();
    // vertical fonts not supported
    //let font = load_ttf_font("./assets/fonts/NotoSansHK-Regular.otf").await.unwrap();
    loop {
        clear_background(LIGHTGRAY);
        
        draw_text_ex("Custom font size:", 120.0, 120.0, TextParams::default());
        let mut y = 20.0;

        //https://pullanswer.com/questions/font-sizes-different-between-wasm-native

/*         let textparams = TextParams {
            font,
            font_size: unsafe { FONT_SIZE as u16 },
            color: BLACK,
            ..Default::default()
        }; */

        for font_size in (30..100).step_by(20) {
            let text = "abcdef";
            let params = TextParams {
                font,
                font_size,
                ..Default::default()
            };

            y += font_size as f32;
            draw_text_ex(text, 20.0, y, params);
        }

        draw_text_ex("Dynamic font scale:", 20.0, 400.0, TextParams::default());
        draw_text_ex(
            "abcd",
            20.0,
            450.0,
            TextParams {
                font_size: 50,
                font_scale: get_time().sin() as f32 / 2.0 + 1.0,
                ..Default::default()
            },
        );
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

        draw_circle_lines(50.0, 50f32, 30f32, 0.5f32, GREEN);
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
