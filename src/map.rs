pub use crate::prelude::*;
use std::{cmp::{max, min}, ops::Index};

// public helper functions
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

// map related structs
#[derive(Component)]
pub struct MapBundle {
    pub map: Map,
    pub player_start_pos: Position,
    pub npcs: Vec<NPCBundle>,
    //pub items: Vec<Item>,
}

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
}

#[derive(Component, PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor,
}

// MapGenerator provides parameters for new map generation
pub struct MapGenerator {
    pub rng: RNG,
    pub map_generator_algo: MapGenerationAlgo,
    pub rooms_range: Option<(usize, usize)>,
    pub room_size_range: Option<(usize, usize)>,
    pub cell_density: Option<usize>,
    pub mobs_range: (usize, usize),
    pub items_range: (usize, usize),
}

impl Default for MapGenerator {
    fn default() -> Self {
        Self {
            rng: RNG(RandomNumberGenerator::seeded(RNG_SEED)),
            map_generator_algo: MapGenerationAlgo::RoomsAndCorridors,
            room_size_range: Some((2, 10)),
            rooms_range: Some((4, (MAP_HEIGHT * MAP_WIDTH / 400).try_into().unwrap())),
            mobs_range: (1, (MAP_HEIGHT * MAP_WIDTH / 400).try_into().unwrap()),
            items_range: (1, (MAP_HEIGHT * MAP_WIDTH / 400 / 2).try_into().unwrap()),
            cell_density: Some((MAP_HEIGHT * MAP_WIDTH / 400) as usize),
        }
    }
}

pub enum MapGenerationAlgo {
    RoomsAndCorridors,
    CellularAutomata,
    HiveMap,
    DrunkardsWalk,
    MazesAndLabyrinths,
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
            exits.push((xy_idx(x - 1, y), 1.0))
        }; // left
           // if self.is_exit_valid(x - 1, y + 1) {
           //     exits.push((xy_idx(x - 1, y + 1), 1.0))
           // }; // up-left

        exits
    }
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

// map generation functions
impl Map {
    pub fn default() -> (Map, Position, Vec<Position>, Vec<Position>) {
        //defaut generation creates a map with lots of coordiors but few rooms
        let mapgen = MapGenerator::default();
        Map::new_map_roomsandcorridors(mapgen)
    }

    pub fn random() -> (Map, Position, Vec<Position>, Vec<Position>) {
        let mut mapgen = MapGenerator::default();
        let map_type = mapgen.rng.0.roll_dice(1, 2);
        let map = match map_type {
            1 => Map::new_map_roomsandcorridors(mapgen), // TODO: randomize the parameters in mapgen
            _ => Map::new_map_cellularautomata(mapgen),  // TODO: randomize the parameters in mapgen
        };
        return map;
    }

