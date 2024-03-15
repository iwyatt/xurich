use crate::components;
use crate::prelude::*;
use rltk::RandomNumberGenerator;

// TODO : Refactor so that keyboard input capture is just one function
//  and flows the program to other functions as appropriate

// Game State Player Inventory
pub fn inventory_cursor(
    input: Res<Input<KeyCode>>,
    mut query_cursor: Query<&mut InventoryCursor>,
    query_player_inventory: Query<(&Inventory, &Children), With<Player>>,
    // query_player_inventory_items: Query<
    //     (&crate::components::Name, &Renderable, Option<&IsEquipped>),
    //     (With<Item>),
    // >,
) {
    // navigate inventory cursor
    let mut cursor = query_cursor.single_mut();
    let mut num_inventory_items = 0;

    if let Ok(pinventory) = query_player_inventory.get_single() {
        num_inventory_items = pinventory.1.iter().len() as i32;
    }

    //println!("inventory count: {}", num_inventory_items);
    //let num_inventory_items = query_player_inventory_items.iter().len() as i32 - 1; // make this an index
    if input.just_pressed(KeyCode::Down) {
        // println!(
        //     "query_player_inventory.length() {}",
        //     query_player_inventory.iter().len()
        // );
        // println!(
        //     "old cursor.pos: {}, index_length_inventory_items: {}",
        //     cursor.pos, num_inventory_items
        // );
        // TODO: this works when num inventory > 1, but not for == 1
        if cursor.pos + 1 >= num_inventory_items {
            cursor.pos = num_inventory_items - 1
        } else {
            cursor.pos += 1;
        }
        //println!("new cursor.pos: {}", cursor.pos);
    }

    if input.just_pressed(KeyCode::Up) {
        if cursor.pos - 1 <= 0 {
            cursor.pos = 0
        } else {
            cursor.pos -= 1
        }
    }

    if input.just_pressed(KeyCode::Left) {
        if cursor.pos - 10 < 0 {
            cursor.pos = 0
        }
        // TODO: the jump between columns of the cursor needs to be
        //  based on the number of items in the rendered column.
        //  the above assumes there is 10 items per column
        else {
            cursor.pos -= 10
        }
    }

    if input.just_pressed(KeyCode::Right) {
        if cursor.pos + 10 > num_inventory_items {
            cursor.pos = num_inventory_items - 1
        }
        // TODO: the jump between columns of the cursor needs to be
        //  based on the number of items in the rendered column.
        //  the above assumes there is 10 items per column
        else {
            cursor.pos += 10
        }
    }
}

pub fn player_inventory_screen(
    //commands: Commands,
    input: Res<Input<KeyCode>>,
    //mut next_state: ResMut<NextState<GameLoopState>>,
    gamestate: Res<State<GameLoopState>>,
    mut query_game_state: Query<&mut components::GameState>,
    mut ev_open_inventory: EventWriter<EV_OpenInventoryTerminal>,
    mut ev_close_inventory: EventWriter<EV_CloseInventoryTerminal>,
) {
    //let move_input = read_movement(input);
    // if move_input.cmpeq(IVec2::ZERO).all() {
    //     return;
    // }
    if input.just_pressed(KeyCode::I) {
        if *gamestate.get() == GameLoopState::Inventory {
            println!("closing inventory");
            ev_close_inventory.send(EV_CloseInventoryTerminal);
            let mut game_state = query_game_state.iter_mut().nth(0).unwrap();
            game_state.runstate = RunState::Running;
        } else {
            ev_open_inventory.send(EV_OpenInventoryTerminal);
        }
    };

    // let mut game_state = query_game_state.iter_mut().nth(0).unwrap();
    // game_state.runstate = RunState::Running;
    // next_state.set(GameLoopState::Inventory);
}

// Game State Player Turn
pub fn player_wait(
    input: Res<Input<KeyCode>>,
    mut query_game_state: Query<&mut components::GameState>,
    gamestate: Res<State<GameLoopState>>,
    mut next_state: ResMut<NextState<GameLoopState>>,
) {
    if !input.just_pressed(KeyCode::S) {
        return;
    };
    // end of player's turn: switch game state so NPCs can take their turn
    let mut game_state = query_game_state.iter_mut().nth(0).unwrap();
    game_state.runstate = RunState::Running;
    next_state.set(GameLoopState::NPCTurn);
}

pub fn player_get_item(
    input: Res<Input<KeyCode>>,
    mut ev_itempickup: EventWriter<EV_ItemPickUp>,
    query_entity: Query<(Entity, &Position, With<Player>)>,
    gamestate: Res<State<GameLoopState>>,
    mut next_state: ResMut<NextState<GameLoopState>>,
    mut query_game_state: Query<&mut components::GameState>,
) {
    if !input.just_pressed(KeyCode::G) {
        return;
    };
    let (entity, position, _) = query_entity.single();
    let item_pickup: EV_ItemPickUp = EV_ItemPickUp {
        target: entity,
        position: Position {
            x: position.x,
            y: position.y,
        },
    };
    //println!("player_itempickup: {:#?}", &item_pickup);
    ev_itempickup.send(item_pickup);
    let mut game_state = query_game_state.iter_mut().nth(0).unwrap();
    game_state.runstate = RunState::Running;
    next_state.set(GameLoopState::NPCTurn);
}

