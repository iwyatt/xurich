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
            .nth(0)
            .unwrap();
        // TODO : Handle if there are no items at position

        // get target's inventory
        let inventory_entity = query_inventory
            .iter()
            .filter(|(a, _)| a == &e.target)
            .map(|(a, _)| a)
            .nth(0)
            .unwrap();

        // add item as child to inventory component
        commands.entity(inventory_entity).add_child(item_entity);
        println!("inventory.items.push(new_item): {:#?}", inventory_entity);
        //  remove item position as it is now only within the character's inventory and not on the map
        commands.entity(item_entity).remove::<Position>();
    }
}

pub fn ev_use_item(
    mut commands: Commands,
    mut ev_use_item: EventReader<EV_ItemUse>,
    query_items: Query<(Entity, Option<&HealthPotion>, Option<&EquipmentBundle>), With<Item>>,
    mut query_combat_stats: Query<(Entity, &mut CombatStats)>,
) {
    for event in ev_use_item.read() {
        let (entity, potion, equipment) = query_items
            .iter()
            .filter(|(e, _, _)| *e == event.item)
            .map(|(e, p, eb)| (e, p, eb))
            .nth(0)
            .unwrap();

        // do something with the item

        // equipment
        if let Some(equipment) = equipment {
            // TODO: if there is an item of the same type already equipped
            let mut stats = query_combat_stats
                .iter_mut()
                .filter(|(e, _)| event.source == *e)
                .map(|(_, c)| c)
                .nth(0)
                .unwrap();

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
