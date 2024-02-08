pub use crate::prelude::*;

#[derive(Component)]
pub struct GameTerminal;

#[derive(Component)]
pub struct Player;

#[derive(Component, Debug)]
pub struct Enemy;

#[derive(Component)]
pub struct NPC_AI {
    pub state: NPC_State,
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum NPC_State {
    Inactive, // dont execute AI
    Alerted,
    Active,
    Passive, // idle
}

#[derive(Component)]
pub struct LeftWalker;

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
pub struct Name {
    pub name: String,
}

#[derive(Component, Debug)]
pub struct BlocksTile;

#[derive(Component, Debug)]
pub struct CombatStats {
    pub max_hp: i32,
    pub hp: i32,
    pub defense: i32,
    pub power: i32,
}
