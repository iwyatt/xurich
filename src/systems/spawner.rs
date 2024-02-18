pub use crate::prelude::*;

pub fn spawn_random_mob(mut commands: &mut Commands, position: Position, mut rng: &mut RNG) {
    // let mut rng = query_rng.single_mut();
    let roll = rng.0.roll_dice(1, 2);
    let (glyph, name) = match roll {
        1 => ('G', "Goblin".to_string()),
        2 => ('O', "Orc".to_string()),
        _ => ('X', "Xenity".to_string()),
    };

    commands
        .spawn(NPCBundle {
            name: Name(name.into()),
            position: position,
            renderable: Renderable {
                glyph: glyph,
                fg: Color::YELLOW,
                bg: Color::BLACK,
            },
            ..Default::default()
        })
        .insert(BlocksTile)
        .insert(Enemy)
        .insert(Actor);
}
