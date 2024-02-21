pub use crate::prelude::*;
use bevy_ascii_terminal::*;

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

pub fn spawn_player(mut commands: &mut Commands, position: Position) {
    commands.spawn(PlayerBundle {
        position: position,
        ..Default::default()
    });
}

pub fn spawn_random_item(commands: &mut Commands, position: Position) {
    let mut rng = RNG(RandomNumberGenerator::seeded(RNG_SEED));
    let (name, renderable, item) = match rng.0.roll_dice(1,2) {
        1 => {
            let name = Name(String::from("Small Health Potion"));
            let renderable = Renderable {glyph: 'Φ', fg: Color::MAROON, bg: Color::BLACK};
            let item = HealthPotion{heal_amount: 10};
            (name, renderable, item)
        },
        2 => {
            let name = Name(String::from("Health Potion"));
            let renderable = Renderable {glyph: 'Φ', fg: Color::CRIMSON, bg: Color::BLACK};
            let item = HealthPotion{heal_amount: 20};
            (name, renderable, item)
        }
        _ => {
            let name = Name(String::from("Health Potion"));
            let renderable = Renderable {glyph: 'Φ', fg: Color::CRIMSON, bg: Color::BLACK};
            let item = HealthPotion{heal_amount: 20};
            (name, renderable, item)
        }        
    };
    commands.spawn((name, renderable, item, position));
}
