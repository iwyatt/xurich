pub use crate::prelude::*;

#[derive(Component)]
pub struct GameTerminal;

#[derive(Component)]
pub struct Player;

#[derive(Bundle)]
pub struct PlayerBundle {
    pub name: Name,
    pub marker: Player,
    pub viewshed: Viewshed,
    pub position: Position,
    pub renderable: Renderable,
    pub stats: CombatStats,
    // We can nest/include another bundle.
    // Add the components for a standard Bevy Sprite:
    // sprite: SpriteSheetBundle,
}

impl Default for PlayerBundle {
    fn default() -> Self {
        Self {
            name: Name("Hero".into()),
            marker: Player,
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
        }
    }
}

#[derive(Component, PartialEq, Clone, Debug)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component, PartialEq, Clone)]
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

#[derive(Component, Copy, Clone, Debug)]
pub struct GameState {
    //pub ecs: World,
    pub runstate: RunState,
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum RunState {
    Paused,
    Running,
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
