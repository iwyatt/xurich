use crate::prelude::*;

// game state: Inventory

// open inventory
pub fn ev_open_inventory(
    mut commands: Commands,
    mut ev_open_inv: EventReader<EV_OpenInventoryTerminal>,
    //gamestate: Res<State<GameLoopState>>,
    //mut query_game_state: Query<&mut components::GameState>,
    mut next_state: ResMut<NextState<GameLoopState>>,
    mut query_map_terminal: Query<(Entity, &mut Terminal), With<MapTerminal>>,
    //mut ev_open_inventory: EventWriter<EV_OpenInventoryTerminal>,
) {
    for event in ev_open_inv.read() {
        println!("ev_open_inv");

        let mut map_terminal = query_map_terminal.single_mut();
        // map_terminal.clear();
        // create a terminal for inventory
        // define the play terminal
        let term_size = [MAP_WIDTH, MAP_HEIGHT];
        // TODO: BUG: I suspect that the above is causing an issue with NPC_AI.rs pathing when player is on bottom row of map

        let terminal =
            Terminal::new(term_size).with_border(Border::single_line().with_title("Inventory"));
        let term_bundle = TerminalBundle::from(terminal);

        // create the inventory cursor
        let cursor = InventoryCursor { pos: 0 };
        commands.spawn(cursor);

        // create the terminal and camera
        commands
            .spawn((term_bundle, AutoCamera))
            .insert(InventoryTerminal);

        //let mut game_state = query_game_state.iter_mut().nth(0).unwrap();
        //game_state.runstate = RunState::Running;
        next_state.set(GameLoopState::Inventory);
        commands.entity(map_terminal.0).despawn_recursive();
    }
}

// close inventory
pub fn ev_close_inventory(
    mut commands: Commands,
    mut query_cursor: Query<Entity, With<InventoryCursor>>,
    mut ev_close_inv: EventReader<EV_CloseInventoryTerminal>,
    mut query_inv_terminal: Query<(Entity, &mut Terminal), With<InventoryTerminal>>,
    mut next_state: ResMut<NextState<GameLoopState>>,
) {
    for event in ev_close_inv.read() {
        println!("ev_close_inv");
        // define the play terminal
        let term_size = [MAP_WIDTH, MAP_HEIGHT];

        let terminal = Terminal::new(term_size).with_border(Border::single_line());
        let term_bundle = TerminalBundle::from(terminal);

        // create the terminal and camera
        commands
            .spawn((term_bundle, AutoCamera, InitialEntity))
            .insert(InitialEntity)
            .insert(MapTerminal);

        let mut inv_terminal = query_inv_terminal.single_mut();
        commands.entity(inv_terminal.0).despawn_recursive();

        // despawn the cursor
        // TODO: if we added the cursor as a child of the terminal when opening it, would the above despawn it?
        for mut cursor in query_cursor.iter_mut() {
            commands.entity(cursor).despawn_recursive();
            //println!("remove cursor!");
        }

        // set next game state
        next_state.set(GameLoopState::NPCTurn);
    }
}

// game state: Player Turn
// pickup item
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

pub fn ev_drop_item(
    mut ev_drop_item: EventReader<EV_ItemDrop>,
    mut ev_unequip_item: EventWriter<EV_ItemUnequip>,
    mut commands: Commands,
    // query_inventory: Query<(Entity, &Inventory)>,
    // query_items: Query<(Entity, &Position), With<Item>>,
    // query_player_position: Query<&Position, With<Player>>,
    query_equipped: Query<
        (
            Entity,
            Option<&Weapon>,
            Option<&Armor>,
            Option<&EquipmentBundle>,
        ),
        With<IsEquipped>,
    >,
) {
    for event in ev_drop_item.read() {
        // if item is equipped, unequip it first
        if query_equipped.contains(event.item) {
            ev_unequip_item.send(EV_ItemUnequip {
                actor: event.actor,
                item: event.item,
            });
        }

        // remove the item's parent, which is the inventory it is currently attached to
        commands.entity(event.item).remove_parent();

        // give the item a position
        // which, if it also has a Renderable, then it will be rendered
        commands.entity(event.item).insert(Position {
            x: event.position.x,
            y: event.position.y,
        });
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
            println!("equipment bundle: {:#?}", equipment);
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
                        //commands.entity(e.0).remove_parent();

                        // drop item at player's position
                        //let pos = query_player_position.single();
                        //commands.entity(e.0).insert(Position { x: pos.x, y: pos.y });
                    }
                }

                // if both the new item is a weapon and also there is a weapon equipped
                println!("isarmor: {:#?}", isarmor);
                println!("e.3.is_some(): {:#?}", e.3);
                if isarmor.is_some() && e.3.is_some() {
                    // if the old item weapon has a equipment bundle
                    if let Some(item) = e.3 {
                        //remove item by lowering combat stats
                        println!("removing old armor pre-removal stats: {:#?}", stats);
                        stats.power -= item.stat_bonus.power;
                        stats.defense -= item.stat_bonus.defense;
                        stats.max_hp -= item.stat_bonus.max_hp;
                        println!("removing old armor post-removal stats: {:#?}", stats);

                        // remove the isequipped tag
                        commands.entity(e.0).remove::<IsEquipped>();

                        // remove the parent inventory
                        //commands.entity(e.0).remove_parent();

                        // drop item at player's position
                        //let pos = query_player_position.single();
                        //commands.entity(e.0).insert(Position { x: pos.x, y: pos.y });
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

// process event requests to unequip an item
pub fn ev_unequip_item(
    mut commands: Commands,
    mut ev_unequip_item: EventReader<EV_ItemUnequip>,
    //mut query_combat_stats: Query<(Entity, &mut CombatStats)>,
    mut query_combat_stats: Query<&mut CombatStats>,
    query_equip_bundle: Query<&EquipmentBundle>,
) {
    for event in ev_unequip_item.read() {
        let item_stats = query_equip_bundle
            .get(event.item)
            .map(|equipment| &equipment.stat_bonus)
            .unwrap();

        // get the combat stats for the player and the item being unequipped
        //  - note that .get_many_mut() returns entities in the order in which you specify
        let mut player_stats = query_combat_stats.get_mut(event.actor).unwrap();
        // let [mut player_stats, item_stats] = query_combat_stats
        //     .get_many_mut([event.actor, event.item])
        //     .unwrap();

        // subtract stats of item from the player's stats
        println!("player stats before removing item: {:#?}", player_stats);
        player_stats.power -= item_stats.power;
        player_stats.defense -= item_stats.defense;
        player_stats.max_hp -= item_stats.max_hp;
        println!("player stats after removing item:  {:#?}", player_stats);

        // remove the isequipped tag
        commands.entity(event.item).remove::<IsEquipped>();
    }
}
