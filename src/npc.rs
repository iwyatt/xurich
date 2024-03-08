// load crate prelude
//use crate::components;
use crate::prelude::*;

#[derive(Bundle)]
pub struct NPCBundle {
    pub name: crate::components::Name,
    pub marker: NPC_Type,
    pub viewshed: Viewshed,
    pub position: Position,
    pub world_pos: WorldPosition,
    pub renderable: Renderable,
    pub stats: CombatStats,
    pub ai: NPC_AI,
    // We can nest/include another bundle.
    // Add the components for a standard Bevy Sprite:
    // sprite: SpriteSheetBundle,
}

impl Default for NPCBundle {
    fn default() -> Self {
        Self {
            name: Name("NPC".into()),
            marker: NPC_Type::Enemy,
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
                glyph: 'X',
                fg: Color::YELLOW,
                bg: Color::BLACK,
            },
            stats: CombatStats {
                max_hp: 5,
                hp: 5,
                defense: 0,
                power: 5,
            },
            ai: NPC_AI {
                state: NPC_AI_State::Inactive,
            },
            world_pos: WorldPosition { x: 0, y: 0, z: 0 },
        }
    }
}

#[derive(Component, Debug)]
pub enum NPC_Type {
    Enemy,
}

#[derive(Component)]
pub struct NPC_AI {
    pub state: NPC_AI_State,
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub enum NPC_AI_State {
    Inactive, // dont execute AI
    Active,
}

#[derive(Component, Debug)]
pub struct Enemy;
