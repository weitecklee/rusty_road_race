use rusty_engine::prelude::*;

#[derive(Resource)]
struct GameState {
    health_amount: u8,
    lost: bool,
}

const PLAYER_SPEED: f32 = 250.0;

fn main() {
    let mut game = Game::new();

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

    game.add_logic(game_logic);
    game.run(GameState {
        health_amount: 5,
        lost: false,
    });
}

fn game_logic(engine: &mut Engine, game_state: &mut GameState) {
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
}
