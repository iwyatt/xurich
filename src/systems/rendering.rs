use crate::components;
pub use crate::prelude::*;

// render update
pub fn tick(
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

    // render ui
    // let mut ui = query_terminal.iter_mut().nth(1).unwrap();
    // ui.put_string([0, 0], "Hello".fg(Color::WHITE));
    terminal.put_string([0, MAP_HEIGHT + 1], "Hello".fg(Color::WHITE));
}

// add function to display received text at position for seconds
