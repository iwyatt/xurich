use crate::prelude::*;
use rltk::*;

pub fn spawn_random_mob(commands: &mut Commands, position: Position, rng: &mut RNG) {
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

pub fn spawn_player(commands: &mut Commands, position: Position) {
    commands
        .spawn(PlayerBundle {
            position: position,
            ..Default::default()
        })
        .insert(Inventory);
}

pub fn spawn_random_item(commands: &mut Commands, position: Position, rng: &mut RNG) {
    // TODO: replace this RNG line with getting the world RNG resource
    //let mut rng = RNG(RandomNumberGenerator::seeded(RNG_SEED));

    // roll for the type of item to roll
    let item_type = rng.0.roll_dice(1, 2); // TODO: change this to spawn fewer equipment

    // if random item type is equipment
    if item_type == 1 {
        let name = Name(String::from("Dagger"));
        let renderable = Renderable {
            glyph: '♠',
            fg: Color::ANTIQUE_WHITE,
            bg: Color::BLACK,
        };
        let item = EquipmentBundle {
            stat_bonus: CombatStats {
                max_hp: 0,
                hp: 0,
                defense: 0,
                power: 1,
            },
        };

        commands
            .spawn((name, renderable, item, position))
            .insert(Item)
            .insert(IsEquipped);
        return;
    }

    // if random item type is potion
    let (name, renderable, item) = match rng.0.roll_dice(1, 6) {
        1 => {
            let name = Name(String::from("Small Health Potion"));
            let renderable = Renderable {
                glyph: '♥',
                fg: Color::MAROON,
                bg: Color::BLACK,
            };
            let item = HealthPotion { heal_amount: 5 };
            (name, renderable, item)
        }
        2 | 3 => {
            let name = Name(String::from("Health Potion"));
            let renderable = Renderable {
                glyph: '♥',
                fg: Color::CRIMSON,
                bg: Color::BLACK,
            };
            let item = HealthPotion { heal_amount: 10 };
            (name, renderable, item)
        }
        4 => {
            let name = Name(String::from("Big Health Potion"));
            let renderable = Renderable {
                glyph: '♥',
                fg: Color::RED,
                bg: Color::BLACK,
            };
            let item = HealthPotion { heal_amount: 15 };
            (name, renderable, item)
        }
        _ => {
            let name = Name(String::from("Small Health Potion"));
            let renderable = Renderable {
                glyph: '♥',
                fg: Color::MAROON,
                bg: Color::BLACK,
            };
            let item = HealthPotion { heal_amount: 5 };
            (name, renderable, item)
        }
    };
    commands
        .spawn((name, renderable, item, position))
        .insert(Item);
}
