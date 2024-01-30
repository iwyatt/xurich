use bevy::prelude::*;
use bevy_ascii_terminal::prelude::*;

fn main () {
    App::new()
    .add_plugins((DefaultPlugins, TerminalPlugin))
    .add_systems(Startup, setup)
    .add_systems(Update, tick)
    .run();
}

fn setup(mut commands: Commands) {
    // // Create the terminal
    // let mut terminal = Terminal::new([20,3]).with_border(Border::single_line());
    // // Draw a blue "Hello world!" to the terminal
    // terminal.put_string([1, 1], "Hello world!".fg(Color::BLUE));

    // commands.spawn((
    //     // Spawn the terminal bundle from our terminal
    //     TerminalBundle::from(terminal),
    //     // Automatically set up the camera to render the terminal
    //     AutoCamera,
    // )).insert(GameTerminal);
    let mut terminal = Terminal::new([20,3]).with_border(Border::single_line());
    //terminal.put_string([1, 1], "Hello world!".fg(Color::BLUE));
    let term_bundle = TerminalBundle::from(terminal);
    //commands.spawn_bundle(term_bundle, AutoCamera).insert(GameTerminal);
    commands.spawn((term_bundle, AutoCamera)).insert(GameTerminal);
    
}
fn tick(mut query_terminal: Query<&mut Terminal>) { //may need to add `With<GameTerminal>>`
    // https://github.com/sarkahn/bevy_roguelike/blob/2027f9966fab33e6e303a7b88b3d1e30c56683b0/src/render.rs
    // See line 44: mut q_render_terminal: Query<&mut Terminal, With<GameTerminal>>,
    let mut terminal = query_terminal.iter_mut().nth(0).unwrap();
    terminal.clear();
    terminal.put_string([1,1], "Updates")
}

#[derive(Component)]
pub struct GameTerminal;