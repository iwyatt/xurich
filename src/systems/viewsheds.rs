use crate::prelude::*;

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
