pub use crate::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_ascii_terminal::*;
use Terminal;

// render update
pub fn tick(
    mut query_terminal: Query<&mut Terminal>,
    query_entities: Query<(&Position, &Renderable, &crate::components::Name)>,
    query_combat_stats: Query<&CombatStats, With<Player>>,
    query_maps: Query<&Map>,
    query_camera: Query<&TiledCamera>,
    query_player_inventory: Query<(&Inventory, &Children), With<Player>>,
    query_player_inventory_items: Query<&crate::components::Name, With<Item>>,
    mut query_player_viewshed: Query<&mut Viewshed, With<Player>>,
    query_windows: Query<&Window, With<PrimaryWindow>>,
) {
    let map = query_maps.iter().nth(0).unwrap();
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

    // render player stat bar
    let player_combat_stats = query_combat_stats.single();
    let line = [
        "HP:",
        &player_combat_stats.hp.to_string(),
        "/",
        &player_combat_stats.max_hp.to_string(),
    ]
    .join(" ");
    terminal.put_string([0, MAP_HEIGHT + 1], line.fg(Color::WHITE));

    // render player quick-inventory
    if let Ok(pinventory) = query_player_inventory.get_single() {
        // TODO : loop through  three items only and concatenate a big string
        pinventory
            .1
            .iter()
            .take(3)
            .enumerate()
            .for_each(|(quick_i, c)| {
                if let Ok(i) = query_player_inventory_items.get(*c) {
                    //let line = String::from("(") + quick_i.to_string().as_str() + ") " + (&i.0.to_string());
                    let line = format!("({}) {}", quick_i + 1, i.0);
                    terminal.put_string(
                        [15 * (quick_i + 1) as i32, MAP_HEIGHT + 0],
                        line.fg(Color::WHITE),
                    );
                }
            });
    }

    //render player and npcs
    query_entities.iter().for_each(|(pos, rend, _)| {
        if visible_tiles.contains(&Point::new(pos.x, pos.y)) {
            terminal.put_char([pos.x, pos.y], rend.glyph.fg(rend.fg).bg(rend.bg))
        }
    });

    // render toolips
    let camera = query_camera.single();
    if let Some(cursor_world_position) = query_windows.single().cursor_position() {
        let mouse_map_pos = window_pos_to_map_pos(camera, &cursor_world_position);
        // println!("mouse_idx_pos: {:#?}", mouse_idx_pos);

        // debug to terminal:
        // terminal.put_string(
        //     [0, MAP_HEIGHT],
        //     ["world cursor", &cursor_world_position.to_string()]
        //         .join(": ")
        //         .fg(Color::WHITE),
        // );

        // let (pos, _, _) = query_entities
        //     .iter()
        //     .filter(|x| x.1.glyph == '@')
        //     .nth(0)
        //     .unwrap();
        // terminal.put_string(
        //     [0, MAP_HEIGHT - 1],
        //     ["player pos: ", &pos.x.to_string(), &pos.y.to_string()]
        //         .join(",")
        //         .fg(Color::WHITE),
        // );

        // terminal.put_string(
        //     [0, MAP_HEIGHT - 2],
        //     [
        //         "mouse pos: ",
        //         &mouse_map_pos.0.to_string(),
        //         &mouse_map_pos.1.to_string(),
        //     ]
        //     .join(",")
        //     .fg(Color::WHITE),
        // );

        query_entities
            .iter()
            .filter(|e| e.0.x == mouse_map_pos.0 && e.0.y == mouse_map_pos.1)
            .for_each(|e| {
                terminal.put_string(
                    // TODO: impl smart positioning for edges of screen
                    [mouse_map_pos.0, mouse_map_pos.1 + 1],
                    e.2 .0.clone().fg(Color::WHITE),
                )
            });
    }
}

// add function to display received text at position for seconds

fn window_pos_to_map_pos(camera: &TiledCamera, mouse_pos: &Vec2) -> (i32, i32) {
    // TODO : bring in game desktop Window variables and convert mouse pos based on window size
    let camera_space_max_x = bevy_ascii_terminal::Size2d::width(&camera.viewport_size())
        + bevy_ascii_terminal::Size2d::width(&camera.viewport_pos());
    let camera_space_min_x =
        camera_space_max_x - bevy_ascii_terminal::Size2d::width(&camera.viewport_size());
    let mouse_local_x = mouse_pos.x as i32 - camera_space_min_x as i32;
    let tile_x = mouse_local_x / 8 - 1; //divided by pixels per tile minus border

    let camera_space_max_y = (bevy_ascii_terminal::Size2d::height(&camera.viewport_size())
        + bevy_ascii_terminal::Size2d::height(&camera.viewport_pos()))
        as i32;
    let camera_space_min_y =
        camera_space_max_y - bevy_ascii_terminal::Size2d::height(&camera.viewport_size()) as i32;
    let mouse_local_y = camera_space_max_y - camera_space_min_y - mouse_pos.y as i32;
    let tile_y = (-camera_space_max_y / 2 + mouse_local_y) / 8 + MAP_HEIGHT + 2; //divide by pixel height plus map height plus 2 for gui

    (tile_x, tile_y)
}
