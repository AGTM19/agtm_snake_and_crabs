use crate::{
    apples::AppleEatenEvent,
    geometry::{self, get_square},
    grid::{Cell, Grid},
    score::{Score, ScoreIncreasedEvent},
    Config, GameOverEvent,
};

use bevy::prelude::*;

#[derive(Component)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
    direction: Direction,
}

#[derive(Component)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Resource)]
pub struct Speed {
    in_blocks: f32,
    res: f32,
}

impl Speed {
    pub fn in_blocks(&self) -> f32 {
        self.in_blocks
    }

    pub fn in_pixels(&self) -> f32 {
        self.in_blocks * self.res
    }

    fn set_speed_in_blocks(&mut self, speed: f32) {
        self.in_blocks = speed;
    }
}

#[derive(Component)]
pub struct Body(u32);

enum Direction {
    Vertical,
    Horizontal,
}

#[derive(Event)]
pub struct MoveEvent(pub Vec<Cell>);

pub fn spawn_snake(
    grid: Res<Grid>,
    config: Res<Config>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let res = grid.lambda;
    let speed = Speed {
        res,
        in_blocks: config.initial_speed,
    };

    // Spawn Snake
    let mut cell = grid.get_cell_from_position(0., 0.);
    let square = geometry::get_square(res, cell.pos_x, cell.pos_y, &mut meshes, &mut materials);
    commands.spawn((
        square,
        Position {
            x: cell.pos_x,
            y: cell.pos_y,
        },
        Velocity {
            x: speed.in_pixels(),
            y: 0.,
            direction: Direction::Horizontal,
        },
        cell.clone(),
    ));

    for i in 1..config.initial_bodylength {
        // Spawn body
        // ... First Fragment
        cell = grid.get_cell_from_index(cell.idx_x - 1, cell.idx_y);
        let square = geometry::get_square(res, cell.pos_x, cell.pos_y, &mut meshes, &mut materials);
        commands.spawn((square, Body(i), cell.clone()));
    }

    commands.insert_resource(speed);
}

pub fn move_snake(
    time: Res<Time>,
    grid: Res<Grid>,
    mut ev_move: EventWriter<MoveEvent>,
    mut head: Query<(&mut Transform, &mut Position, &Velocity, &mut Cell)>,
) {
    // Move Head
    let delta_t = time.delta_seconds();
    let head = head.get_single_mut().unwrap();

    let mut transform = head.0;
    let mut pos = head.1;
    let vel = head.2;
    pos.x += vel.x * delta_t;
    pos.y += vel.y * delta_t;

    let new_cell = grid.get_cell_from_position(pos.x, pos.y);

    let mut head_cell = head.3;
    let old_cell = head_cell.clone();
    head_cell.set(&new_cell);

    transform.translation.x = new_cell.pos_x;
    transform.translation.y = new_cell.pos_y;

    if !head_cell.eq(&old_cell) {
        let mut trace = vec![old_cell.clone()];

        if old_cell.idx_x != head_cell.idx_x {
            for idx_x in get_range(old_cell.idx_x, new_cell.idx_x) {
                trace.push(grid.get_cell_from_index(idx_x, old_cell.idx_y))
            }
        }
        if old_cell.idx_y != head_cell.idx_y {
            for idx_y in get_range(old_cell.idx_y, new_cell.idx_y) {
                trace.push(grid.get_cell_from_index(old_cell.idx_x, idx_y))
            }
        }

        ev_move.send(MoveEvent(trace));
    }
}

fn get_range(old: i32, new: i32) -> Box<dyn Iterator<Item = i32>> {
    let direction = if new - old > 0 { 1 } else { -1 };

    if direction > 0 {
        Box::new((old + direction)..new)
    } else {
        Box::new(((new - direction)..old).rev())
    }
}

