pub use crate::prelude::*;

#[derive(Component)]
pub struct GameTerminal;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct LeftWalker;

#[derive(Component, PartialEq, Clone)]
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
