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
use crate::events::inventory::ev_close_inventory;
use crate::events::inventory::ev_drop_item;
use crate::events::inventory::ev_open_inventory;
use crate::events::inventory::ev_pickup_item;
use crate::events::inventory::ev_unequip_item;
use crate::events::inventory::ev_use_item;
use crate::systems::npc_ai::run_npc_ai;
use crate::systems::player_input::inventory_cursor;
use crate::systems::player_input::player_get_item;
use crate::systems::player_input::player_inventory_screen;
use crate::systems::player_input::player_use_item;
use crate::systems::player_input::player_wait;
use crate::systems::player_input::player_walk;
use crate::systems::rendering::*;
use rltk::*;
use systems::gameover;
use systems::gameover::*;
use systems::newgame::*;
use systems::rendering;
use systems::spawner;

use crate::viewsheds::get_visible_tiles;
use crate::viewsheds::update_viewsheds;

use prelude::*;

fn main() {
    #[rustfmt::skip]
    let mut app = App::new()
        // add plugins
        .add_plugins((DefaultPlugins, TerminalPlugin))
        // initialize resources
        //.init_resource::<State<GameLoopState>>()
        // add state
        .add_state::<GameLoopState>()
        // configure Startup systems
        .add_systems(Startup, setup)
        .add_systems(Update, init_new_game.run_if(in_state(GameLoopState::NewGame)))
        // configure Update systems
        .add_systems(
            Update,
            (
                // Systems running on Player's Turn
                (
                    MapIndexingSystem::run,
                    player_inventory_screen,
                    player_get_item,
                    player_use_item,
                    ev_pickup_item,
                    ev_use_item,
                    ev_open_inventory,
                    player_walk,
                    player_wait,
                    get_visible_tiles,
                    update_viewsheds,
                    resolve_combat_events,
                    tick,
                    render_statbar,
                )
                    .run_if(in_state(GameLoopState::PlayerTurn)),
                
                // TODO: Inventory Screen, Input, Events
                // Systems running on Player Inventory Screen
                (
                 rendering::inventory,
                 inventory_cursor,
                 ev_close_inventory,
                 systems::player_input::inventory_use,
                 systems::player_input::inventory_drop,
                 ev_use_item,
                 ev_drop_item,
                 ev_unequip_item,
                 render_statbar,
                 //ev_use_item,
                 //ev_equip_item,
                 //ev_drop_item,
                 player_inventory_screen,
                )
                    .run_if(in_state(GameLoopState::Inventory)),

                // Systems Running on NPC's Turn
                (
                    run_npc_ai,
                    update_viewsheds,
                    resolve_combat_events,
                    tick,
                    render_statbar,
                )
                    .run_if(in_state(GameLoopState::NPCTurn)),
                
                // Systems Running on Player Death
                (
                    //render_statbar,
                    gameover::player_input,
                    gameover::render_game_over,
                    render_statbar,
                )
                    .run_if(in_state(GameLoopState::Defeat)),
            ),
        )
        // register event processing
        .add_event::<CombatAttack>()
        .add_event::<EV_ItemPickUp>()
        .add_event::<EV_ItemUse>()
        .add_event::<EV_ItemDrop>()
        .add_event::<EV_ItemUnequip>()
        .add_event::<EV_OpenInventoryTerminal>()
        .add_event::<EV_CloseInventoryTerminal>()
        // run the app!
        .run();
}

// set up loop
fn setup(
    mut commands: Commands,
    gamestate: Res<State<GameLoopState>>,
    mut next_state: ResMut<NextState<GameLoopState>>,
    entities: Query<Entity>,
) {
    // define the play terminal
    let term_size = [MAP_WIDTH, MAP_HEIGHT];
    // TODO: BUG: I suspect that the above is causing an issue with NPC_AI.rs pathing when player is on bottom row of map

    let terminal = Terminal::new(term_size).with_border(Border::single_line());
    let term_bundle = TerminalBundle::from(terminal);

    // create the terminal and camera
    commands
        .spawn((term_bundle, AutoCamera, InitialEntity))
        .insert(InitialEntity)
        .insert(MapTerminal);

    // stat bar
    let mut term_statbar = Terminal::new([MAP_WIDTH, 1]).with_border(Border::single_line());
    //term_statbar.put_string([0, 0], "Hello!");
    commands
        .spawn((
            TerminalBundle::from(term_statbar).with_position([0, MAP_HEIGHT / 2 + 4]),
            AutoCamera,
            InitialEntity,
        ))
        .insert(StatBarTerminal)
        .insert(InitialEntity);

    // add all these entities into set of initial entities that we do not want cleared on New Game
    for entity in entities.iter() {
        commands.entity(entity).insert(InitialEntity);
    }
    next_state.set(GameLoopState::NewGame);
}
