pub use crate::prelude::*;
use std::cmp::{max, min};

#[derive(Component, PartialEq, Clone)]
pub struct Map {
    pub tiles: Vec<Tile>,
    pub height: i32,
    pub width: i32,
    pub revealed_tiles: Vec<bool>,
    pub rooms: Vec<rltk::Rect>,
    pub blocked_tiles: Vec<bool>,
}

#[derive(Component, PartialEq, Clone)]
pub struct Tile {
    pub tile: TileType,
    pub render: Renderable,
    pub location: Position,
    pub contents: Vec<Option<Entity>>,
}

#[derive(Component, PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor,
}

impl Map {
    pub fn clear_content_index(&mut self) {
        self.tiles.clear();
    }

    pub fn new() -> Map {
        let mut map = Map {
            rooms: Vec::new(),
            blocked_tiles: vec![false; (MAP_HEIGHT * MAP_WIDTH) as usize],
            height: MAP_HEIGHT,
            width: MAP_WIDTH,
            tiles: vec![
                Tile {
                    //contents: Vec::new(),
                    contents: vec![None; (MAP_HEIGHT * MAP_WIDTH) as usize],
                    tile: TileType::Floor,
                    render: Renderable {
                        glyph: '.',
                        fg: Color::DARK_GRAY,
                        bg: Color::BLACK
                    },
                    location: Position { x: 0, y: 0 }
                };
                (MAP_HEIGHT * MAP_WIDTH) as usize
            ],
            revealed_tiles: vec![false; (MAP_HEIGHT * MAP_WIDTH) as usize],
        };

        for x in 0..MAP_WIDTH {
            for y in 0..MAP_HEIGHT {
                map.tiles[xy_idx(x, y)].location.x = x;
                map.tiles[xy_idx(x, y)].location.y = y;
            }
        }

        // Make the boundaries walls
        for x in 0..MAP_WIDTH {
            //map.tiles[xy_idx(x, 0)] = TileType::Wall;
            map.tiles[xy_idx(x, 0)] = Tile {
                contents: vec![None; (MAP_HEIGHT * MAP_WIDTH) as usize],
                tile: TileType::Wall,
                render: Renderable {
                    glyph: '#',
                    fg: Color::GRAY,
                    bg: Color::BLACK,
                },
                location: Position { x: x, y: 0 },
            };

            // map.tiles[xy_idx(x, 49)] = TileType::Wall;

            map.tiles[xy_idx(x, MAP_HEIGHT - 1)] = Tile {
                contents: vec![None; (MAP_HEIGHT * MAP_WIDTH) as usize],
                tile: TileType::Wall,
                render: Renderable {
                    glyph: '#',
                    fg: Color::GRAY,
                    bg: Color::BLACK,
                },
                location: Position {
                    x: x,
                    y: MAP_HEIGHT - 1,
                },
            };
        }
        for y in 0..50 {
            //map.tiles[xy_idx(0, y)] = TileType::Wall;
            map.tiles[xy_idx(0, y)] = Tile {
                contents: vec![None; (MAP_HEIGHT * MAP_WIDTH) as usize],
                tile: TileType::Wall,
                render: Renderable {
                    glyph: '#',
                    fg: Color::GRAY,
                    bg: Color::BLACK,
                },
                location: Position { x: 0, y: y },
            };

            // map.tiles[xy_idx(79, y)] = TileType::Wall;
            map.tiles[xy_idx(MAP_WIDTH - 1, y)] = Tile {
                contents: vec![None; (MAP_HEIGHT * MAP_WIDTH) as usize],
                tile: TileType::Wall,
                render: Renderable {
                    glyph: '#',
                    fg: Color::GRAY,
                    bg: Color::BLACK,
                },
                location: Position {
                    x: MAP_WIDTH - 1,
                    y: y,
                },
            };
        }

        // Now we'll randomly splat a bunch of walls. It won't be pretty, but it's a decent illustration.
        // First, obtain the thread-local RNG:
        let mut rng = rltk::RandomNumberGenerator::new();

        for _i in 0..(MAP_WIDTH * MAP_HEIGHT / 10) {
            //approx 10% of map covered in walls
            let x = rng.roll_dice(1, MAP_WIDTH - 1);
            let y = rng.roll_dice(1, MAP_HEIGHT - 1);
            let idx = xy_idx(x, y);
            if idx != xy_idx(MAP_WIDTH / 2, MAP_HEIGHT / 2) {
                //if wall position != middle of screen (player start)
                map.tiles[idx] = Tile {
                    contents: vec![None; (MAP_HEIGHT * MAP_WIDTH) as usize],
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
    pub fn populate_blocked_tiles(&mut self) {
        for (i, tile) in self.tiles.iter_mut().enumerate() {
            self.blocked_tiles[i] = (tile.tile == TileType::Wall);
        }
    }

    fn is_exit_valid(&self, x: i32, y: i32) -> bool {
        if x < 1 || x > self.width - 1 || y < 1 || y > self.height - 1 {
            return false;
        }
        let idx = xy_idx(x, y);
        !self.blocked_tiles[idx]
        //self.tiles[idx as usize].tile != TileType::Wall
    }

    pub fn new_map_rooms_and_corridors() -> Map {
        let mut map = Map {
            rooms: Vec::new(),
            blocked_tiles: vec![false; (MAP_HEIGHT * MAP_WIDTH) as usize],
            tiles: vec![
                Tile {
                    contents: vec![None; (MAP_HEIGHT * MAP_WIDTH) as usize],
                    tile: TileType::Wall,
                    render: Renderable {
                        glyph: '#',
                        fg: Color::GRAY,
                        bg: Color::BLACK
                    },
                    location: Position { x: 0, y: 0 }
                };
                (MAP_HEIGHT * MAP_WIDTH) as usize
            ],
            height: MAP_HEIGHT,
            width: MAP_WIDTH,
            revealed_tiles: vec![false; (MAP_HEIGHT * MAP_WIDTH) as usize],
        };

        // REFACTOR: dumb but works - set the position of each item in the vector of map tiles to a different value
        for x in 0..MAP_WIDTH {
            for y in 0..MAP_HEIGHT {
                map.tiles[xy_idx(x, y)].location.x = x;
                map.tiles[xy_idx(x, y)].location.y = y;
            }
        }

        let mut rooms: Vec<rltk::Rect> = Vec::new();
        const MAX_ROOMS: i32 = 30;
        const MIN_SIZE: i32 = 6;
        const MAX_SIZE: i32 = 10;

        let mut rng = RandomNumberGenerator::new();

        for _ in 0..MAX_ROOMS {
            let w = rng.range(MIN_SIZE, MAX_SIZE);
            let h = rng.range(MIN_SIZE, MAX_SIZE);
            let x = rng.roll_dice(1, MAP_WIDTH - w - 1) - 1;
            let y = rng.roll_dice(1, MAP_HEIGHT - h - 1) - 1;
            let new_room = rltk::Rect::with_size(x, y, w, h);
            let mut ok = true;
            for other_room in rooms.iter() {
                if new_room.intersect(other_room) {
                    ok = false
                }
            }
            if ok {
                apply_room_to_map(&new_room, &mut map);

                if !rooms.is_empty() {
                    let (new_x, new_y) = (new_room.center().x, new_room.center().y);
                    let (prev_x, prev_y) = (
                        rooms[rooms.len() - 1].center().x,
                        rooms[rooms.len() - 1].center().y,
                    );
                    if rng.range(0, 2) == 1 {
                        apply_horizontal_tunnel(&mut map, prev_x, new_x, prev_y);
                        apply_vertical_tunnel(&mut map, prev_y, new_y, new_x);
                    } else {
                        apply_vertical_tunnel(&mut map, prev_y, new_y, prev_x);
                        apply_horizontal_tunnel(&mut map, prev_x, new_x, new_y);
                    }
                }
                rooms.push(new_room);
            }
        }

        // add rooms to map struct
        map.rooms = rooms;

        // return map
        map
    }
}

fn apply_room_to_map(room: &rltk::Rect, map: &mut Map) {
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
        if idx > 0 && idx < (MAP_WIDTH as usize * MAP_HEIGHT as usize) as usize {
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
        if idx > 0 && idx < (MAP_WIDTH as usize * MAP_HEIGHT as usize) as usize {
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

impl Algorithm2D for Map {
    fn dimensions(&self) -> Point {
        Point::new(self.width, self.height)
    }
}

impl BaseMap for Map {
    fn is_opaque(&self, idx: usize) -> bool {
        self.tiles[idx as usize].tile == TileType::Wall
    }

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        let w = self.width as usize;
        let p1 = Point::new(idx1 % w, idx1 / w);
        let p2 = Point::new(idx2 % w, idx2 / w);
        rltk::DistanceAlg::Pythagoras.distance2d(p1, p2)
    }

    fn get_available_exits(&self, idx: usize) -> rltk::SmallVec<[(usize, f32); 10]> {
        let mut exits = rltk::SmallVec::new();
        let x = idx as i32 % self.width;
        let y = idx as i32 / self.width;
        let w = self.width as usize;
        // let (_, _) = idx_xy(idx); // DEBUG ONLY: remove
        //                           // Cardinal directions
        if self.is_exit_valid(x, y + 1) {
            exits.push((xy_idx(x, y + 1), 1.0))
        }; // up
           // if self.is_exit_valid(x + 1, y + 1) {
           //     exits.push((xy_idx(x + 1, y + 1), 1.0))
           // }; // up-right
        if self.is_exit_valid(x + 1, y) {
            exits.push((xy_idx(x + 1, y), 1.0))
        }; // right
           // if self.is_exit_valid(x + 1, y - 1) {
           //     exits.push((xy_idx(x + 1, y - 1), 1.0))
           // }; // down-right
        if self.is_exit_valid(x, y - 1) {
            exits.push((xy_idx(x, y - 1), 1.0))
        }; // down
           // if self.is_exit_valid(x - 1, y - 1) {
           //     exits.push((xy_idx(x - 1, y - 1), 1.0))
           // }; // down-left
        if self.is_exit_valid(x - 1, y) {
            exits.push((xy_idx(x - 1, y) - 1, 1.0))
        }; // left
           // if self.is_exit_valid(x - 1, y + 1) {
           //     exits.push((xy_idx(x - 1, y + 1), 1.0))
           // }; // up-left

        exits
    }
}

// map building
pub fn xy_idx(x: i32, y: i32) -> usize {
    (y as usize * MAP_WIDTH as usize) + x as usize
}

pub fn idx_xy(idx: usize) -> (i32, i32) {
    // y * MAP_WIDTH + x = idx
    // y = y / MAP_WIDTH - x
    let y = idx as i32 / MAP_WIDTH;
    let x = idx as i32 - (MAP_WIDTH * y);
    (x, y)
}

// determines where there are blocks on the map that are more than just walls.
pub struct MapIndexingSystem {}
impl MapIndexingSystem {
    pub fn run(
        mut query_map: Query<&mut Map>,
        query_blocked_positions: Query<&Position, With<BlocksTile>>,
    ) {
        let mut map = query_map.iter_mut().nth(0).unwrap();
        map.populate_blocked_tiles();

        query_blocked_positions.iter().for_each(|pos| {
            //println!("blocked_pos: {:#?}", pos);
            let idx = xy_idx(pos.x, pos.y);
            map.blocked_tiles[idx] = true;
        });
    }
}

// fn spawn_monsters(map: Map) {
//     // spawn npcs
//     for i in 1..map.rooms.len() {
//         let mut rng = rltk::RandomNumberGenerator::new();
//         let roll = rng.roll_dice(1, 2);
//         let (glyph, name) = match roll {
//             1 => ('G', "Goblin".to_string()),
//             2 => ('O', "Orc".to_string()),
//             _ => ('X', "Xenity".to_string()),
//         };

//         // spawn player in center of first room on map
//         Commands::spawn(NPCBundle {
//             position: Position {
//                 x: map.rooms[0].center().x,
//                 y: map.rooms[0].center().y,
//             },
//             ..Default::default()
//             });
//     }
// }
