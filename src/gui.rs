pub use crate::prelude::*;
use bevy_ascii_terminal::Border;

pub fn draw_ui(terminal: &mut Terminal) {
    let border = bevy_ascii_terminal::Border::default();
    let border = Border::from_string(
        "┌─┐
         │ │
         └─┘",
    );
    terminal.set_border(border);
}
