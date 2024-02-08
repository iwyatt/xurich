// TODO: This use / mod loading needs to be reviewed and cleaned up
mod components;
mod map;
mod rect;
mod systems;
mod prelude {
    // game play area max width and height
    pub const MAP_WIDTH: i32 = 80;
    pub const MAP_HEIGHT: i32 = 50;
    pub use crate::components::*;
    pub use crate::map::*;
    pub use crate::systems::entity_viewsheds;
    pub use crate::systems::npc_spawner::*;
    pub use crate::systems::*;
    pub use bevy::prelude::*;
    pub use bevy_ascii_terminal::prelude::*;
    pub use rltk::*;
}
use crate::entity_viewsheds::get_visible_tiles;
use crate::entity_viewsheds::update_viewsheds;
use crate::systems::npc_ai::run_npc_ai;
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
    commands.spawn(map.clone());

    // spawn player in center of first room on map
    commands
        .spawn((
            Position {
                x: map.rooms[0].center().x,
                y: map.rooms[0].center().y,
            },
            Renderable {
                glyph: '@',
                fg: Color::YELLOW,
                bg: Color::BLACK,
            },
        ))
        //.insert(BlocksTile)
        .insert(Player)
        .insert(components::Name {
            name: String::from("Player"),
        })
        .insert(CombatStats {
            max_hp: 30,
            hp: 30,
            defense: 2,
            power: 5,
        })
        .insert(Viewshed {
            visible_tiles: Vec::new(),
            range: 5,
            dirty: true,
        });

    // spawn npcs
    for i in 1..map.rooms.len() {
        let mut rng = rltk::RandomNumberGenerator::new();
        let roll = rng.roll_dice(1, 2);
        let (glyph, name) = match roll {
            1 => ('G', "Goblin".to_string()),
            2 => ('O', "Orc".to_string()),
            _ => ('X', "Xenity".to_string()),
        };

        commands
            .spawn((
                Position {
                    x: map.rooms[i].center().x,
                    y: map.rooms[i].center().y,
                },
                Renderable {
                    glyph: glyph,
                    fg: Color::RED,
                    bg: Color::BLACK,
                },
                components::Name { name: name },
                LeftWalker,
                NPC_AI {
                    state: NPC_State::Inactive,
                },
            ))
            .insert(BlocksTile)
            .insert(Enemy)
            .insert(Viewshed {
                visible_tiles: Vec::new(),
                range: 4,
                dirty: true,
            });
    }
}

// render update
fn tick(
    mut query_terminal: Query<&mut Terminal>,
    query_entities: Query<(&Position, &Renderable)>,
    query_maps: Query<&Map>,
    mut query_player_viewshed: Query<&mut Viewshed, With<Player>>,
    // mut query_game_state: Query<&mut components::GameState>,
) {
    let map = query_maps.iter().nth(0).unwrap();
    // let mut game_state = query_game_state.iter_mut().nth(0).unwrap();
    let mut terminal = query_terminal.iter_mut().nth(0).unwrap();
    let mut viewshed = query_player_viewshed.iter_mut().nth(0).unwrap();

    // stop rendering if the player's view shed isn't dirty.
    // if !viewshed.dirty {
    //     return;
    // } else {
    //     terminal.clear();
    //     viewshed.dirty = false;
    // };

    // declare the viewshed clean - then update it.
    viewshed.dirty = false;
    let visible_tiles = &viewshed.visible_tiles;

    // clear the terminal screen
    terminal.clear();

    map.tiles.iter().for_each(|tile| {
        // render revealed tiles
        let idx = xy_idx(tile.location.x, tile.location.y);
        if map.revealed_tiles[idx] {
            let tilefg = Color::Rgba {
                red: tile.render.fg.r() / 2.0,
                green: tile.render.fg.g() / 2.0,
                blue: tile.render.fg.b() / 2.0,
                alpha: 0.1,
            };

            terminal.put_char(
                [tile.location.x, tile.location.y],
                tile.render.glyph.fg(tilefg).bg(tile.render.bg),
            );
        }

        // render currently visible map tiles
        // TODO: change this so that it only re-renders this when something changes (player moves, monster moves, etc)
        if visible_tiles.contains(&Point::new(tile.location.x, tile.location.y)) {
            terminal.put_char(
                [tile.location.x, tile.location.y],
                tile.render.glyph.fg(tile.render.fg).bg(tile.render.bg),
            );
        }
    });

    //render npcs
    query_entities.iter().for_each(|(pos, rend)| {
        if visible_tiles.contains(&Point::new(pos.x, pos.y)) {
            terminal.put_char([pos.x, pos.y], rend.glyph.fg(rend.fg).bg(rend.bg))
        }
    });
}

fn player_walk(
    input: Res<Input<KeyCode>>,
    mut player_pos: Query<(&Player, &mut Position)>,
    query_map: Query<&Map>,
    mut query_viewshed: Query<&mut Viewshed>,
    mut query_game_state: Query<&mut components::GameState>,
) {
    let map = query_map.iter().nth(0).unwrap();
    let move_input = read_movement(input);
    if move_input.cmpeq(IVec2::ZERO).all() {
        return;
    }

    let (player, mut pos) = player_pos
        .iter_mut()
        .nth(0)
        .map(|(player, mut pos)| (player, pos))
        .unwrap();

    let curr = IVec2::new(pos.x, pos.y);
    let next = curr + move_input;

    // check if player can validly move to desired spot
    if map.blocked_tiles[xy_idx(next.x, next.y)] {
        return;
    };

    // if so, then update player position
    pos.x = next.x;
    pos.y = next.y;

    let mut viewshed = query_viewshed.iter_mut().nth(0).unwrap();
    viewshed.dirty = true;
    let mut game_state = query_game_state.iter_mut().nth(0).unwrap();
    game_state.runstate = RunState::Running;
}

// an IVec2 is a 2-dimensional vector (direction and distance for x and y both)
fn read_movement(input: Res<Input<KeyCode>>) -> IVec2 {
    let mut p = IVec2::ZERO;

    if input.just_pressed(KeyCode::Numpad1) || input.just_pressed(KeyCode::Z) {
        p.x = -1;
        p.y = -1;
    }
    if input.just_pressed(KeyCode::Numpad2)
        || input.just_pressed(KeyCode::X)
        || input.just_pressed(KeyCode::Down)
    {
        p.y = -1;
    }
    if input.just_pressed(KeyCode::Numpad3) || input.just_pressed(KeyCode::C) {
        p.x = 1;
        p.y = -1;
    }
    if input.just_pressed(KeyCode::Numpad4)
        || input.just_pressed(KeyCode::A)
        || input.just_pressed(KeyCode::Left)
    {
        p.x = -1;
    }
    if input.just_pressed(KeyCode::Numpad6)
        || input.just_pressed(KeyCode::D)
        || input.just_pressed(KeyCode::Right)
    {
        p.x = 1;
    }
    if input.just_pressed(KeyCode::Numpad7) || input.just_pressed(KeyCode::Q) {
        p.x = -1;
        p.y = 1;
    }
    if input.just_pressed(KeyCode::Numpad8)
        || input.just_pressed(KeyCode::W)
        || input.just_pressed(KeyCode::Up)
    {
        p.y = 1;
    }
    if input.just_pressed(KeyCode::Numpad9) || input.just_pressed(KeyCode::E) {
        p.x = 1;
        p.y = 1;
    }

    p
}
