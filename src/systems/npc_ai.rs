use std::borrow::Borrow;

use crate::components;
pub use crate::prelude::*;

// impl NPC_AI {}
pub fn run_npc_ai(
    mut commands: Commands,
    mut paramset: ParamSet<(
        Query<(Entity, &mut Position, &mut Viewshed, &mut NPC_AI), With<Enemy>>,
        Query<(Entity, &mut Position), With<Player>>,
    )>,
    mut query_game_state: Query<&mut components::GameState>,
    mut query_terminal: Query<&mut Terminal>,
    mut query_map: Query<&mut Map>,
) {
    let mut terminal = query_terminal.iter_mut().nth(0).unwrap();
    let mut map = query_map.iter_mut().nth(0).unwrap();
    let mut game_state = query_game_state.iter_mut().nth(0).unwrap();

    // skip if the game state is still paused
    if game_state.runstate == components::RunState::Paused {
        return;
    };
    // otherwise set the game state to paused regardless of monter ai outcome
    game_state.runstate = RunState::Paused;

    // set player entity id and position
    //let enemy_query = paramset.p0();
    // let player_query = paramset.p1();
    // let player = player_query.iter().nth(0).unwrap();
    let mut player_position = Position { x: 0, y: 0 };
    let mut player_entity = Entity::PLACEHOLDER;
    // set the player position variable to use
    for player in paramset.p1().iter() {
        (player_position, player_entity) = (player.1.clone(), player.0.clone());
    }
    println!("player_position: {:#?}", player_position);

    // process ai for each enemy with a position, viewshed, and NPC_AI
    paramset.p0().iter_mut().for_each(|enemy| {
        //for enemy in paramset.p0().iter_mut() {
        let (entity, mut pos, mut view, mut ai) = enemy;
        //println!("ai.state: {:#?}", ai.state);
        match ai.state {
            NPC_AI_State::Inactive => {
                if view
                    .visible_tiles
                    .contains(&Point::new(player_position.x, player_position.y))
                {
                    println!("Player within Monster Visibility");
                    ai.state = NPC_AI_State::Active;

                    // TODO: Move NPC emotes to UI logic
                    let npc_text = "Hello, World!".to_string();
                    let npc_text_pos_x = std::cmp::min(
                        std::cmp::max(pos.x - (npc_text.len() / 2) as i32, 0),
                        MAP_WIDTH - npc_text.len() as i32,
                    );
                    let npc_text_pos_y = std::cmp::min(pos.y + 1, MAP_HEIGHT);
                    terminal.put_string(
                        [npc_text_pos_x, npc_text_pos_y + 1],
                        npc_text.fg(Color::BLUE),
                    );
                }
                return;
            }
            NPC_AI_State::Active => {
                // get distance to player in order to inform what action monster takes
                // let distance_to_player = rltk::DistanceAlg::Pythagoras.distance2d(
                //     Point::new(player.1.x, player.1.y),
                //     Point::new(pos.x, pos.y),
                // );
                //println!("distance_to_player: {:#?}", distance_to_player);

                // get path (and # of steps) from NPC to player
                let path = rltk::a_star_search(
                    xy_idx(pos.x, pos.y) as i32,
                    xy_idx(player_position.x, player_position.y) as i32,
                    &mut *map,
                );
                println!("path.steps.len(): {:#?}", path.steps.len());

                let (next_x, next_y) = (
                    path.steps[1] as i32 % map.width,
                    path.steps[1] as i32 / map.width,
                );

                // assume npc viewshed needs to be updated
                view.dirty = true;

                // TODO: change these if statements to a MATCH statement
                // if the # of steps is > 10, then set NPC to inactive (consider resetting to their initial state)
                if path.success && path.steps.len() > 10 {
                    ai.state = NPC_AI_State::Inactive;
                }

                // if the next step would put the NPC on top of the player, do an attack instead
                if path.success && path.steps.len() <= 2 {
                    if true {
                        //if (next_x != player_position.x || next_y != player_position.y) {
                        // if where the npc wants to move is different than both the player and also other monsters
                        // if (next_x != player_position.x || next_y != player_position.y)
                        //     && !map.blocked_tiles[xy_idx(next_x, next_y)]
                        {
                            //define the parameters of the combat attack
                            let combat_attack: CombatAttack = CombatAttack {
                                source: enemy.0,
                                //TODO: need to change this to be whatever entity is occupying the space that is
                                // trying to be moved in to
                                target: player_entity,
                                damage: (1, 4),
                            };

                            //insert the combat attack to be resolved in the next run of the system
                            commands.spawn(combat_attack);

                            let npc_text = "Attack!".to_string();
                            println!("npc_text: {:#?}", npc_text);
                            let npc_text_pos_x = std::cmp::min(
                                std::cmp::max(pos.x - (npc_text.len() / 2) as i32, 0),
                                MAP_WIDTH - npc_text.len() as i32,
                            );
                            let npc_text_pos_y = std::cmp::min(pos.y + 1, MAP_HEIGHT);
                            // TODO: smart positioning of text re: relative to border of game window
                            terminal.put_string(
                                [npc_text_pos_x, npc_text_pos_y + 1],
                                npc_text.fg(Color::BLUE),
                            );
                            //return;
                        }
                    }
                }

                // if distance_to_player <= 1.43 { //1.43 is a good number
                //     let npc_text = "Attack!".to_string();
                //     let npc_text_pos_x = std::cmp::min(
                //         std::cmp::max(pos.x - (npc_text.len() / 2) as i32, 0),
                //         MAP_WIDTH - npc_text.len() as i32,
                //     );
                //     let npc_text_pos_y = std::cmp::min(pos.y + 1, MAP_HEIGHT);
                //     // TODO: smart positioning of text re: relative to border of game window
                //     terminal.put_string(
                //         [npc_text_pos_x, npc_text_pos_y + 1],
                //         npc_text.fg(Color::BLUE),
                //     );
                //     return;
                // }
                if path.success && path.steps.len() > 2 {
                    //if distance_to_player > 1.2 { // 1.43 is a good number
                    if view
                        .visible_tiles
                        .contains(&Point::new(player_position.x, player_position.y))
                    {
                        // let path = rltk::a_star_search(
                        //     xy_idx(pos.x, pos.y) as i32,
                        //     xy_idx(player_position.x, player_position.y) as i32,
                        //     &mut *map,
                        // );

                        // println!(
                        //     "path_success: {:#?}; path.steps.len(): {:#?}",
                        //     path.success,
                        //     path.steps.len()
                        // );
                        //if path.success && path.steps.len() > 1 {
                        // let (next_x, next_y) = (
                        //     path.steps[1] as i32 % map.width,
                        //     path.steps[1] as i32 / map.width,
                        // );

                        // if where the npc wants to move is different than both the player and also other monsters
                        if (next_x != player_position.x || next_y != player_position.y)
                            && !map.blocked_tiles[xy_idx(next_x, next_y)]
                        {
                            //println!("path.steps[1]: {:#?}", path.steps[1]);
                            //println!("before: pos.x: {:#?}, pos.y: {:#?}", pos.x, pos.y);

                            // set the position that the npc is vacating to not-blocking
                            map.blocked_tiles[xy_idx(pos.x, pos.y)] = false;

                            pos.x = path.steps[1] as i32 % map.width;
                            pos.y = path.steps[1] as i32 / map.width;
                            //println!("after: pos.x: {:#?}, pos.y: {:#?}", pos.x, pos.y);

                            // set the new position to be a blocked tile
                            map.blocked_tiles[xy_idx(pos.x, pos.y)] = true;
                        }

                        //}
                    }
                }
            }
            _ => {}
        }
    });
}
