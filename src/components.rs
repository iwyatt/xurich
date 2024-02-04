pub use crate::prelude::*;

#[derive(Component)]
pub struct GameTerminal;

#[derive(Component)]
pub struct Player;

#[derive(Component, Debug)]
pub struct Enemy;

#[derive(Component, Debug)]
pub struct NPC_AI;

// impl NPC_AI {}
pub fn run_npc_ai(
    query_enemy: Query<(&Position, &Viewshed, With<Enemy>)>,
    query_player: Query<&Position, With<Player>>,
    mut query_terminal: Query<&mut Terminal>,
) {
    let mut terminal = query_terminal.iter_mut().nth(0).unwrap();
    let player_position = query_player.iter().nth(0).unwrap();
    query_enemy.iter().for_each(|e| {
        if e.1
            .visible_tiles
            .contains(&Point::new(player_position.x, player_position.y))
        {
            terminal.put_string([e.0.x, e.0.y], "Hello, World!".fg(Color::BLUE))
        }
    });
}

#[derive(Component)]
pub struct LeftWalker;

#[derive(Component, PartialEq, Clone)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component, PartialEq, Clone)]
pub struct Renderable {
    pub glyph: char,
    pub fg: Color,
    pub bg: Color,
}

#[derive(Component)] // TODO : Implement viewshed so NPCs can us it
pub struct Viewshed {
    pub visible_tiles: Vec<rltk::Point>,
    pub range: i32,
    pub dirty: bool,
}

//impl Viewshed {}
pub fn get_visible_tiles(
    mut query_player_pos: Query<(&Position, &mut Viewshed), With<Player>>,
    mut query_map: Query<&mut Map>,
) {
    let (position, mut viewshed) = query_player_pos.iter_mut().nth(0).unwrap();
    let mut map = query_map.iter_mut().nth(0).unwrap();
    let mut visible_tiles =
        field_of_view(Point::new(position.x, position.y), viewshed.range, &*map);
    visible_tiles.retain(|p| p.x >= 0 && p.x < map.width && p.y >= 0 && p.y < map.height);
    viewshed.visible_tiles = visible_tiles;
    viewshed
        .visible_tiles
        .iter()
        .for_each(|position| map.revealed_tiles[xy_idx(position.x, position.y)] = true);
}

pub fn update_viewsheds(
    mut query_viewsheds: Query<(&Position, &mut Viewshed)>,
    mut query_map: Query<&mut Map>,
) {
    let mut map = query_map.iter_mut().nth(0).unwrap();
    query_viewsheds.iter_mut().for_each(|(p, mut v)| {
        let mut visible_tiles = field_of_view(Point::new(p.x, p.y), v.range, &*map);
        visible_tiles.retain(|p| p.x >= 0 && p.x < map.width && p.y >= 0 && p.y < map.height);
        v.visible_tiles = visible_tiles;
    });
}

#[derive(Component)]
pub struct GameState {
    //pub ecs: World,
    pub runstate: RunState,
}

#[derive(PartialEq, Copy, Clone)]
pub enum RunState {
    Paused,
    Running,
}
