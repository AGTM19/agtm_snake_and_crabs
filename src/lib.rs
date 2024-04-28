mod apples;
mod bloom_example;
mod geometry;
mod grid;
mod playground;
mod score;
pub mod snake;

use bevy::{
    app::PluginGroupBuilder,
    // core_pipeline::bloom::BloomSettings,
    core_pipeline::tonemapping::Tonemapping,
    prelude::*,
    window::WindowMode,
};
use grid::Grid;
use snake::{Position, Velocity};

use crate::{
    apples::{AppleEatenEvent, RelocateAppleEvent},
    score::ScoreIncreasedEvent,
    snake::MoveEvent,
};

#[derive(Resource)]
pub struct Config {
    pub n_vertical_cells: u32,   // Should be uneven
    pub n_horizontal_cells: u32, // Should be uneven
    pub initial_bodylength: u32,
    pub initial_speed: f32,
    pub n_elements_per_apple: u32,
    pub score_increment: u32,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            n_vertical_cells: 25,
            n_horizontal_cells: 45,
            initial_bodylength: 3,
            initial_speed: 10.,
            n_elements_per_apple: 1,
            score_increment: 1,
        }
    }
}

fn set_config(mut commands: Commands) {
    commands.insert_resource(Config {
        // n_vertical_cells: 95,
        initial_bodylength: 10,
        initial_speed: 10.,
        ..default()
    });
}

#[derive(Event)]
pub struct GameOverEvent();

pub fn run() {
    println!("Welcome to Snakes and Crabs.");
    App::new()
        .add_plugins(get_full_screen_default_plugins())
        .add_event::<MoveEvent>()
        .add_event::<GameOverEvent>()
        .add_event::<AppleEatenEvent>()
        .add_event::<RelocateAppleEvent>()
        .add_event::<ScoreIncreasedEvent>()
        .add_systems(
            Startup,
            (
                set_config,
                playground::spawn_playing_ground,
                (
                    setup_camera,
                    snake::spawn_snake,
                    apples::spawn_apple,
                    score::spawn_score,
                ),
            )
                .chain(),
        )
        .add_systems(
            Update,
            (
                (
                    snake::speed_up,
                    snake::steer_snake,
                    snake::move_snake,
                    playground::snake_hits_wall,
                    snake::snake_hits_itself,
                    snake::move_body,
                )
                    .chain(),
                snake::snake_grows,
            ),
        )
        .add_systems(
            Update,
            (
                apples::apple_eaten,
                apples::find_free_cells,
                apples::relocate_apple,
            )
                .chain(),
        )
        .add_systems(Update, (score::increment_score, score::render_score))
        .add_systems(Update, game_over_teleport_to_center)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            camera: Camera {
                hdr: true, // 1. HDR is required for bloom
                ..default()
            },
            tonemapping: Tonemapping::TonyMcMapface, // 2. Using a tonemapper that desaturates to white is recommended
            ..default()
        },
        // BloomSettings::default(), // 3. Enable bloom for the camera
    ));
}

fn get_full_screen_default_plugins() -> PluginGroupBuilder {
    DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            resizable: false,
            mode: WindowMode::BorderlessFullscreen,
            ..default()
        }),
        ..default()
    })
}

fn game_over_teleport_to_center(
    grid: Res<Grid>,
    mut snake: Query<(&mut Position, &Velocity)>,
    mut ev_game_over: EventReader<GameOverEvent>,
) {
    for _ in ev_game_over.read() {
        let snake = snake.get_single_mut().unwrap();

        let mut pos = snake.0;

        let cell = grid.get_cell_from_position(0., 0.);
        pos.x = cell.pos_x;
        pos.y = cell.pos_y;
    }
}
