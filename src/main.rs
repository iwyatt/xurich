use bevy::prelude::*;
use bevy_ascii_terminal::prelude::*;

fn main () {
    App::new()
    .add_plugins((DefaultPlugins, TerminalPlugin))
    .add_systems(Startup, setup)
    .add_systems(Update, (player_walk, npc_walk))
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
    let mut terminal = Terminal::new([80,60]).with_border(Border::single_line());
    //terminal.put_string([1, 1], "Hello world!".fg(Color::BLUE));
    let term_bundle = TerminalBundle::from(terminal);
    //commands.spawn_bundle(term_bundle, AutoCamera).insert(GameTerminal);
    commands.spawn((term_bundle, AutoCamera)).insert(GameTerminal);


    let mut gs = State {
        ecs: World::new()
    };
    
    commands.spawn(
        (
            Position { x: 1, y:1},
            Renderable {glyph: '@', fg: Color::YELLOW, bg: Color::BLACK}
        )
    ).insert(Player);

    for i in 0..2 {
        commands.spawn(
            (
                Position { x: i, y:2},
                Renderable {glyph: 'G', fg: Color::RED, bg: Color::BLACK},
                LeftWalker,
            )
        ).insert(Enemy);
    }                
}

fn tick(mut query_terminal: Query<&mut Terminal>, query_entities: Query<(&Position, &Renderable)>) { //may need to add `With<GameTerminal>>`
    // https://github.com/sarkahn/bevy_roguelike/blob/2027f9966fab33e6e303a7b88b3d1e30c56683b0/src/render.rs
    // See line 44: mut q_render_terminal: Query<&mut Terminal, With<GameTerminal>>,
    let mut terminal = query_terminal.iter_mut().nth(0).unwrap();
    terminal.clear();

    // for (pos, rend) in &query_entities {
    //     terminal.put_char([pos.x, pos.y], rend.glyph)
    //query_entities.iter().for_each(|position, renderable)| terminal.put_char([p.x, p.y], r.glyph.fg(r.fg).bg(r.bg)));
    //println!("{:#?}", query_entities);
    query_entities.iter().for_each(|(pos, rend)| terminal.put_char([pos.x, pos.y], rend.glyph.fg(rend.fg).bg(rend.bg)));
    //terminal.put_string([4,1], "Updates")
}

fn npc_walk(mut query_walkers: Query<(&mut Position, &Enemy)>) {
    query_walkers.iter_mut().for_each(|(mut p,_)| {
        p.x = p.x + 1;
        if p.x <= 1 {
            p.x = 1;
        }
    }
    );
}

fn player_walk(input: Res<Input<KeyCode>>, mut player_pos: Query<(&Player, &mut Position)>){
    let move_input = read_movement(input);
    if move_input.cmpeq(IVec2::ZERO).all() {
        return;
    }

    let (player, mut pos) = player_pos.iter_mut().nth(0).map(|(player, mut pos)|(player, pos)).unwrap();
    
    let curr = IVec2::new(pos.x, pos.y);
    let next = curr + move_input;
    pos.x = next.x;
    pos.y = next.y;
}

// an IVec2 is a 2-dimensional vector (direction and distance for x and y both)
fn read_movement(input: Res<Input<KeyCode>>) -> IVec2 {
    let mut p = IVec2::ZERO;

    if input.just_pressed(KeyCode::Numpad1) || input.just_pressed(KeyCode::Z) {
        p.x = -1;
        p.y = -1;
    }
    if input.just_pressed(KeyCode::Numpad2) || input.just_pressed(KeyCode::X) || input.just_pressed(KeyCode::Down) {
        p.y = -1;
    }
    if input.just_pressed(KeyCode::Numpad3) || input.just_pressed(KeyCode::C) {
        p.x = 1;
        p.y = -1;
    }
    if input.just_pressed(KeyCode::Numpad4) || input.just_pressed(KeyCode::A) || input.just_pressed(KeyCode::Left) {
        p.x = -1;
    }
    if input.just_pressed(KeyCode::Numpad6) || input.just_pressed(KeyCode::D) || input.just_pressed(KeyCode::Right) {
        p.x = 1;
    }
    if input.just_pressed(KeyCode::Numpad7) || input.just_pressed(KeyCode::Q) {
        p.x = -1;
        p.y = 1;
    }
    if input.just_pressed(KeyCode::Numpad8) || input.just_pressed(KeyCode::W) || input.just_pressed(KeyCode::Up) {
        p.y = 1;
    }
    if input.just_pressed(KeyCode::Numpad9) || input.just_pressed(KeyCode::E) {
        p.x = 1;
        p.y = 1;
    }
    p
}

#[derive(Component)]
pub struct GameTerminal;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct LeftWalker;

#[derive(Component)]
struct Position {
    pub x: i32,
    pub y: i32,
}

#[derive(Component)]
struct Renderable {
    pub glyph: char,
    pub fg: Color,
    pub bg: Color
}

struct State {
    ecs: World
}
