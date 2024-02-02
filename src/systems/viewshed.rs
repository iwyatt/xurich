pub use crate::prelude::*;
use rltk::{field_of_view, Point};

pub struct VisibilitySystem {}

impl Viewshed {
    fn get_visible_tiles(query_player : Query<&Player>) {
        let player = query_player.iter_mut().nth(0).unwrap();
    }
}