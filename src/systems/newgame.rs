use crate::prelude::*;
use crate::systems::*;
use rltk::RandomNumberGenerator;

pub fn init_new_game(
    mut commands: Commands,
    gamestate: Res<State<GameLoopState>>,
    mut next_state: ResMut<NextState<GameLoopState>>,
    entities: Query<Entity>,
    mut query_terminals: Query<&mut Terminal>,
) {
    // new games dont have border for some reason, 
    // need to figure out why and how to add them back
    // for mut terminal in query_terminals.iter_mut() {
    //     terminal.set_border(Border::single_line());
    // }

    // begin world building // TODO: Split this off into second setup function
    let mut myrng = RNG(RandomNumberGenerator::seeded(RNG_SEED)); // TODO: Change this to be a u64 hash of player name/year
                                                                  //commands.spawn(myrng);
                                                                  // set the game state
    let game_state = crate::components::GameState {
        runstate: RunState::Running,
    };
    commands.spawn(game_state);

    //let (map, player_start_position, mob_start_positions, item_start_positions) = Map::random();
    let mapgen = MapGenerator::default();
    let (map, player_start_position, mob_start_positions, item_start_positions) =
        Map::random(&mut myrng);
    // TODO: move player and npc spawn into map generation
    // spawn player on map in world
    spawner::spawn_player(&mut commands, player_start_position);

    // spawn npc bundle
    mob_start_positions.iter().for_each(|pos| {
        spawner::spawn_random_mob(
            &mut commands,
            pos.clone(),
            &mut myrng,
            WorldPosition { x: 0, y: 0, z: 0 },
        )
    });

    // spawn item bundle
    item_start_positions.iter().for_each(|pos| {
        spawner::spawn_random_item(
            &mut commands,
            pos.clone(),
            &mut myrng,
            WorldPosition { x: 0, y: 0, z: 0 },
        )
    });

    // add map to worldmap resource
    let mut worldmap = WorldMap {
        maps: Vec::<Map>::with_capacity(WORLD_MAP_HEIGHT as usize * WORLD_MAP_WIDTH as usize),
    };
    //let mapidx = world_xy_idx(map.world_pos.x, map.world_pos.y);
    worldmap.maps.push(map);
    commands.insert_resource(worldmap);
    //commands.spawn(map);
    commands.spawn(myrng);

    next_state.set(GameLoopState::PlayerTurn);
}
