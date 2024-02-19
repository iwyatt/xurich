// TODO: This use / mod loading needs to be reviewed and cleaned up
mod components;
mod gui;
mod map;
mod npc;
mod systems;
mod prelude {
    // game play area max width and height
    // set random seed
    pub const RNG_SEED: u64 = 0;
    pub const MAP_WIDTH: i32 = 80;
    pub const MAP_HEIGHT: i32 = 50;
    pub use crate::components::*;
    pub use crate::map::*;
    pub use crate::npc::*;
    pub use crate::systems::viewsheds;
    pub use bevy::prelude::*;
    pub use bevy_ascii_terminal::prelude::*;
    pub use rltk::*;
}
use crate::systems::combat::resolve_combat_events;
use crate::systems::npc_ai::run_npc_ai;
use crate::systems::player_input::player_walk;
use crate::systems::rendering::*;
use crate::viewsheds::get_visible_tiles;
use crate::viewsheds::update_viewsheds;
use prelude::*;
use systems::spawner;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, TerminalPlugin))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                MapIndexingSystem::run,
                player_walk,
                get_visible_tiles,
                update_viewsheds,
                run_npc_ai,
                resolve_combat_events,
                get_visible_tiles,
                update_viewsheds,
                tick,
            )
                .chain(),
        )
        .add_event::<CombatAttack>()
        //.add_systems(Update, tick)
        .run();
}

// set up loop
fn setup(mut commands: Commands) {
    let mut myrng = RNG(RandomNumberGenerator::seeded(RNG_SEED)); // TODO: Change this to be a u64 hash of player name/year
                                                                  //commands.spawn(myrng);

    // set the game state
    let game_state = components::GameState {
        runstate: RunState::Running,
    };
    commands.spawn(game_state);

    // define the terminal
    let term_size = [MAP_WIDTH, MAP_HEIGHT + 2]; // +2 for 2 lines of UI. Note this is 1-index, not 0-index unlike term.put_char
    let terminal = Terminal::new(term_size).with_border(Border::single_line());
    let term_bundle = TerminalBundle::from(terminal);

    // create the terminal and camera
    commands
        .spawn((term_bundle, AutoCamera))
        .insert(GameTerminal);

    //let map = Map::default();
    //let nmap = MapGenerator::new();
    let (map, player_start_position, mob_start_positions) = Map::random();
    //let (map, player_start_position) = Map::new_map_cellularautomata(MapGenerator::new());

    // TODO: move player and npc spawn into map generation
    // spawn player on map
    spawner::spawn_player(&mut commands, player_start_position);

    // spawn npc bundle
    mob_start_positions
        .iter()
        .for_each(|pos| spawner::spawn_random_mob(&mut commands, pos.clone(), &mut myrng));

    // for i in 1..map.rooms.len() {
    //     let position = Position {
    //         x: map.rooms[i].center().x,
    //         y: map.rooms[i].center().y,
    //     };
    //     spawner::spawn_random_mob(&mut commands, position, &mut myrng);
    // }
    commands.spawn(map);
    commands.spawn(myrng);
}
