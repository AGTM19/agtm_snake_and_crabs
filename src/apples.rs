use bevy::prelude::*;

use crate::{
    geometry,
    grid::{Cell, Grid},
    snake::{Body, MoveEvent, Position},
};

use rand::Rng;

#[derive(Component)]
pub struct Apple;

#[derive(Event)]
pub struct AppleEatenEvent;

#[derive(Event)]
pub struct RelocateAppleEvent(Vec<Cell>);

pub fn spawn_apple(
    grid: Res<Grid>,
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut rng = rand::thread_rng();
    let idx_x = rng.gen_range(0..grid.max_idx_x);
    let idx_y = rng.gen_range(0..grid.max_idx_y);
    let cell = grid.get_cell_from_index(idx_x, idx_y);
    let square = geometry::get_square(
        grid.lambda,
        cell.pos_x,
        cell.pos_y,
        &mut meshes,
        &mut materials,
    );

    commands.spawn((square, cell, Apple));
}

pub fn apple_eaten(
    mut ev_move: EventReader<MoveEvent>,
    head: Query<(&Cell, &Position)>,
    apple: Query<(&Cell, &Apple)>,
    mut ev_apple_eaten: EventWriter<AppleEatenEvent>,
) {
    for MoveEvent(trace) in ev_move.read() {
        let (c, _) = apple.get_single().unwrap();

        let (head, _) = head.single();
        if *head == *c {
            ev_apple_eaten.send(AppleEatenEvent);
            return;
        }

        if let Some(_) = trace.iter().find(|tc| *tc == c) {
            ev_apple_eaten.send(AppleEatenEvent);
            return;
        }
    }
}

pub fn find_free_cells(
    head: Query<(&Cell, &Position)>,
    body: Query<(&Cell, &Body)>,
    mut ev_apple_eaten: EventReader<AppleEatenEvent>,
    mut ev_spawn_apple: EventWriter<RelocateAppleEvent>,
) {
    for _ in ev_apple_eaten.read() {
        let mut snake_cells: Vec<_> = body.iter().map(|(c, _)| c.clone()).collect();
        snake_cells.push(head.single().0.clone());
        ev_spawn_apple.send(RelocateAppleEvent(snake_cells));
    }
}

pub fn relocate_apple(
    grid: Res<Grid>,
    mut apple: Query<(&mut Transform, &mut Cell, &Apple)>,
    mut ev_spawn_apple: EventReader<RelocateAppleEvent>,
) {
    for RelocateAppleEvent(snake_cells) in ev_spawn_apple.read() {
        let mut retry = true;
        while retry {
            let mut rng = rand::thread_rng();
            let idx_x = rng.gen_range((-grid.max_idx_x + 1)..grid.max_idx_x);
            let idx_y = rng.gen_range((-grid.max_idx_y + 1)..grid.max_idx_y);
            let apple_cell = grid.get_cell_from_index(idx_x, idx_y);

            let mut apple_at_free_spot = true;
            for snake_cell in snake_cells {
                if apple_cell == *snake_cell {
                    apple_at_free_spot = false;
                    break;
                }
            }

            if apple_at_free_spot {
                let (mut t, mut c, _) = apple.get_single_mut().unwrap();
                c.set(&apple_cell);
                t.translation.x = apple_cell.pos_x;
                t.translation.y = apple_cell.pos_y;

                retry = false;
            }
        }
    }
}
