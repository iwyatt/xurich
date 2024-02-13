pub use crate::prelude::*;

pub fn resolve_combat_events(
    mut commands: Commands,
    //mut query_player_pos: Query<(&Position, &mut Viewshed), With<Player>>,
    mut query_actors: Query<(&mut Actor, &mut CombatStats)>, //TODO: Remove Actor Tag from Systems
    // query_combat_attacks: Query<&CombatAttack>,
    mut ev_combat: EventReader<CombatAttack>,
    //query_entities: Query<(Entity, &CombatStats)>,
    //mut event_combat_attack: EventReader<CombatAttack>,
) {
    let mut actors: Vec<(&Actor, &CombatStats)> = Vec::new();
    query_actors.iter().for_each(|a| {
        let actor = a.0;
        let stats = a.1;
        actors.push((actor, stats));
    });

    for e in ev_combat.read() {
        // println!("e in ev_combat.iter(): {:#?}", e);
        let mut rng = RandomNumberGenerator::new();
        let mut target = query_actors.get_mut(e.target).unwrap();
        target.1.hp -= rng.roll_dice(e.damage.0, e.damage.1);
        println!("target.1.hp: {:#?}", target.1.hp);
        if target.1.hp <= 0 {
            commands.entity(e.target).despawn() //TODO: Add game over screen for player
        };
    }
}
