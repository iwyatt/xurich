pub use crate::prelude::*;

pub fn resolve_combat_events(
    mut commands: Commands,
    //mut query_player_pos: Query<(&Position, &mut Viewshed), With<Player>>,
    mut query_actors: Query<(Entity, &mut Actor, &mut CombatStats)>, //TODO: Remove Actor Tag from Systems
    mut query_player: Query<Entity, With<Player>>,
    // query_combat_attacks: Query<&CombatAttack>,
    mut ev_combat: EventReader<CombatAttack>,
    //query_entities: Query<(Entity, &CombatStats)>,
    //mut event_combat_attack: EventReader<CombatAttack>,
    mut query_rng: Query<&mut RNG>,
    mut query_game_state: Query<&mut GameState>,
) {
    let mut rng = query_rng.single_mut();
    let mut actors: Vec<(Entity, &Actor, &CombatStats)> = Vec::new();
    query_actors.iter().for_each(|a| {
        let entity = a.0;
        let actor = a.1;
        let stats = a.2;
        actors.push((entity, actor, stats));
    });

    for e in ev_combat.read() {
        // println!("e in ev_combat.iter(): {:#?}", e);
        //let mut rng = RandomNumberGenerator::new();
        let mut target = query_actors.get_mut(e.target).unwrap();
        target.2.hp -= rng.0.roll_dice(e.damage.0, e.damage.1);
        println!("target.1.hp: {:#?}", target.2.hp);
        if target.2.hp <= 0 {
            if target.0 == query_player.single() {
                let mut game_state = query_game_state.single_mut();

                // TODO: should consider sending game state change to an event system
                game_state.runstate = RunState::GameOver;
                println!("player died");
            } else {
                commands.entity(e.target).despawn(); //TODO: Add game over screen for player
            }
        }
    }
}
