use bevy::prelude::*;
use rltk::{Rltk, GameState};
use bevy_ascii_terminal::prelude::*;


fn main () {
    App::new()
    .add_plugins((DefaultPlugins, TerminalPlugin))
    .add_systems(Startup, setup)
    .run();
}

fn setup(mut commands: Commands) {
    // Create the terminal
    let mut terminal = Terminal::new([20,3]).with_border(Border::single_line());
    // Draw a blue "Hello world!" to the terminal
    terminal.put_string([1, 1], "Hello world!".fg(Color::BLUE));

    commands.spawn((
        // Spawn the terminal bundle from our terminal
        TerminalBundle::from(terminal),
        // Automatically set up the camera to render the terminal
        AutoCamera,
    ));
}

struct State {}
impl GameState for State {
    fn tick(&mut self, ctx : &mut Rltk) {
        ctx.cls();
        ctx.print(1, 1, "Hello Rust World");
    }
}