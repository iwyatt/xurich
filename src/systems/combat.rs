use crate::components;
pub use crate::prelude::*;

pub fn resolve_combat_events(
    //mut query_player_pos: Query<(&Position, &mut Viewshed), With<Player>>,
    mut query_actors: Query<(&mut Actor, &mut CombatStats)>,
    query_combat_attacks: Query<&CombatAttack>,
    //query_entities: Query<(Entity, &CombatStats)>,
    //mut event_combat_attack: EventReader<CombatAttack>,
) {
    //let mut actors: Vec<(Actor, CombatStats)> = query_actors.iter().map(|(actor, combat_stats)| vec![(actor, combat_stats)]).collect();
    // for attack in event_combat_attack.iter() {

    // }

    let mut actors: Vec<(&Actor, &CombatStats)> = Vec::new();

    query_actors.iter().for_each(|a| {
        let actor = a.0;
        let stats = a.1;
        actors.push((actor, stats));
    });

    query_combat_attacks.iter().for_each(|attack| {
        let mut rng = RandomNumberGenerator::new();
        //actors[1].1.hp -= attack.damage;
        //actors[attack.target].1 -= attack.damage;
        //let target = query_combat_attacks.get_mut(attack.target);
        //let source = query_combat_attacks.get(attack.source).unwrap();
        let mut target = query_actors.get_mut(attack.target).unwrap();
        target.1.hp -= rng.roll_dice(attack.damage.0, attack.damage.1);
        println!("target.1.hp: {:#?}", target.1.hp);
    });
}

// example from documentation
// fn print_selected_character_name_system(
//     query: Query<&Character>,
//     selection: Res<SelectedCharacter>
// )
// {
//  if let Ok(selected_character) = query.get(selection.entity) {
//      println!("{}", selected_character.name);
//  }
// }
