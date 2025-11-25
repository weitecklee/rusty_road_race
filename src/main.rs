use rand::{distr::Uniform, prelude::*};
use rusty_engine::prelude::*;

#[derive(Resource)]
struct GameState {
    health_amount: u8,
    lost: bool,
    x_values: Uniform<f32>,
    y_values: Uniform<f32>,
}

const PLAYER_SPEED: f32 = 250.0;
const ROAD_SPEED: f32 = 400.0;
const OBSTACLE_SPEED: f32 = 600.0;

fn main() {
    let mut game = Game::new();
    let mut rng = rand::rng();
    let x_values = Uniform::new(800.0, 1600.0).unwrap();
    let y_values = Uniform::new(-360.0, 360.0).unwrap();

    game.window_settings(Window {
        title: "Rusty Road Race".to_string(),
        ..Default::default()
    });

    game.audio_manager
        .play_music(MusicPreset::MysteriousMagic, 0.15);

    let health_text = game.add_text("health_text", "Health: 5");
    health_text.translation = Vec2::new(550.0, 320.0);

    let player1 = game.add_sprite("player1", "sprite/spacerage/player_b_m.png");
    player1.translation.x = -500.0;
    player1.rotation = SOUTH;
    player1.layer = 10.0;
    player1.collision = true;

    for i in 0..rng.random_range(8..=17) {
        let roadline = game.add_sprite(format!("roadline{}", i), SpritePreset::RacingBarrierWhite);
        roadline.scale = 0.1;
        roadline.translation.x = rng.random_range(-640.0..=640.0);
        roadline.translation.y = rng.sample(y_values);
    }

    let obstacle_presets = [
        SpritePreset::RacingBarrelBlue,
        SpritePreset::RacingBarrelRed,
        SpritePreset::RollingBallBlue,
        SpritePreset::RollingBallRed,
        SpritePreset::RacingConeStraight,
        SpritePreset::RollingBallRedAlt,
        SpritePreset::RollingBallBlueAlt,
        SpritePreset::RollingBlockSquare,
    ];

    for (i, preset) in obstacle_presets.into_iter().enumerate() {
        let obstacle = game.add_sprite(format!("obstacle{}", i), preset);
        obstacle.layer = 5.0;
        obstacle.collision = true;
        obstacle.translation.x = rng.sample(x_values);
        obstacle.translation.y = rng.sample(y_values);
    }

    game.add_logic(game_logic);
    game.run(GameState {
        health_amount: 5,
        lost: false,
        x_values,
        y_values,
    });
}

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
    if game_state.lost {
        return;
    }

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
                sprite.translation.x = rng.sample(game_state.x_values);
                sprite.translation.y = rng.sample(game_state.y_values);
            }
        } else if sprite.label.starts_with("obstacle") {
            sprite.translation.x -= OBSTACLE_SPEED * engine.delta_f32;
            if sprite.translation.x < -800.0 {
                sprite.translation.x = rng.sample(game_state.x_values);
                sprite.translation.y = rng.sample(game_state.y_values);
            }
        }
    }

    let health_text = engine.texts.get_mut("health_text").unwrap();
    for event in engine.collision_events.drain(..) {
        if !event.pair.either_contains("player") || event.state.is_end() {
            continue;
        }
        if game_state.health_amount > 0 {
            game_state.health_amount -= 1;
            health_text.value = format!("Health: {}", game_state.health_amount);
            engine.audio_manager.play_sfx(SfxPreset::Impact3, 0.5);
        }
        if game_state.health_amount == 0 {
            break;
        }
    }

    if game_state.health_amount == 0 {
        game_state.lost = true;
        let game_over_text = engine.add_text("game_over", "GAME OVER");
        game_over_text.font_size = 128.0;
        engine.audio_manager.stop_music();
        engine.audio_manager.play_sfx(SfxPreset::Jingle3, 0.5);
    }
}
