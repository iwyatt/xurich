pub use crate::prelude::*;

#[derive(Component)]
pub struct GameTerminal;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Enemy;

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

#[derive(Component)]
pub struct Viewshed {
    pub visible_tiles: Vec<rltk::Point>,
    pub range: i32,
}

impl Viewshed {}
pub fn get_visible_tiles(
    mut query_player_pos: Query<(&Player, &Position, &mut Viewshed)>,
    mut query_map: Query<&mut Map>,
) {
    let (_, position, mut viewshed) = query_player_pos.iter_mut().nth(0).unwrap();
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
