use crate::components;
pub use crate::prelude::*;

pub fn player_walk(
    input: Res<Input<KeyCode>>,
    mut ev_combat: EventWriter<CombatAttack>,
    //mut player_pos: Query<(Entity, &Player, &mut Position)>,
    mut entity_positions: Query<&mut Position>,
    mut player_pos: Query<(Entity, &Player), With<Position>>,
    query_enemy: Query<Entity, (With<Enemy>, With<Position>)>,
    query_map: Query<&Map>,
    mut query_viewshed: Query<&mut Viewshed>,
    mut query_game_state: Query<&mut components::GameState>,
) {
    let map = query_map.iter().nth(0).unwrap();
    let move_input = read_movement(input);
    if move_input.cmpeq(IVec2::ZERO).all() {
        return;
    }

    let (entity, player) = player_pos //rename player_pos to another variable
        .iter_mut()
        .nth(0)
        .map(|(entity, player)| (entity, player))
        .unwrap();

    let mut pos = entity_positions
        .get_component_mut::<Position>(entity)
        .unwrap();

    let curr = IVec2::new(pos.x, pos.y);
    let next = curr + move_input;

    // check if player can validly move to desired spot
    // if map.blocked_tiles[xy_idx(next.x, next.y)] {
    //     return;
    // };

    //     fn print_selected_character_name_system(
    //         query: Query<&Character>,
    //         selection: Res<SelectedCharacter>
    //  )
    //  {
    //      if let Ok(selected_character) = query.get_component::<Character>(selection.entity) {
    //          println!("{}", selected_character.name);
    //      }
    //  }

    if map.blocked_tiles[xy_idx(next.x, next.y)] {
        //define the parameters of the combat attack
        // let enemy = query_enemy
        //     .iter()
        //     .filter(|e| xy_idx(e.2.x, e.2.y) == xy_idx(next.x, next.y))
        //     .map(|e| e.0)
        //     .nth(0)
        //     .unwrap();

        query_enemy.iter().for_each(|e| {
            //println!("query_enemy.iter().for_each(|e| : {:#?}", &e);
            if let Ok(enemy_pos) = entity_positions.get_component::<Position>(e) {
                //println!("Ok(enemy_pos) : {:#?}", &enemy_pos);
                if xy_idx(enemy_pos.x, enemy_pos.y) == xy_idx(next.x, next.y) {
                    let combat_attack: CombatAttack = CombatAttack {
                        source: entity,
                        // TODO: need to change this to be whatever entity is occupying the space that is
                        // trying to be moved in to
                        target: e,
                        damage: (1, 4), //TODO: Update the damage to be based on the combat stats
                    };
                    println!("player_combat_attack: {:#?}", &combat_attack);
                    ev_combat.send(combat_attack);
                }
            }
        });
        // .filter(|e| xy_idx(e.x, e.y) == xy_idx(next.x, next.y))
        // .map(|e| )
        // .nth(0)
        // .unwrap();
        return;
    }

    // if so, then update player position
    pos.x = next.x;
    pos.y = next.y;

    let mut viewshed = query_viewshed.iter_mut().nth(0).unwrap();
    viewshed.dirty = true;
    let mut game_state = query_game_state.iter_mut().nth(0).unwrap();
    game_state.runstate = RunState::Running;
}

// an IVec2 is a 2-dimensional vector (direction and distance for x and y both)
fn read_movement(input: Res<Input<KeyCode>>) -> IVec2 {
    let mut p = IVec2::ZERO;

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
