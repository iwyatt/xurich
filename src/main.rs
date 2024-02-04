mod components;
mod map;
mod rect;
mod systems;
mod prelude {
    pub use crate::components::*;
    pub use crate::map::*;
    pub use crate::rect::*;
    pub use bevy::prelude::*;
    pub use bevy_ascii_terminal::prelude::*;
    pub use rltk::*;
    pub const MAP_WIDTH: i32 = 80;
    pub const MAP_HEIGHT: i32 = 50;
    pub use crate::systems::npc_spawner::*;
}
use prelude::*;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, TerminalPlugin))
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (player_walk, get_visible_tiles, update_viewsheds).chain(),
        )
        .add_systems(Update, (run_npc_ai, tick).chain())
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
    let mut terminal = Terminal::new([MAP_WIDTH, MAP_HEIGHT]).with_border(Border::single_line());
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
        .insert(Player)
        .insert(Viewshed {
            visible_tiles: Vec::new(),
            range: 3,
            dirty: true,
        });

    // spawn npcs
    for i in 1..map.rooms.len() {
        let mut rng = rltk::RandomNumberGenerator::new();
        let roll = rng.roll_dice(1, 2);
        let glyph = match roll {
            1 => 'G',
            2 => 'O',
            _ => 'X',
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
                LeftWalker,
            ))
            .insert(Enemy)
            .insert(Viewshed {
                visible_tiles: Vec::new(),
                range: 2,
                dirty: true,
            })
            .insert(NPC_AI);
    }
}

// render update
fn tick(
    mut query_terminal: Query<&mut Terminal>,
    query_entities: Query<(&Position, &Renderable)>,
    query_maps: Query<&Map>,
    mut query_player_viewshed: Query<&mut Viewshed, With<Player>>,
    mut query_game_state: Query<&mut components::GameState>,
) {
    let mut game_state = query_game_state.iter_mut().nth(0).unwrap();
    if game_state.runstate == RunState::Paused {
        return;
    };

    // may need to add `With<GameTerminal>>`
    // https://github.com/sarkahn/bevy_roguelike/blob/2027f9966fab33e6e303a7b88b3d1e30c56683b0/src/render.rs
    // See line 44: mut q_render_terminal: Query<&mut Terminal, With<GameTerminal>>,
    let mut terminal = query_terminal.iter_mut().nth(0).unwrap();

    //render map
    let mut viewshed = query_player_viewshed.iter_mut().nth(0).unwrap();
    if !viewshed.dirty {
        return;
    } else {
        terminal.clear();
        viewshed.dirty = false;
    };
    let visible_tiles = &viewshed.visible_tiles;
    let map = query_maps.iter().nth(0).unwrap();

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
        // TODO: change this so that it only re-renders this when the players moves
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
    game_state.runstate = RunState::Paused;
}

fn npc_walk(mut query_walkers: Query<(&mut Position, &Enemy)>) {
    query_walkers.iter_mut().for_each(|(mut p, _)| {
        p.x = p.x + 1;
        if p.x <= 1 {
            p.x = 1;
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
    if map.tiles[xy_idx(next.x, next.y)].tile == TileType::Wall {
        return;
    };
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
