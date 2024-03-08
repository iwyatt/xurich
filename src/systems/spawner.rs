use crate::prelude::*;
use rltk::*;

pub fn spawn_random_mob(
    commands: &mut Commands,
    position: Position,
    rng: &mut RNG,
    world_pos: WorldPosition,
) {
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
        .insert(Actor)
        .insert(world_pos);
}

pub fn spawn_player(commands: &mut Commands, position: Position) {
    commands
        .spawn(PlayerBundle {
            position: position,
            ..Default::default()
        })
        .insert(Inventory);
}

pub fn spawn_random_item(
    commands: &mut Commands,
    position: Position,
    rng: &mut RNG,
    world_pos: WorldPosition,
) {
    // TODO: replace this RNG line with getting the world RNG resource
    //let mut rng = RNG(RandomNumberGenerator::seeded(RNG_SEED));

    // roll for the type of item to roll
    let item_type = rng.0.roll_dice(1, 3); // TODO: change this to spawn fewer equipment

    // if random item type is armor
    if item_type == 1 {
        let (name, renderable, item) = match rng.0.roll_dice(1, 100) {
            1..=25 => {
                let name = Name(String::from("Leather Poncho"));
                let renderable = Renderable {
                    glyph: '♣',
                    fg: Color::WHITE,
                    bg: Color::BLACK,
                };
                let item = EquipmentBundle {
                    stat_bonus: CombatStats {
                        max_hp: 0,
                        hp: 0,
                        defense: 1,
                        power: 0,
                    },
                };
                (name, renderable, item)
            }

            _ => {
                let name = Name(String::from("Magic Leather Jerkin"));
                let renderable = Renderable {
                    glyph: '♣',
                    fg: Color::SEA_GREEN,
                    bg: Color::BLACK,
                };
                let item = EquipmentBundle {
                    stat_bonus: CombatStats {
                        max_hp: 10,
                        hp: 0,
                        defense: 2,
                        power: 1,
                    },
                };
                (name, renderable, item)
            }
        };

        commands
            .spawn((name, renderable, item, position))
            .insert(Item)
            .insert(Armor)
            .insert(world_pos);
        return;
    };

    // if random item type is weapon
    if item_type == 2 {
        // let name = Name(String::from("Dagger"));
        // let renderable = Renderable {
        //     glyph: '♠',
        //     fg: Color::ANTIQUE_WHITE,
        //     bg: Color::BLACK,
        // };
        // let item = EquipmentBundle {
        //     stat_bonus: CombatStats {
        //         max_hp: 0,
        //         hp: 0,
        //         defense: 0,
        //         power: 1,
        //     },
        // };

        let (name, renderable, item) = match rng.0.roll_dice(1, 100) {
            1..=10 => {
                let name = Name(String::from("Fine Dagger"));
                let renderable = Renderable {
                    glyph: '♠',
                    fg: Color::WHITE,
                    bg: Color::BLACK,
                };
                let item = EquipmentBundle {
                    stat_bonus: CombatStats {
                        max_hp: 0,
                        hp: 0,
                        defense: 0,
                        power: 2,
                    },
                };
                (name, renderable, item)
            }
            11..=16 => {
                let name = Name(String::from("Magic Dagger"));
                let renderable = Renderable {
                    glyph: '♠',
                    fg: Color::SEA_GREEN,
                    bg: Color::BLACK,
                };
                let item = EquipmentBundle {
                    stat_bonus: CombatStats {
                        max_hp: 5,
                        hp: 0,
                        defense: 0,
                        power: 1,
                    },
                };
                (name, renderable, item)
            }
            17..=21 => {
                let name = Name(String::from("Rare Dagger"));
                let renderable = Renderable {
                    glyph: '♠',
                    fg: Color::BLUE,
                    bg: Color::BLACK,
                };
                let item = EquipmentBundle {
                    stat_bonus: CombatStats {
                        max_hp: 5,
                        hp: 0,
                        defense: 0,
                        power: 2,
                    },
                };
                (name, renderable, item)
            }
            22..=25 => {
                let name = Name(String::from("Epic Dagger of Parrying"));
                let renderable = Renderable {
                    glyph: '♠',
                    fg: Color::PURPLE,
                    bg: Color::BLACK,
                };
                let item = EquipmentBundle {
                    stat_bonus: CombatStats {
                        max_hp: 5,
                        hp: 0,
                        defense: 2,
                        power: 1,
                    },
                };
                (name, renderable, item)
            }
            26 => {
                let name = Name(String::from("Legendary Dagger of Heros"));
                let renderable = Renderable {
                    glyph: '♠',
                    fg: Color::PURPLE,
                    bg: Color::BLACK,
                };
                let item = EquipmentBundle {
                    stat_bonus: CombatStats {
                        max_hp: 10,
                        hp: 0,
                        defense: 3,
                        power: 3,
                    },
                };
                (name, renderable, item)
            }
            _ => {
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
                (name, renderable, item)
            }
        };

        commands
            .spawn((name, renderable, item, position))
            .insert(Item)
            .insert(Weapon);
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
