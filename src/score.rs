use bevy::prelude::*;

use crate::{apples::AppleEatenEvent, grid::Grid, snake::Speed, Config};

#[derive(Resource)]
pub struct Score {
    pub n_apples: u32,
    pub score: f32,
}

impl Default for Score {
    fn default() -> Self {
        Score {
            n_apples: 0,
            score: 0.,
        }
    }
}

#[derive(Event)]
pub struct ScoreIncreasedEvent;

pub fn spawn_score(mut commands: Commands, grid: Res<Grid>) {
    // spawn resource
    commands.insert_resource(Score { ..default() });

    // spawn text
    let cell_top_left = grid.get_cell_from_index(-grid.max_idx_x + 3, -grid.max_idx_y + 1);
    let pos_x = grid.screen_width / 2. + cell_top_left.pos_x;
    let pos_y = grid.screen_height / 2. + cell_top_left.pos_y;
    commands.spawn(
        TextBundle::from_section(
            format!("Score: {}", 0),
            TextStyle {
                font_size: grid.lambda * 1.30,
                color: Color::rgb(1.00, 0.34, 0.20),
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(pos_y),
            left: Val::Px(pos_x),
            height: Val::Px(grid.lambda),
            ..default()
        }),
    );
}

pub fn increment_score(
    config: Res<Config>,
    mut score: ResMut<Score>,
    speed: Res<Speed>,
    mut ev_apple_eaten: EventReader<AppleEatenEvent>,
    mut ev_score_increased: EventWriter<ScoreIncreasedEvent>,
) {
    for _ in ev_apple_eaten.read() {
        score.n_apples += config.score_increment;
        score.score += config.score_increment as f32 * speed.in_blocks() * speed.in_blocks();
        ev_score_increased.send(ScoreIncreasedEvent);
    }
}

pub fn render_score(score: Res<Score>, mut text: Query<&mut Text>) {
    let mut text = text.single_mut();
    let text = &mut text.sections[0].value;

    *text = format!(
        "Apples: {:0>7} | Score: {:0>10}",
        score.n_apples, score.score as u32
    );
}
