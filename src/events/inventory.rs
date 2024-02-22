pub use crate::prelude::*;

pub fn ev_pickup_item(
    mut commands: Commands,
    query_inventory: Query<(Entity, &Inventory)>,
    query_items: Query<(Entity, &Position), With<Item>>,
    //mut query_entity_items: Query<(Entity, (With<Position>, With<Item>))>,
    //query_items: Query<(&Item, &Position)>,
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

        // add item to target's inventory...
        // ... but first, manually Copy() the item from a reference to create a new item
        // TODO : Figure out a better way!
        // let new_item: Item = Item {
        //     name: Name(item.name.0.to_string()),
        // };
        commands.entity(inventory_entity).add_child(item_entity);
        //inventory.items.push(new_item); // TODO : change this so that only a single item is retrieved at a time
        println!("inventory.items.push(new_item): {:#?}", inventory_entity);
        //  remove item position as it is now only within the character's inventory and not on the map
        commands.entity(item_entity).remove::<Position>();
    }
}