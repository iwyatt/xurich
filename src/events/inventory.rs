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
    query_items: Query<(Entity, Option<&HealthPotion>), With<Item>>,
    mut query_combat_stats: Query<(Entity, &mut CombatStats)>,
) {
    for event in ev_use_item.read() {
        let (entity, potion) = query_items
            .iter()
            .filter(|(e, _)| *e == event.item)
            .map(|(e, p)| (e, p))
            .nth(0)
            .unwrap();

        // do something with the item
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
