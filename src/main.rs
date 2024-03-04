// TODO: This use / mod loading needs to be reviewed and cleaned up
mod components;
mod events;
mod map;
mod npc;
mod systems;
mod prelude {
    // game play area max width and height
    // set random seed
    pub const RNG_SEED: u64 = 0;
    pub const MAP_WIDTH: i32 = 80;
    pub const MAP_HEIGHT: i32 = 50;
    pub const WORLD_MAP_WIDTH: i32 = 3;
    pub const WORLD_MAP_HEIGHT: i32 = 3;
    pub use crate::components::*;
    pub use crate::map::*;
    pub use crate::npc::*;
    pub use crate::systems::viewsheds;
    pub use bevy::prelude::*;
    pub use bevy_ascii_terminal::prelude::*;
    //use rltk::*;
}
use crate::events::combat::resolve_combat_events;
use crate::events::inventory::ev_pickup_item;
use crate::events::inventory::ev_use_item;
use rltk::*;

use crate::systems::npc_ai::run_npc_ai;
use crate::systems::player_input::player_get_item;
use crate::systems::player_input::player_use_item;
use crate::systems::player_input::player_wait;
use crate::systems::player_input::player_walk;
use crate::systems::rendering::*;
use systems::spawner;

use crate::viewsheds::get_visible_tiles;
use crate::viewsheds::update_viewsheds;

use prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, TerminalPlugin))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                MapIndexingSystem::run,
                player_get_item,
                player_use_item,
                ev_pickup_item,
                ev_use_item,
                player_walk,
                player_wait,
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
        .add_event::<EV_ItemPickUp>()
        .add_event::<EV_ItemUse>()
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

    // define the play terminal
    let term_size = [MAP_WIDTH, MAP_HEIGHT + 2]; // +2 for 2 lines of UI. Note this is 1-index, not 0-index unlike term.put_char
                                                 // TODO: BUG: I suspect that the above is causing an issue with NPC_AI.rs pathing when player is on bottom row of map
    let terminal = Terminal::new(term_size).with_border(Border::single_line());
    let term_bundle = TerminalBundle::from(terminal);

    // create the terminal and camera
    commands
        .spawn((term_bundle, AutoCamera))
        .insert(GameTerminal);

    //let (map, player_start_position, mob_start_positions, item_start_positions) = Map::random();
    let mapgen = MapGenerator::default();
    let (map, player_start_position, mob_start_positions, item_start_positions) = Map::random();
    // TODO: move player and npc spawn into map generation
    // spawn player on map
    spawner::spawn_player(&mut commands, player_start_position);

    // spawn npc bundle
    mob_start_positions
        .iter()
        .for_each(|pos| spawner::spawn_random_mob(&mut commands, pos.clone(), &mut myrng));

    // spawn item bundle
    item_start_positions
        .iter()
        .for_each(|pos| spawner::spawn_random_item(&mut commands, pos.clone(), &mut myrng));

    // add map to worldmap resource
    let mut worldmap = WorldMap {
        maps: Vec::<Map>::with_capacity(WORLD_MAP_HEIGHT as usize * WORLD_MAP_WIDTH as usize),
    };
    //let mapidx = world_xy_idx(map.world_pos.x, map.world_pos.y);
    worldmap.maps.push(map);
    commands.insert_resource(worldmap);
    //commands.spawn(map);
    commands.spawn(myrng);
}
