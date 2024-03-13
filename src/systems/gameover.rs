use crate::prelude::*;
//use bevy::window::PrimaryWindow;
use bevy_ascii_terminal::*;
//use rltk::*;
use Terminal;
// use super::player_input::GameState;
// use rltk::RandomNumberGenerator;
// use crate::components;

pub fn player_input(
    input: Res<Input<KeyCode>>,
    mut commands: Commands,
    mut query_entities: Query<Entity, (Without<InitialEntity>, Without<Parent>)>,
    gamestate: Res<State<GameLoopState>>,
    mut next_state: ResMut<NextState<GameLoopState>>,
    //mut world: &mut World,
) {
    if input.just_pressed(KeyCode::N) {
        //println!("new game key pressed");

        // clear entities
        for entity in query_entities.iter_mut() {
            //println!("removing entities: {:#?}", entity);
            commands.entity(entity).despawn_recursive();
            //world.clear_entities();
        }
        next_state.set(GameLoopState::NewGame);
    }
}

pub fn render_game_over(mut query_terminal: Query<&mut Terminal, With<MapTerminal>>) {
    let mut terminal = query_terminal.single_mut();
    //terminal.clear();
    terminal.put_string(
        //[MAP_WIDTH / 2, MAP_HEIGHT / 2],
        [0, 0].pivot(Pivot::Center),
        "YOUR QUEST HAS ENDED".fg(Color::RED).bg(Color::BLACK),
    );

    terminal.put_string(
        //[MAP_WIDTH / 2, MAP_HEIGHT / 2],
        [0, -2].pivot(Pivot::Center),
        "PRESS 'N' to start a new game"
            .fg(Color::WHITE)
            .bg(Color::BLACK),
    );
}
