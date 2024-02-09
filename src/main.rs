// TODO: This use / mod loading needs to be reviewed and cleaned up
mod components;
mod map;
mod npc;
mod systems;
mod prelude {
    // game play area max width and height
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
use crate::systems::npc_ai::run_npc_ai;
use crate::systems::player_input::player_walk;
use crate::systems::rendering::*;
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
                player_walk,
                get_visible_tiles,
                update_viewsheds,
                run_npc_ai,
                get_visible_tiles,
                update_viewsheds,
                tick,
            )
                .chain(),
        )
        //.add_systems(Update, tick)
        .run();
}

// set up loop
fn setup(mut commands: Commands) {
    // set the game state
    let game_state = components::GameState {
        runstate: RunState::Running,
    };
    commands.spawn(game_state);

    // // Create the terminal
    let terminal = Terminal::new([MAP_WIDTH, MAP_HEIGHT]).with_border(Border::single_line());
    let term_bundle = TerminalBundle::from(terminal);
    commands
        .spawn((term_bundle, AutoCamera))
        .insert(GameTerminal);

    //let map = Map::new();
    let map = Map::new_map_rooms_and_corridors();
    commands.spawn(map.clone()); //TODO: why do I need this clone() ?

    // TODO: move player and npc spawn into map generation

    // spawn player bundle in center of first room on map
    commands.spawn(PlayerBundle {
        position: Position {
            x: map.rooms[0].center().x,
            y: map.rooms[0].center().y,
        },
        ..Default::default()
    });

    // spawn npc bundle
    for i in 1..map.rooms.len() {
        let mut rng = rltk::RandomNumberGenerator::new();
        let roll = rng.roll_dice(1, 2);
        let (glyph, name) = match roll {
            1 => ('G', "Goblin".to_string()),
            2 => ('O', "Orc".to_string()),
            _ => ('X', "Xenity".to_string()),
        };

        commands
            .spawn(NPCBundle {
                name: Name(name.into()),
                position: Position {
                    x: map.rooms[i].center().x,
                    y: map.rooms[i].center().y,
                },
                renderable: Renderable {
                    glyph: glyph,
                    fg: Color::YELLOW,
                    bg: Color::BLACK,
                },
                ..Default::default()
            })
            .insert(BlocksTile)
            .insert(Enemy);
    }
}
