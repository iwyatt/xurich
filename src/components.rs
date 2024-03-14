use crate::prelude::*;
use rltk::*;

#[derive(Component, Event)]
pub struct EV_OpenInventoryTerminal;

#[derive(Component, Event)]
pub struct EV_CloseInventoryTerminal;

#[derive(Component)]
pub struct InventoryTerminal;

#[derive(Component)]
pub struct InitialEntity;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameLoopState {
    #[default]
    Initialization,
    NewGame,
    // Main Game Loop
    PlayerTurn,
    // Player Turn Interstitial States:
    Inventory,
    // LevelUp
    // ViewMap
    // Enemy/NPC/MOB Turn:
    NPCTurn,
    // End Game States:
    Defeat,
    // Victory,
}

// TODO: Remove Run State and GameState when GameLoopState is fully implemented
#[derive(PartialEq, Copy, Clone, Debug)]
pub enum RunState {
    Paused,
    Running,
    GameOver,
}

#[derive(Component, Copy, Clone, Debug)]
pub struct GameState {
    //pub ecs: World,
    pub runstate: RunState,
}

#[derive(Component)]
pub struct MapTerminal;

#[derive(Component)]
pub struct StatBarTerminal;

// #[derive(Component)]
// pub struct UI_Terminal;

// Actors - players, NPCs (enemies and friendly), interactables
#[derive(Component, Debug)]
pub struct Actors {
    pub actors: Vec<Actors>,
}

#[derive(Component, Debug)]
pub struct Actor;

#[derive(Component)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub name: Name,
    //pub marker: Player,
    pub viewshed: Viewshed,
    pub position: Position,
    pub renderable: Renderable,
    pub stats: CombatStats,
    pub markers: (Player, Actor), // We can nest/include another bundle.
    // Add the components for a standard Bevy Sprite:
    // sprite: SpriteSheetBundle,
    pub world_pos: WorldPosition,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            name: Name("Hero".into()),
            //marker: Player,
            viewshed: Viewshed {
                visible_tiles: Vec::new(),
                range: 3,
                dirty: true,
            },
            position: Position {
                x: MAP_WIDTH / 2,
                y: MAP_HEIGHT / 2,
            },
            renderable: Renderable {
                glyph: '@',
                fg: Color::LIME_GREEN,
                bg: Color::BLACK,
            },
            stats: CombatStats {
                max_hp: 30,
                hp: 30,
                defense: 2,
                power: 5,
            },
            markers: (Player, Actor),
            world_pos: WorldPosition { x: 0, y: 0, z: 0 },
        }
    }
}

#[derive(Component, PartialEq, Clone, Debug)]
pub struct WorldPosition {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

#[derive(Component, PartialEq, Clone, Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component, PartialEq, Clone, Debug)]
pub struct Renderable {
    pub glyph: char,
    pub fg: Color,
    pub bg: Color,
}

#[derive(Component)] // TODO : Implement viewshed so NPCs can us it
pub struct Viewshed {
    pub visible_tiles: Vec<rltk::Point>,
    pub range: i32,
    pub dirty: bool,
}

#[derive(Component, Debug)]
pub struct Name(pub String);

#[derive(Component, Debug)]
pub struct BlocksTile;

#[derive(Component, Debug)]
pub struct CombatStats {
    pub max_hp: i32,
    pub hp: i32,
    pub defense: i32,
    pub power: i32,
}

// event struct
#[derive(Component, Debug, Event)]
pub struct CombatAttack {
    pub source: Entity,
    pub target: Entity,
    // should these be on a weapon struct?
    // pub range: i32,
    pub damage: (i32, i32), // eg xdx eg 1d6,
                            // pub attack_type: AttackType,
                            // weapon name
                            // weapon type
                            // chance to hit
                            // damage type(s)
}

// #[derive(Component, Debug)]
// pub enum AttackType {
//     Magic,
//     Environmental,
//     Item
// }

#[derive(Component, Debug)]
pub struct Item;

// #[derive(Debug)]
// pub enum ItemType {
//     Weapon,
//     Armor,
//     Consumable,
// }

#[derive(Component, Debug)]
pub struct Weapon;

#[derive(Component, Debug)]
pub struct Armor;

#[derive(Component, Debug)]
pub struct IsEquipped;

#[derive(Component, Debug)]
pub struct EquipmentBundle {
    pub stat_bonus: CombatStats, //these are the stats that get added to the player's combat stats
                                 // durability
                                 // weight
                                 // volume
}

#[derive(Component, Debug)]
pub struct Consumable;

#[derive(Component, Debug)]
pub struct HealthPotion {
    pub heal_amount: i32,
}

#[derive(Component, Debug)]
pub struct Inventory;

#[derive(Component, Clone)]
pub struct RNG(pub RandomNumberGenerator);

// event struct
#[derive(Component, Debug, Event)]
pub struct EV_ItemPickUp {
    pub target: Entity,
    pub position: Position,
}

// event struct
#[derive(Component, Debug, Event)]
pub struct EV_ItemUse {
    pub source: Entity,
    pub item: Entity,
}
