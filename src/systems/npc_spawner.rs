pub use crate::prelude::*;

// not used right now
// pub fn spawn_npc(mut commands: Commands, query_map: Query<&Map>) {
//     let map = query_map.iter().nth(0).unwrap();

//     for i in 1..map.rooms.len() {
//         let mut rng = rltk::RandomNumberGenerator::new();
//         let roll = rng.roll_dice(1, 2);
//         let glyph = match roll {
//             1 => 'G',
//             2 => 'O',
//             _ => 'X',
//         };

//         commands
//             .spawn((
//                 Position {
//                     x: map.rooms[i].center().x,
//                     y: map.rooms[i].center().y,
//                 },
//                 Renderable {
//                     glyph: glyph,
//                     fg: Color::RED,
//                     bg: Color::BLACK,
//                 },
//                 LeftWalker,
//             ))
//             .insert(Enemy)
//             .insert(Viewshed {
//                 visible_tiles: Vec::new(),
//                 range: 2,
//                 dirty: true,
//             });
//     }
// }