pub fn move_body(
    mut ev_move: EventReader<MoveEvent>,
    mut body: Query<(&mut Transform, &mut Cell, &Body)>,
) {
    for ev in ev_move.read() {
        let mut trace = ev.0.clone();

        let mut sorted: Vec<_> = body.iter_mut().collect();
        sorted.sort_by(|(_, _, Body(b1)), (_, _, Body(b2))| b1.cmp(b2));

        let mut target_index = trace.len() - 1;

        for (t, c, _) in &mut sorted {
            let new_cell = trace[target_index].clone();
            trace[target_index].set(c);

            if target_index != 0 {
                target_index -= 1;
            } else {
                target_index = trace.len() - 1;
            }

            c.set(&new_cell);

            t.translation.x = c.pos_x;
            t.translation.y = c.pos_y;
        }
    }
}

pub fn steer_snake(
    speed: Res<Speed>,
    mut snake: Query<&mut Velocity, With<Position>>,
    keycode: Res<ButtonInput<KeyCode>>,
) {
    keycode
        .get_just_pressed()
        .for_each(|k| handle_steering(&speed, &mut snake, k));
}

fn handle_steering(
    speed: &Res<Speed>,
    snake: &mut Query<&mut Velocity, With<Position>>,
    keycode: &KeyCode,
) {
    let mut vel = snake.get_single_mut().unwrap();

    match keycode {
        KeyCode::KeyW | KeyCode::ArrowUp => {
            if let Direction::Horizontal = vel.direction {
                vel.x = 0.;
                vel.y = speed.in_pixels();
                vel.direction = Direction::Vertical
            }
        }
        KeyCode::KeyA | KeyCode::ArrowLeft => {
            if let Direction::Vertical = vel.direction {
                vel.x = -speed.in_pixels();
                vel.y = 0.;
                vel.direction = Direction::Horizontal
            }
        }
        KeyCode::KeyS | KeyCode::ArrowDown => {
            if let Direction::Horizontal = vel.direction {
                vel.x = 0.;
                vel.y = -speed.in_pixels();
                vel.direction = Direction::Vertical
            }
        }
        KeyCode::KeyD | KeyCode::ArrowRight => {
            if let Direction::Vertical = vel.direction {
                vel.x = speed.in_pixels();
                vel.y = 0.;
                vel.direction = Direction::Horizontal
            }
        }
        _ => {}
    }
}

pub fn snake_hits_itself(
    mut ev_move: EventReader<MoveEvent>,
    head: Query<(&Cell, &Position)>,
    body: Query<(&Cell, &Body)>,
    mut ev_game_over: EventWriter<GameOverEvent>,
) {
    for ev in ev_move.read() {
        for (body_cell, _) in &body {
            for trace_cell in &ev.0 {
                if body_cell == trace_cell {
                    ev_game_over.send(GameOverEvent());
                }
            }
        }
    }

    let (head_cell, _) = head.get_single().unwrap();
    for (body_cell, _) in &body {
        if body_cell == head_cell {
            ev_game_over.send(GameOverEvent());
        }
    }
}

pub fn snake_grows(
    mut commands: Commands,
    config: Res<Config>,
    grid: Res<Grid>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    body: Query<(&Cell, &Body)>,
    mut ev_eaten: EventReader<AppleEatenEvent>,
) {
    for _ in ev_eaten.read() {
        let (last, Body(n)) = body.iter().last().unwrap();
        for i in (n + 1)..(n + 1 + config.n_elements_per_apple) {
            let square = get_square(
                grid.lambda,
                last.pos_x,
                last.pos_y,
                &mut meshes,
                &mut materials,
            );
            commands.spawn((square, Body(i), last.clone()));
        }
    }
}

pub fn speed_up(
    config: Res<Config>,
    score: Res<Score>,
    mut speed: ResMut<Speed>,
    mut ev_score_increased: EventReader<ScoreIncreasedEvent>,
) {
    for _ in ev_score_increased.read() {
        if score.n_apples % (3 * config.score_increment) == 0 {
            let old_speed = speed.in_blocks();
            speed.set_speed_in_blocks(old_speed * 1.1);
        }
    }
}