    pub fn new_map_roomsandcorridors(mut mapgen: MapGenerator) -> (Map, Position, Vec<Position>, Vec<Position>) {
        // const to describe size and number of rooms in this map generation algorithm // TODO : Set these to parameters or a default impl?
        // const MAX_ROOMS: i32 = 0;
        // const MIN_SIZE: i32 = 6;
        // const MAX_SIZE: i32 = 10;

        let mut map = Map {
            rooms: Vec::new(),
            blocked_tiles: vec![false; (MAP_HEIGHT * MAP_WIDTH) as usize],
            tiles: vec![
                Tile {
                    //contents: vec![None; (MAP_HEIGHT * MAP_WIDTH) as usize],
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

        // REFACTOR: dumb but works - set the position of each tile in the vector of map tiles to a different value
        for x in 0..MAP_WIDTH {
            for y in 0..MAP_HEIGHT {
                map.tiles[xy_idx(x, y)].location.x = x;
                map.tiles[xy_idx(x, y)].location.y = y;
            }
        }

        // make rooms
        let mut rooms: Vec<rltk::Rect> = Vec::new();

        for _ in 0..mapgen.rooms_range.unwrap().1 {
            let w = mapgen
                .rng
                .0
                .range(mapgen.rooms_range.unwrap().0, mapgen.rooms_range.unwrap().1);
            let h = mapgen
                .rng
                .0
                .range(mapgen.rooms_range.unwrap().0, mapgen.rooms_range.unwrap().1);
            let x = mapgen.rng.0.roll_dice(1, MAP_WIDTH - w as i32 - 1) - 1;
            let y = mapgen.rng.0.roll_dice(1, MAP_HEIGHT - h as i32 - 1) - 1;
            let new_room = rltk::Rect::with_size(x, y, w as i32, h as i32);
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
                    if mapgen.rng.0.range(0, 2) == 1 {
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

        // npc positions vector
        let mut mob_start_pos = Vec::<Position>::new();
        for i in 1..map.rooms.len() {
            let position = Position {
                x: map.rooms[i].center().x,
                y: map.rooms[i].center().y,
            };
            mob_start_pos.push(position);
        }

        // set player start location
        let player_start_pos = Position {
            x: map.rooms[0].center().x,
            y: map.rooms[0].center().y,
        };

        // decide the number of items
        let mut num_items = mapgen
            .rng
            .0
            .range(mapgen.items_range.0, mapgen.items_range.1 + 1);

        // initialize item positions vector
        let mut item_start_pos = Vec::<Position>::new();

        // get set of tiles where there is not a wall
        let available_tiles = map
            .tiles
            .iter()
            .filter(|t| t.tile != TileType::Wall)
            .map(|t| &t.location)
            .collect::<Vec<&Position>>();
        
        // add items to random position in avialable tiles until number ofitems have been added
        while num_items > 0 {
            //let position = available_tiles[mapgen.rng.0.range(0, available_tiles.len())].clone();
            let tile = available_tiles[mapgen.rng.0.range(0, available_tiles.len())];
            let position = Position {
                x: tile.x,
                y: tile.y,
            };

            // if the random position is not the player's start position, then add
            // TODO : should just remove the player start position from the avialable tiles
            // TODO : should just remove the mob start position from the available tiles
            if position != player_start_pos && !mob_start_pos.contains(&position){ 
                 // remove the available tile now that it has an item on it
                item_start_pos.push(position);
                num_items -= 1;
            }            
        }

        // return map and player start position
        (map, player_start_pos, mob_start_pos, item_start_pos)
    }

    pub fn new_map_cellularautomata(mut mapgen: MapGenerator) -> (Map, Position, Vec<Position>, Vec<Position>) {
        let mut map = Map {
            rooms: Vec::new(),
            blocked_tiles: vec![false; (MAP_HEIGHT * MAP_WIDTH) as usize],
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
                (MAP_HEIGHT * MAP_WIDTH) as usize
            ],
            height: MAP_HEIGHT,
            width: MAP_WIDTH,
            revealed_tiles: vec![false; (MAP_HEIGHT * MAP_WIDTH) as usize],
        };

        // REFACTOR: dumb but works - set the position of each tile in the vector of map tiles to a different value
        for x in 0..MAP_WIDTH {
            for y in 0..MAP_HEIGHT {
                map.tiles[xy_idx(x, y)].location.x = x;
                map.tiles[xy_idx(x, y)].location.y = y;
            }
        }

        // Now we'll randomly splat a bunch of walls. It won't be pretty, but it's a decent illustration.
        for _ in 0..(MAP_WIDTH * MAP_HEIGHT / mapgen.cell_density.unwrap() as i32) {
            let x = mapgen.rng.0.roll_dice(1, MAP_WIDTH - 1);
            let y = mapgen.rng.0.roll_dice(1, MAP_HEIGHT - 1);
            let idx = xy_idx(x, y);
            if idx != xy_idx(MAP_WIDTH / 2, MAP_HEIGHT / 2) {
                //if wall position != middle of screen (player start)
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

        // set player start position
        let player_start_pos = Position {
            x: MAP_WIDTH / 2,
            y: MAP_HEIGHT / 2,
        };

        // decide the number of mobs
        let mut num_mobs = mapgen
            .rng
            .0
            .range(mapgen.mobs_range.0, mapgen.mobs_range.1 + 1);


        // decide the number of items
        let mut num_items = mapgen
            .rng
            .0
            .range(mapgen.items_range.0, mapgen.items_range.1 + 1);

        // get set of tiles where a mob can be spawned
        let available_tiles = map
            .tiles
            .iter()
            .filter(|t| t.tile != TileType::Wall)
            .map(|t| &t.location)
            .collect::<Vec<&Position>>();

        // println!("available_tiles: {:#?}", available_tiles.len());

        // initialize npc positions vector
        let mut mob_start_pos = Vec::<Position>::new();

        // add mobs to random position in available tiles until number of mobs have been added
        while num_mobs > 0 {
            //let position = available_tiles[mapgen.rng.0.range(0, available_tiles.len())].clone();
            let tile = available_tiles[mapgen.rng.0.range(0, available_tiles.len())];
            let position = Position {
                x: tile.x,
                y: tile.y,
            };

            // if the random position is not the player's start position, then add
            if position != player_start_pos {
                mob_start_pos.push(position);
                num_mobs -= 1;
            }
        }
        // println!("mob_start_pos: {:#?}", mob_start_pos);

        // initialize item positions vector
        let mut item_start_pos = Vec::<Position>::new();
        // add items to random position in avialable tiles until number ofitems have been added
        while num_items > 0 {
            //let position = available_tiles[mapgen.rng.0.range(0, available_tiles.len())].clone();
            let tile = available_tiles[mapgen.rng.0.range(0, available_tiles.len())];
            let position = Position {
                x: tile.x,
                y: tile.y,
            };

            // if the random position is not the player's start position, then add
            // TODO : should just remove the player start position from the avialable tiles
            // TODO : should just remove the mob start position from the available tiles
            if position != player_start_pos && !mob_start_pos.contains(&position){ 
                 // remove the available tile now that it has an item on it
                item_start_pos.push(position);
                num_items -= 1;
            }            
        }

        (map, player_start_pos, mob_start_pos, item_start_pos)
    }
}

// map generation helper functions
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

// helper functions for pathing
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
}
