use rand::{distr::Uniform, prelude::*};
use rusty_engine::prelude::{bevy::utils::HashMap, *};

#[derive(Resource)]
struct GameState {
    health_amount: u8,
    lost: bool,
    x_values: Uniform<f32>,
    y_values: Uniform<f32>,
    speed_values: Uniform<f32>,
    obstacle_speeds: HashMap<String, f32>,
    spawn_timer: Timer,
}

const PLAYER_SPEED: f32 = 400.0;
const ROAD_SPEED: f32 = 400.0;
const OBSTACLE_PRESETS: [SpritePreset; 8] = [
    SpritePreset::RacingBarrelBlue,
    SpritePreset::RacingBarrelRed,
    SpritePreset::RollingBallBlue,
    SpritePreset::RollingBallRed,
    SpritePreset::RacingConeStraight,
    SpritePreset::RollingBallRedAlt,
    SpritePreset::RollingBallBlueAlt,
    SpritePreset::RollingBlockSquare,
];

fn main() {
    let mut game = Game::new();
    let mut rng = rand::rng();
    let x_values = Uniform::new(800.0, 1600.0).unwrap();
    let y_values = Uniform::new(-360.0, 360.0).unwrap();
    let speed_values = Uniform::new(300.0, 1500.0).unwrap();
    let mut obstacle_speeds: HashMap<String, f32> = HashMap::new();
    let spawn_timer = Timer::from_seconds(3.0, TimerMode::Repeating);

    game.window_settings(Window {
        title: "Rusty Road Race".to_string(),
        ..Default::default()
    });

    game.audio_manager
        .play_music(MusicPreset::MysteriousMagic, 0.15);

    let health_text = game.add_text("health_text", "Health: 5");
    health_text.translation = Vec2::new(550.0, 320.0);

    // let debug_text = game.add_text("debug_text", "Obstacles: 8");
    // debug_text.font_size = 24.0;
    // debug_text.translation = Vec2::new(550.0, -320.0);

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

    for i in 0..8 {
        let label = format!("obstacle{}", i);
        let obstacle = game.add_sprite(
            &label,
            OBSTACLE_PRESETS[rng.random_range(0..OBSTACLE_PRESETS.len())],
        );
        obstacle.layer = 5.0;
        obstacle.collision = true;
        obstacle.translation.x = rng.sample(x_values);
        obstacle.translation.y = rng.sample(y_values);

        obstacle_speeds.insert(label, rng.sample(speed_values));
    }

    game.add_logic(game_logic);
    game.run(GameState {
        health_amount: 5,
        lost: false,
        x_values,
        y_values,
        speed_values,
        obstacle_speeds,
        spawn_timer,
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
            let obstacle_speed = game_state.obstacle_speeds.get(&sprite.label).unwrap();
            sprite.translation.x -= obstacle_speed * engine.delta_f32;
            if sprite.translation.x < -800.0 {
                sprite.translation.x = rng.sample(game_state.x_values);
                sprite.translation.y = rng.sample(game_state.y_values);
                game_state
                    .obstacle_speeds
                    .insert(sprite.label.clone(), rng.sample(game_state.speed_values));
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

    if game_state.spawn_timer.tick(engine.delta).just_finished() {
        let i = game_state.obstacle_speeds.len();
        let label = format!("obstacle{}", i);
        let obstacle = engine.add_sprite(
            &label,
            OBSTACLE_PRESETS[rng.random_range(0..OBSTACLE_PRESETS.len())],
        );
        obstacle.layer = 5.0;
        obstacle.collision = true;
        obstacle.translation.x = rng.sample(game_state.x_values);
        obstacle.translation.y = rng.sample(game_state.y_values);

        game_state
            .obstacle_speeds
            .insert(label, rng.sample(game_state.speed_values));

        // let debug_text = engine.texts.get_mut("debug_text").unwrap();
        // debug_text.value = format!("Obstacles: {}", i + 1);
    }
}