pub fn player_use_item(
    input: Res<Input<KeyCode>>,
    mut ev_itemuse: EventWriter<EV_ItemUse>,
    query_inventory: Query<(Entity, &Inventory, &Children), With<Player>>,
    query_items: Query<&crate::components::Name>,
    mut query_game_state: Query<&mut components::GameState>,
    gamestate: Res<State<GameLoopState>>,
    mut next_state: ResMut<NextState<GameLoopState>>,
) {
    //println!("input: {:#?}", input);
    // only do something if inventory slot = key pressed slot
    let quick_slot = if input.just_pressed(KeyCode::Key1) {
        0
    } else if input.just_pressed(KeyCode::Key2) {
        1
    } else if input.just_pressed(KeyCode::Key3) {
        2
    } else {
        return;
    };

    //println!("input: {:#?}", input);

    // TODO: this code seems like it could be cleaner Ok(i) isn't necessary.
    if let Ok(pinventory) = query_inventory.get_single() {
        pinventory
            .2
            .iter()
            .enumerate()
            .filter(|(e, _)| e == &quick_slot)
            .for_each(|(_, c)| {
                // println!("pinventory.2.iter().for_each(|c {:#?}", pinventory.2);

                if let Ok(_) = query_items.get(*c) {
                    // println!("item: {:#?}", i);
                    let item_use = EV_ItemUse {
                        source: pinventory.0,
                        item: *c,
                    };
                    ev_itemuse.send(item_use);
                    let mut game_state = query_game_state.iter_mut().nth(0).unwrap();
                    game_state.runstate = RunState::Running;
                    next_state.set(GameLoopState::NPCTurn);
                }
            });
    }
}

