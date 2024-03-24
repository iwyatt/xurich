use crate::prelude::*;

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
    mut rng: &mut RNG,
    world_pos: WorldPosition,
) {
    // TODO: replace this RNG line with getting the world RNG resource
    //let mut rng = RNG(RandomNumberGenerator::seeded(RNG_SEED));

    // roll for the type of item to roll
    let item_type = rng.0.roll_dice(1, 6); // TODO: change this to spawn fewer equipment
    println!("item_type = {:#?}", item_type);

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

            26..=46 => {
                let name = Name(String::from("Enchanted Leather Jerkin"));
                let renderable = Renderable {
                    glyph: '♣',
                    fg: Color::BLUE,
                    bg: Color::BLACK,
                };
                let item = EquipmentBundle {
                    stat_bonus: CombatStats {
                        max_hp: 0,
                        hp: 0,
                        defense: 2,
                        power: 2,
                    },
                };
                (name, renderable, item)
            }

            47..=62 => {
                let name = Name(String::from("Magic Chain Mail"));
                let renderable = Renderable {
                    glyph: '♣',
                    fg: Color::PURPLE,
                    bg: Color::BLACK,
                };
                let item = EquipmentBundle {
                    stat_bonus: CombatStats {
                        max_hp: 5,
                        hp: 0,
                        defense: 3,
                        power: 2,
                    },
                };
                (name, renderable, item)
            }
            63..=75 => {
                let name = Name(String::from("Splint Mail"));
                let renderable = Renderable {
                    glyph: '♣',
                    fg: Color::WHITE,
                    bg: Color::BLACK,
                };
                let item = EquipmentBundle {
                    stat_bonus: CombatStats {
                        max_hp: 0,
                        hp: 0,
                        defense: 5,
                        power: 0,
                    },
                };
                (name, renderable, item)
            }
            76..=85 => {
                let name = Name(String::from("Xurich's Arcane Full Plate"));
                let renderable = Renderable {
                    glyph: '♣',
                    fg: Color::ORANGE,
                    bg: Color::BLACK,
                };
                let item = EquipmentBundle {
                    stat_bonus: CombatStats {
                        max_hp: 10,
                        hp: 0,
                        defense: 5,
                        power: 3,
                    },
                };
                (name, renderable, item)
            }
            _ => {
                let name = Name(String::from("Cloak"));
                let renderable = Renderable {
                    glyph: '♣',
                    fg: Color::ANTIQUE_WHITE,
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
        };

        commands
            .spawn((name, renderable, item, position.clone()))
            .insert(Item)
            .insert(Armor)
            .insert(world_pos.clone());
        //return;
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

        let (name, renderable, item) = match rng.0.roll_dice(1, 50) {
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
                let name = Name(String::from("Enchanted Dagger"));
                let renderable = Renderable {
                    glyph: '♠',
                    fg: Color::BLUE,
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
            22..=25 => {
                let name = Name(String::from("Magic Dagger of Parrying"));
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
                let name = Name(String::from("Magic Dagger of Heros"));
                let renderable = Renderable {
                    glyph: '♠',
                    fg: Color::PURPLE,
                    bg: Color::BLACK,
                };
                let item = EquipmentBundle {
                    stat_bonus: CombatStats {
                        max_hp: 5,
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
            .spawn((name, renderable, item, position.clone()))
            .insert(world_pos.clone())
            .insert(Item)
            .insert(Weapon);
    }

    // if random item type is potion
    if item_type >= 3 {
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
            .insert(world_pos.clone())
            .insert(Item);
    };
}
