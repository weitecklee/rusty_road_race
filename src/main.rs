use rand::{distr::Uniform, prelude::*};
use rusty_engine::prelude::*;

#[derive(Resource)]
struct GameState {
    health_amount: u8,
    lost: bool,
    y_values: Uniform<f32>,
}

const PLAYER_SPEED: f32 = 250.0;
const ROAD_SPEED: f32 = 400.0;

fn main() {
    let mut game = Game::new();
    let mut rng = rand::rng();
    let x_values = Uniform::new(-640.0, 640.0).unwrap();
    let y_values = Uniform::new(-360.0, 360.0).unwrap();

    game.window_settings(Window {
        title: "Rusty Road Race".to_string(),
        ..Default::default()
    });

    game.audio_manager
        .play_music(MusicPreset::MysteriousMagic, 0.15);

    let player1 = game.add_sprite("player1", "sprite/spacerage/player_b_m.png");
    player1.translation.x = -500.0;
    player1.rotation = SOUTH;
    player1.layer = 10.0;
    player1.collision = true;

    for i in 0..rng.random_range(8..=17) {
        let label = format!("roadline{}", i);
        let roadline = game.add_sprite(label, SpritePreset::RacingBarrierWhite);
        roadline.scale = 0.1;
        roadline.translation.x = rng.sample(x_values);
        roadline.translation.y = rng.sample(y_values);
    }

    game.add_logic(game_logic);
    game.run(GameState {
        health_amount: 5,
        lost: false,
        y_values,
    });
}

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    let mut rng = rand::rng();

    let mut direction = 0.0_f32;

    if engine
        .keyboard_state
        .pressed_any(&[KeyCode::Up, KeyCode::W])
    {
        direction += 1.0;
    }
    if engine
        .keyboard_state
        .pressed_any(&[KeyCode::Down, KeyCode::S])
    {
        direction -= 1.0;
    }
    // if engine
    //     .keyboard_state
    //     .pressed_any(&[KeyCode::Right, KeyCode::D])
    // {
    //     player.translation.x += PLAYER_SPEED * engine.delta_f32;
    // }
    // if engine
    //     .keyboard_state
    //     .pressed_any(&[KeyCode::Left, KeyCode::A])
    // {
    //     player.translation.x -= PLAYER_SPEED * engine.delta_f32;
    // }

    let player1 = engine.sprites.get_mut("player1").unwrap();
    player1.translation.y += direction * PLAYER_SPEED * engine.delta_f32;
    player1.rotation = direction * 0.15 + SOUTH;
    if player1.translation.y > 360.0 || player1.translation.y < -360.0 {
        game_state.health_amount = 0;
    }

    for sprite in engine.sprites.values_mut() {
        if sprite.label.starts_with("roadline") {
            sprite.translation.x -= ROAD_SPEED * engine.delta_f32;
            if sprite.translation.x < -675.0 {
                sprite.translation.x += 1500.0;
                sprite.translation.y = rng.sample(game_state.y_values);
            }
        }
    }
}
