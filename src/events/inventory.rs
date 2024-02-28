pub use crate::prelude::*;

pub fn ev_pickup_item(
    mut commands: Commands,
    query_inventory: Query<(Entity, &Inventory)>,
    query_items: Query<(Entity, &Position), With<Item>>,
    mut ev_pickup_item: EventReader<EV_ItemPickUp>,
) {
    for e in ev_pickup_item.read() {
        println!("for e in ev_pickup_item.read(): {:#?}", e);
        // get the first item at the position of the triggering entity
        let item_entity = query_items
            .iter()
            .filter(|(_, p)| p.x == e.position.x && p.y == e.position.y)
            .map(|(e, _)| e)
            .nth(0);
        if item_entity.is_none() {
            return;
        };

        // let item_entity = query_items
        //     .iter()
        //     .filter(|(_, p)| p.x == e.position.x && p.y == e.position.y)
        //     .map(|(e, _)| e)
        //     .nth(0)
        //     .unwrap();

        // get target's inventory
        let inventory_entity = query_inventory
            .iter()
            .filter(|(a, _)| a == &e.target)
            .map(|(a, _)| a)
            .nth(0)
            .unwrap();

        // add item as child to inventory component
        commands
            .entity(inventory_entity)
            .add_child(item_entity.unwrap());
        println!("inventory.items.push(new_item): {:#?}", inventory_entity);
        //  remove item position as it is now only within the character's inventory and not on the map
        commands.entity(item_entity.unwrap()).remove::<Position>();
    }
}

pub fn ev_use_item(
    mut commands: Commands,
    mut ev_use_item: EventReader<EV_ItemUse>,
    query_items: Query<
        (
            Entity,
            Option<&HealthPotion>,
            Option<&EquipmentBundle>,
            Option<&Weapon>,
            Option<&Armor>,
        ),
        With<Item>,
    >,
    query_equipped: Query<
        (
            Entity,
            Option<&Weapon>,
            Option<&Armor>,
            Option<&EquipmentBundle>,
        ),
        With<IsEquipped>,
    >,
    query_player_position: Query<&Position, With<Player>>,
    mut query_combat_stats: Query<(Entity, &mut CombatStats)>,
) {
    for event in ev_use_item.read() {
        let (entity, potion, equipment, isweapon, isarmor) = query_items
            .iter()
            .filter(|(e, _, _, _, _)| *e == event.item)
            .map(|(e, p, eb, w, a)| (e, p, eb, w, a))
            .nth(0)
            .unwrap();

        // get stats of entity equipping the item
        let mut stats = query_combat_stats
            .iter_mut()
            .filter(|(e, _)| event.source == *e)
            .map(|(_, c)| c)
            .nth(0)
            .unwrap();
        // do something with the item

        // equipment // TODO: Update so that Armor can be equipped
        if let Some(equipment) = equipment {
            // TODO: if there is an item of the same type already equipped

            // if there are any equipped weapons
            // TODO: this assumes the new item is a weapon, change it
            query_equipped.iter().for_each(|e| {
                // if both the new item is a weapon and also there is a weapon equipped
                if isweapon.is_some() && e.1.is_some() {
                    // if the old item weapon has a equipment bundle
                    if let Some(item) = e.3 {
                        //remove item by lowering combat stats
                        println!("removing old weapon pre-removal stats: {:#?}", stats);
                        stats.power -= item.stat_bonus.power;
                        stats.defense -= item.stat_bonus.defense;
                        stats.max_hp -= item.stat_bonus.max_hp;
                        println!("removing old weapon post-removal stats: {:#?}", stats);

                        // remove the isequipped tag
                        commands.entity(e.0).remove::<IsEquipped>();

                        // remove the parent inventory
                        commands.entity(e.0).remove_parent();

                        // drop item at player's position
                        let pos = query_player_position.single();
                        commands.entity(e.0).insert(Position { x: pos.x, y: pos.y });
                    }

                    // if both the new item is a weapon and also there is a weapon equipped
                    if isarmor.is_some() && e.3.is_some() {
                        // if the old item weapon has a equipment bundle
                        if let Some(item) = e.3 {
                            //remove item by lowering combat stats
                            println!("removing old weapon pre-removal stats: {:#?}", stats);
                            stats.power -= item.stat_bonus.power;
                            stats.defense -= item.stat_bonus.defense;
                            stats.max_hp -= item.stat_bonus.max_hp;
                            println!("removing old weapon post-removal stats: {:#?}", stats);

                            // remove the isequipped tag
                            commands.entity(e.0).remove::<IsEquipped>();

                            // remove the parent inventory
                            commands.entity(e.0).remove_parent();

                            // drop item at player's position
                            let pos = query_player_position.single();
                            commands.entity(e.0).insert(Position { x: pos.x, y: pos.y });
                        }
                    }
                }
            });

            println!("pre-equip: {:#?}", stats);
            // equipping the item increases the combat stats of the entity
            stats.power += equipment.stat_bonus.power;
            stats.defense += equipment.stat_bonus.defense;
            stats.max_hp += equipment.stat_bonus.max_hp;

            // remove the equipment from the usable inventory
            //commands.entity(entity).despawn();
            println!("post-equip: {:#?}", stats);
            commands.entity(entity).insert(IsEquipped);
            return;
        }

        // potion
        if let Some(potion) = potion {
            let mut stats = query_combat_stats
                .iter_mut()
                .filter(|(e, _)| event.source == *e)
                .map(|(_, c)| c)
                .nth(0)
                .unwrap();
            stats.hp = stats.max_hp.min(stats.hp + potion.heal_amount);

            // remove potion from inventory
            commands.entity(entity).despawn();
        }
    }
}

// when player presses the num key corresponding to an unequipped item in inventory
pub fn equip_item(
    commands: Commands,
    equpment: EquipmentBundle,
    //mut ev_use_item: EventReader<EV_ItemEquip>,
    //query_items: Query<(Entity, Option<&EquipmentBundle>, Option<&IsEquipped>)>,
    mut combatstats: CombatStats,
) {
    // remove existing item of same type (weapon or body armor)

    // update combat stats for new item
    // add IsEquipped tag for renderer to pick up
    // remove item from 'usable' inventory
}

// when player presses the num key corresponding to an unequipped item in inventory
fn ev_unequip_item(
    commands: Commands,
    //mut ev_use_item: EventReader<EV_ItemUnequip>,
    query_items: Query<(Entity, &EquipmentBundle, &IsEquipped), With<Player>>,
    mut query_combat_stats: Query<(Entity, &mut CombatStats), With<Player>>,
) {
    // remove IsEquipped tag for renderer to pick up
    // update combat stats to reflect removed item
    // drop removed item on ground near player
}
