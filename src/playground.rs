use bevy::prelude::*;

use crate::{
    geometry,
    grid::Grid,
    snake::{Position, Velocity},
    Config, GameOverEvent,
};

pub fn spawn_playing_ground(
    mut commands: Commands,
    config: Res<Config>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    window: Query<&Window>,
) {
    // Define Grid
    let factor = 1.1;
    let resolution = &window.get_single().unwrap().resolution;
    let pixels_x = (resolution.width() / factor).floor();
    let pixels_y = (resolution.height() / factor).floor();
    // let res = (res_x, res_y).min();
    let grid = Grid::new(
        pixels_x,
        pixels_y,
        config.n_horizontal_cells,
        config.n_vertical_cells,
    );

    // Spawn Walls
    for cell in grid.get_wall() {
        let square = geometry::get_square(
            grid.lambda,
            cell.pos_x,
            cell.pos_y,
            &mut meshes,
            &mut materials,
        );
        commands.spawn(square);
    }

    commands.insert_resource(grid);
}

pub fn snake_hits_wall(
    grid: Res<Grid>,
    snake: Query<(&Position, &Velocity)>,
    mut ev_game_over: EventWriter<GameOverEvent>,
) {
    let snake = snake.get_single().unwrap();

    let pos = snake.0;
    let cell = grid.get_cell_from_position(pos.x, pos.y);
    if !grid.is_inside_walls(cell.idx_x, cell.idx_y) {
        ev_game_over.send(GameOverEvent());
    }
}
