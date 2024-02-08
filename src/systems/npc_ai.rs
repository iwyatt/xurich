use crate::components;
pub use crate::prelude::*;

// impl NPC_AI {}
pub fn run_npc_ai(
    mut paramset: ParamSet<(
        Query<(&mut Position, &mut Viewshed, &mut NPC_AI), With<Enemy>>,
        Query<&mut Position, With<Player>>,
    )>,
    mut query_game_state: Query<&mut components::GameState>,
    mut query_terminal: Query<&mut Terminal>,
    mut query_map: Query<&mut Map>,
) {
    let mut game_state = query_game_state.iter_mut().nth(0).unwrap();
    if game_state.runstate == components::RunState::Paused {
        return;
    };
    game_state.runstate = RunState::Paused;

    // let query_terminal = &paramset.p2();
    let mut terminal = query_terminal.iter_mut().nth(0).unwrap();
    // let mut terminal = paramset.p2().iter_mut().nth(0).unwrap();

    // let query_map = &paramset.p3();
    let mut map = query_map.iter_mut().nth(0).unwrap();
    //let mut map = paramset.p3().iter_mut().nth(0).unwrap();

    let mut player_position = Position { x: 0, y: 0 };
    for position in paramset.p1().iter() {
        player_position = position.clone();
        //println!("{:#?}", player_position);
    }
    //let query_player = paramset.p1();
    //let player_position = query_player.iter().nth(0).unwrap();
    //paramset.p0().iter_mut().for_each_mut(|(pos, view, ai)|{
    paramset.p0().iter_mut().for_each(|enemy| {
        //for mut enemy in paramset.p0().iter_mut() {
        let (mut pos, mut view, mut ai) = enemy;
        //println!("{:#?}", ai.state);
        match ai.state {
            NPC_State::Inactive => {
                if view
                    .visible_tiles
                    .contains(&Point::new(player_position.x, player_position.y))
                {
                    println!("Player within Monster Visibility");
                    ai.state = NPC_State::Active;
                    let npc_text = "Hello, World!".to_string();
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
                }
                return;
            }
            NPC_State::Active => {
                // let distance_to_player : f32 = ((pos.x as f32 - player_position.x as f32).abs()
                //     + (pos.y as f32 - player_position.y as f32).abs())
                // .abs();
                let distance_to_player = rltk::DistanceAlg::Pythagoras.distance2d(
                    Point::new(player_position.x, player_position.y),
                    Point::new(pos.x, pos.y),
                );
                println!("distance_to_player: {:#?}", distance_to_player);

                if distance_to_player <= 1.42 {
                    let npc_text = "Attack!".to_string();
                    let npc_text_pos_x = pos.x - (npc_text.len() / 2) as i32;
                    let npc_text_pos_y = pos.y + 1;
                    // TODO: smart positioning of text re: relative to border of game window
                    terminal.put_string(
                        [npc_text_pos_x, npc_text_pos_y + 1],
                        npc_text.fg(Color::BLUE),
                    );
                    return;
                }

                if distance_to_player > 1.9 {
                    // this is a good distance to move around corners without losing sight
                    if view
                        .visible_tiles
                        .contains(&Point::new(player_position.x, player_position.y))
                    {
                        let path = rltk::a_star_search(
                            xy_idx(pos.x, pos.y) as i32,
                            xy_idx(player_position.x, player_position.y) as i32,
                            &mut *map,
                        );

                        // println!(
                        //     "path_success: {:#?}; path.steps.len(): {:#?}",
                        //     path.success,
                        //     path.steps.len()
                        // );
                        if path.success && path.steps.len() > 1 {
                            println!("path.steps[1]: {:#?}", path.steps[1]);
                            println!("before: pos.x: {:#?}, pos.y: {:#?}", pos.x, pos.y);
                            pos.x = path.steps[1] as i32 % map.width;
                            pos.y = path.steps[1] as i32 / map.width;
                            println!("after: pos.x: {:#?}, pos.y: {:#?}", pos.x, pos.y);
                        }
                        view.dirty = true;
                    }
                }
            }
            _ => {}
        };
    })
}
//}

//let (mut a, mut b, mut c) = query_enemy.iter_mut().nth(0).unwrap();
// query_enemy.iter_mut().for_each(|(&mut Position, &mut Viewshed, &mut NPC_AI)| {
// match ai.state {
//     NPC_State::Inactive => {
//         if view.visible_tiles
//             .contains(&Point::new(player_position.x, player_position.y))
//         {
//             ai.state = NPC_State::Active;
//             let npc_text = "Hello, World!".to_string();
//             let npc_text_pos_x = pos.x - (npc_text.len() / 2) as i32;
//             let npc_text_pos_y = pos.y + 1;
//             // TODO: smart positioning of text re: relative to border of game window
//             terminal.put_string(
//                 [npc_text_pos_x, npc_text_pos_y + 1],
//                 npc_text.fg(Color::BLUE),
//             );
//         }
//     }
//     NPC_State::Active => {
//         if view.visible_tiles
//             .contains(&Point::new(player_position.x, player_position.y))
//         {
//             let path = rltk::a_star_search(
//                 xy_idx(pos.x, pos.y) as i32,
//                 xy_idx(player_position.x, player_position.y) as i32,
//                 &mut *map,
//             );

//             if path.success && path.steps.len() > 1 {
//                 pos.x = path.steps[1] as i32 % map.width;
//                 pos.y = path.steps[1] as i32 / map.width;
//                 view.dirty = true;
//             }
//         }
//     }
//     _ => {}
// };