pub fn player_walk(
    input: Res<Input<KeyCode>>,
    mut commands: Commands,
    mut ev_combat: EventWriter<CombatAttack>,
    //mut player_pos: Query<(Entity, &Player, &mut Position)>,
    mut entity_positions: Query<&mut Position>,
    mut query_player_world_pos: Query<&mut WorldPosition, With<Player>>,
    mut player_pos: Query<(Entity, &Player), With<Position>>,
    query_player_stats: Query<(Entity, &CombatStats), With<Player>>,
    query_enemy: Query<Entity, (With<Enemy>, With<Position>)>,
    //query_map: Query<&Map>,
    mut query_world_map: ResMut<WorldMap>,
    mut query_viewshed: Query<&mut Viewshed>,
    mut query_game_state: Query<&mut components::GameState>,
    mut query_rng: Query<&mut RNG>,
    gamestate: Res<State<GameLoopState>>,
    mut next_state: ResMut<NextState<GameLoopState>>,
) {
    // if using turn to move
    //let map = query_map.iter().nth(0).unwrap();
    let px = query_player_world_pos.single().x;
    let py = query_player_world_pos.single().y;
    let map = &mut query_world_map
        .maps
        .iter_mut()
        .filter(|m| m.world_pos.x == px && m.world_pos.y == py)
        .nth(0)
        .unwrap();

    let move_input = read_movement(input);
    if move_input.cmpeq(IVec2::ZERO).all() {
        return;
    }

    let (entity, _) = player_pos //TODO: change query / iter / map - rename player_pos to another variable
        .iter_mut()
        .nth(0)
        .map(|(entity, player)| (entity, player))
        .unwrap();

    let mut pos = entity_positions
        .get_component_mut::<Position>(entity)
        .unwrap();

    let curr = IVec2::new(pos.x, pos.y);
    //println!("player position IVec2::new(pos.x, pos.y) : {:#?}", curr);

    let next = curr + move_input;

    // check if next is on the adjacent map
    if next.x < 0 || next.x > MAP_WIDTH - 1 || next.y < 0 || next.y > MAP_HEIGHT - 1 {
        // get map's world pos and idx of current map FROM
        //curr_world_map_pos = map.world_pos;

        // get world pos and idx of map we are moving TO

        // TODO: BUG: implement logic for when we are changing both X and Y at same time (diaganal moves between maps)

        // moving to left
        let mut next_map_pos = WorldPosition { x: 0, y: 0, z: 0 };
        if next.x < 0 {
            next_map_pos = WorldPosition {
                x: &map.world_pos.x - 1,
                y: map.world_pos.y,
                z: map.world_pos.z,
            };
        }

        // moving to right
        if next.x > MAP_WIDTH - 1 {
            next_map_pos = WorldPosition {
                x: &map.world_pos.x + 1,
                y: map.world_pos.y,
                z: map.world_pos.z,
            };
        }

        // moving down
        if next.y < 0 {
            next_map_pos = WorldPosition {
                x: map.world_pos.x,
                y: &map.world_pos.y - 1,
                z: map.world_pos.z,
            };
        }

        // moving up
        if next.y > MAP_HEIGHT - 1 {
            next_map_pos = WorldPosition {
                x: map.world_pos.x,
                y: &map.world_pos.y + 1,
                z: map.world_pos.z,
            };
        }

        // if the map we are moving TO already exists
        if let Some(next_map) = query_world_map
            //.single()
            .maps
            .iter()
            .filter(|m| m.world_pos.x == next_map_pos.x && m.world_pos.y == next_map_pos.y)
            .nth(0)
        //.get(world_xy_idx(next_map_pos.x, next_map_pos.y))
        {
        } else {
            // if does not exist, then generate it
            let mut rng = query_rng.single_mut().to_owned();
            let mut mapgen = MapGenerator::default();
            mapgen.rng.0 = RandomNumberGenerator::seeded(
                (&next.x + WORLD_MAP_WIDTH * &next.y + WORLD_MAP_HEIGHT) as u64,
            );
            // println!(
            //     "mapgen rng: {:#?}",
            //     (&next.x + WORLD_MAP_WIDTH * &next.y + WORLD_MAP_HEIGHT) as u64
            // );
            // build new map
            let mut new_map = Map::new_map_cellularautomata(mapgen);

            // set map's worldmap pos
            new_map.0.world_pos.x = next_map_pos.x;
            new_map.0.world_pos.y = next_map_pos.y;
            new_map.0.world_pos.z = next_map_pos.z;

            // spawn npc bundle
            new_map.2.iter().for_each(|pos| {
                crate::spawner::spawn_random_mob(
                    &mut commands,
                    pos.clone(),
                    &mut rng,
                    WorldPosition {
                        x: next_map_pos.x,
                        y: next_map_pos.y,
                        z: next_map_pos.z,
                    },
                )
            });

            // spawn item bundle
            new_map.3.iter().for_each(|pos| {
                crate::spawner::spawn_random_item(
                    &mut commands,
                    pos.clone(),
                    &mut rng,
                    WorldPosition {
                        x: next_map_pos.x,
                        y: next_map_pos.y,
                        z: next_map_pos.z,
                    },
                )
            });

            // insert map into world map vector @ idx
            query_world_map
                //.single_mut()
                .maps
                .push(new_map.0);
        }

        // set the player's new world map location
        let mut player_world_pos = query_player_world_pos.single_mut();
        player_world_pos.x = next_map_pos.x;
        player_world_pos.y = next_map_pos.y;
        player_world_pos.z = next_map_pos.z;

        // set the player's new map position
        if next.y > MAP_HEIGHT - 1 {
            pos.y = pos.y - MAP_HEIGHT + 1
        };
        if next.y < 0 {
            pos.y = MAP_HEIGHT - 1
        };
        if next.x > MAP_WIDTH - 1 {
            pos.x = pos.x - MAP_WIDTH + 1
        };
        if next.x < 0 {
            pos.x = MAP_WIDTH - 1
        };
        return;
    }
    //println!("nx: {:#?}, ny: {:#?}", next.x, next.y);
    // check if tile to be moved in to is in the list of blocked tiles
    if map.blocked_tiles[xy_idx(next.x, next.y)] {
        // if it is, then get the enemy that is blocking
        query_enemy.iter().for_each(|e| {
            //println!("query_enemy.iter().for_each(|e| : {:#?}", &e);

            if let Ok(enemy_pos) = entity_positions.get_component::<Position>(e) {
                //println!("Ok(enemy_pos) : {:#?}", &enemy_pos);
                if xy_idx(enemy_pos.x, enemy_pos.y) == xy_idx(next.x, next.y) {
                    let player_power = query_player_stats.single();
                    let combat_attack: CombatAttack = CombatAttack {
                        source: entity,
                        // TODO: need to change this to be whatever entity is occupying the space that is
                        // trying to be moved in to
                        target: e,
                        damage: (1, player_power.1.power), // TODO: Update the damage to have a lower end
                    };
                    println!("player_combat_attack: {:#?}", &combat_attack);
                    ev_combat.send(combat_attack);
                }
            }
        });
    } else {
        // if not blocked, then update player position
        pos.x = next.x;
        pos.y = next.y;

        let mut viewshed = query_viewshed.iter_mut().nth(0).unwrap();
        viewshed.dirty = true;
    }

    // end of player's turn: switch game state so NPCs can take their turn
    let mut game_state = query_game_state.iter_mut().nth(0).unwrap();

    // TODO: Remove this game state after full implementation of GameLoopState
    game_state.runstate = RunState::Running;
    next_state.set(GameLoopState::NPCTurn);
}

// an IVec2 is a 2-dimensional vector (direction and distance for x and y both)
fn read_movement(input: Res<Input<KeyCode>>) -> IVec2 {
    let mut p = IVec2::ZERO;

    // move in cardinal directions, attack if moving into hostile actor's space
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
