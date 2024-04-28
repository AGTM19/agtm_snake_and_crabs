use bevy::{
    ecs::{component::Component, system::Resource},
    log,
};

#[derive(Resource)]
pub struct Grid {
    pub max_idx_x: i32,
    pub max_idx_y: i32,
    pub screen_height: f32,
    pub screen_width: f32,
    pub lambda: f32,
}

#[derive(Clone, Component)]
pub struct Cell {
    pub pos_x: f32,
    pub pos_y: f32,
    pub idx_x: i32,
    pub idx_y: i32,
}

impl Cell {
    pub fn set(&mut self, other: &Cell) {
        self.pos_x = other.pos_x;
        self.pos_y = other.pos_y;
        self.idx_x = other.idx_x;
        self.idx_y = other.idx_y;
    }
}

impl PartialEq for Cell {
    fn eq(&self, other: &Self) -> bool {
        self.idx_x == other.idx_x && self.idx_y == other.idx_y
    }
}

impl Grid {
    pub fn new(screen_width: f32, screen_height: f32, n_cells_x: u32, n_cells_y: u32) -> Grid {
        let n_cells_x_f32 = if n_cells_x % 2 == 0 {
            n_cells_x as f32 + 1.
        } else {
            n_cells_x as f32
        };

        let n_cells_y_f32 = if n_cells_y % 2 == 0 {
            n_cells_y as f32 + 1.
        } else {
            n_cells_y as f32
        };

        let res_x = screen_width / n_cells_x_f32;
        let res_y = screen_height / n_cells_y_f32;
        // let res = res_x.min(res_y);
        let res = res_y.min(res_x);

        // let pixels_x = res_x * n_cells_x_f32;
        // let pixels_y = res_y * n_cells_y_f32;

        let max_idx_x = ((n_cells_x_f32 - 1.) / 2.).round() as i32;
        let max_idx_y = ((n_cells_y_f32 - 1.) / 2.).round() as i32;
        // let max_idx_y = ((screen_height / 2. / resolution).floor() as i32) - 1;
        log::info!(
            "Grid: Width = {} ; Height = {}",
            max_idx_x * 2 + 3,
            max_idx_y * 2 + 3
        );
        Grid {
            max_idx_x,
            max_idx_y,
            screen_width,
            screen_height,
            lambda: res,
        }
    }

    pub fn get_cell_from_position(&self, pos_x: f32, pos_y: f32) -> Cell {
        let idx_x = (pos_x / self.lambda).round();
        let idx_y = (pos_y / self.lambda).round();
        let pos_x = idx_x * self.lambda;
        let pos_y = idx_y * self.lambda;
        let idx_x = idx_x as i32;
        let idx_y = idx_y as i32;
        Cell {
            pos_x,
            pos_y,
            idx_x,
            idx_y,
        }
    }

    pub fn get_cell_from_index(&self, idx_x: i32, idx_y: i32) -> Cell {
        let pos_x = idx_x as f32 * self.lambda;
        let pos_y = idx_y as f32 * self.lambda;
        Cell {
            pos_x,
            pos_y,
            idx_x,
            idx_y,
        }
    }

    pub fn get_wall(&self) -> Vec<Cell> {
        let mut wall =
            Vec::with_capacity((2 * 2 * self.max_idx_x + 2 * 2 * self.max_idx_y - 4 + 4) as usize);

        // top and bot rows
        for idx_x in [-self.max_idx_x, self.max_idx_x] {
            for idx_y in -self.max_idx_y..=self.max_idx_y {
                wall.push(self.get_cell_from_index(idx_x, idx_y));
            }
        }

        // left and right cols
        for idx_y in [-self.max_idx_y, self.max_idx_y] {
            for idx_x in (-self.max_idx_x + 1)..self.max_idx_x {
                wall.push(self.get_cell_from_index(idx_x, idx_y));
            }
        }

        let capacity_before = wall.capacity();
        wall.shrink_to_fit();
        let capacity_after = wall.capacity();
        assert_eq!(capacity_before, capacity_after);

        wall
    }

    pub fn is_inside_walls(&self, idx_x: i32, idx_y: i32) -> bool {
        idx_x > -self.max_idx_x
            && idx_x < self.max_idx_x
            && idx_y > -self.max_idx_y
            && idx_y < self.max_idx_y
    }
}
