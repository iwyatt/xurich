pub use crate::prelude::*;
use std::cmp::{max, min};

// map building
pub fn xy_idx(x: i32, y: i32) -> usize {
    (y as usize * 80) + x as usize
}

#[derive(Component, PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor,
}

#[derive(Component, PartialEq, Clone)]
pub struct Map {
    pub tiles: Vec<Tile>,
}

#[derive(Component, PartialEq, Clone)]
pub struct Tile {
    pub tile: TileType,
    pub render: Renderable,
    pub location: Position,
}

impl Map {
    pub fn new() -> Map {
        let mut map = Map {
            tiles: vec![
                Tile {
                    tile: TileType::Floor,
                    render: Renderable {
                        glyph: '.',
                        fg: Color::DARK_GRAY,
                        bg: Color::BLACK
                    },
                    location: Position { x: 0, y: 0 }
                };
                80 * 50
            ],
        };

        for x in 0..80 {
            for y in 0..50 {
                map.tiles[xy_idx(x, y)].location.x = x;
                map.tiles[xy_idx(x, y)].location.y = y;
            }
        }

        // Make the boundaries walls
        for x in 0..80 {
            //map.tiles[xy_idx(x, 0)] = TileType::Wall;
            map.tiles[xy_idx(x, 0)] = Tile {
                tile: TileType::Wall,
                render: Renderable {
                    glyph: '#',
                    fg: Color::GRAY,
                    bg: Color::BLACK,
                },
                location: Position { x: x, y: 0 },
            };

            // map.tiles[xy_idx(x, 49)] = TileType::Wall;

            map.tiles[xy_idx(x, 49)] = Tile {
                tile: TileType::Wall,
                render: Renderable {
                    glyph: '#',
                    fg: Color::GRAY,
                    bg: Color::BLACK,
                },
                location: Position { x: x, y: 49 },
            };
        }
        for y in 0..50 {
            //map.tiles[xy_idx(0, y)] = TileType::Wall;
            map.tiles[xy_idx(0, y)] = Tile {
                tile: TileType::Wall,
                render: Renderable {
                    glyph: '#',
                    fg: Color::GRAY,
                    bg: Color::BLACK,
                },
                location: Position { x: 0, y: y },
            };

            // map.tiles[xy_idx(79, y)] = TileType::Wall;
            map.tiles[xy_idx(79, y)] = Tile {
                tile: TileType::Wall,
                render: Renderable {
                    glyph: '#',
                    fg: Color::GRAY,
                    bg: Color::BLACK,
                },
                location: Position { x: 79, y: y },
            };
        }

        // Now we'll randomly splat a bunch of walls. It won't be pretty, but it's a decent illustration.
        // First, obtain the thread-local RNG:
        let mut rng = rltk::RandomNumberGenerator::new();

        for _i in 0..400 {
            let x = rng.roll_dice(1, 79);
            let y = rng.roll_dice(1, 49);
            let idx = xy_idx(x, y);
            if idx != xy_idx(40, 25) {
                map.tiles[idx] = Tile {
                    tile: TileType::Wall,
                    render: Renderable {
                        glyph: '#',
                        fg: Color::GRAY,
                        bg: Color::BLACK,
                    },
                    location: Position { x: x, y: y },
                };
            }
        }

        map
    }
}

// create a map of rooms and corridors
impl Map {
    pub fn new_map_rooms_and_corridors() -> Map {
        let mut map = Map {
            tiles: vec![
                Tile {
                    tile: TileType::Wall,
                    render: Renderable {
                        glyph: '#',
                        fg: Color::GRAY,
                        bg: Color::BLACK
                    },
                    location: Position { x: 0, y: 0 }
                };
                80 * 50
            ],
        };

        // DUMB BUT WORKS: set the position of each item in the vector of map tiles to a different value
        for x in 0..80 {
            for y in 0..50 {
                map.tiles[xy_idx(x, y)].location.x = x;
                map.tiles[xy_idx(x, y)].location.y = y;
            }
        }

        let room1 = Recti::new(20, 15, 10, 15);
        let room2 = Recti::new(35, 15, 10, 15);

        apply_room_to_map(&room1, &mut map);
        apply_room_to_map(&room2, &mut map);
        apply_horizontal_tunnel(&mut map, 25, 40, 23);

        map
    }
}

fn apply_room_to_map(room: &Recti, map: &mut Map) {
    for y in room.y1 + 1..=room.y2 {
        for x in room.x1 + 1..=room.x2 {
            map.tiles[xy_idx(x, y)].tile = TileType::Floor;
            map.tiles[xy_idx(x, y)].render = Renderable {
                glyph: '.',
                fg: Color::DARK_GRAY,
                bg: Color::BLACK,
            };
            map.tiles[xy_idx(x, y)].location = Position { x: x, y: y };
        }
    }
}

fn apply_horizontal_tunnel(map: &mut Map, x1: i32, x2: i32, y: i32) {
    for x in min(x1, x2)..=max(x1, x2) {
        let idx = xy_idx(x, y);
        if idx > 0 && idx < 80 * 50 {
            map.tiles[idx as usize].tile = TileType::Floor;
            map.tiles[idx as usize].render = Renderable {
                glyph: '.',
                fg: Color::DARK_GRAY,
                bg: Color::BLACK,
            };
            map.tiles[idx as usize].location = Position { x: x, y: y };
        }
    }
}

fn apply_vertical_tunnel(map: &mut Map, y1: i32, y2: i32, x: i32) {
    for y in min(y1, y2)..=max(y1, y2) {
        let idx = xy_idx(x, y);
        if idx > 0 && idx < 80 * 50 {
            map.tiles[idx as usize].tile = TileType::Floor;
            map.tiles[idx as usize].render = Renderable {
                glyph: '.',
                fg: Color::DARK_GRAY,
                bg: Color::BLACK,
            };
            map.tiles[idx as usize].location = Position { x: x, y: y };
        }
    }
}
