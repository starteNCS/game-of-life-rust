pub struct GameOfLife {
    pub field: Box<Vec<Vec<bool>>>,
    in_edit: Box<Vec<Vec<bool>>>,
    pub cols: usize,
    pub rows: usize,
}

pub enum GameOfLifeErrors {
    FieldTooSmall,
}

impl GameOfLife {
    pub fn new(rows: usize, cols: usize) -> Result<GameOfLife, GameOfLifeErrors> {
        Ok(GameOfLife {
            cols,
            rows,
            field: Box::new(vec![vec![false; cols]; rows]),
            in_edit: Box::new(vec![vec![false; cols]; rows]),
        })
    }

    #[allow(dead_code)]
    pub fn new_with_blinker(rows: usize, cols: usize) -> Result<GameOfLife, GameOfLifeErrors> {
        let mut field = Box::new(vec![vec![false; cols]; rows]);

        if cols < 5 || rows < 5 {
            return Err(GameOfLifeErrors::FieldTooSmall);
        }

        field[1][1] = true;
        field[1][2] = true;
        field[1][3] = true;

        return Ok(GameOfLife {
            cols,
            rows,
            field,
            in_edit: Box::new(vec![vec![false; cols]; rows]),
        });
    }

    pub fn run(&mut self) -> () {
        self.in_edit = self.field.clone();
        for row in 0..self.rows {
            for col in 0..self.cols {
                let neighbours = self.get_neighbour_count_for_cell(row as isize, col as isize);
                let is_alive = self.field[row][col];
                if neighbours == 3 && !is_alive {
                    self.in_edit[row][col] = true;
                } else if (neighbours == 2 || neighbours == 3) && is_alive {
                    self.in_edit[row][col] = true;
                } else if neighbours < 2 && is_alive {
                    self.in_edit[row][col] = false;
                } else if neighbours > 3 && is_alive {
                    self.in_edit[row][col] = false;
                }
            }
        }
        self.field = self.in_edit.clone();
    }

    pub fn draw(&mut self, window_x: usize, window_y: usize, width: usize, height: usize) -> () {
        let clicked_col = window_y / (height / self.rows);
        let clicked_row = window_x / (width / self.cols);

        if clicked_col >= self.cols || clicked_row >= self.rows{
            return;
        }

        self.field[clicked_row][clicked_col] = !self.field[clicked_row][clicked_col];
    }

    fn get_neighbour_count_for_cell(&self, row: isize, col: isize) -> usize {
        let directions = vec![(1, 0), (1, 1), (0, 1), (-1, 1), (-1, 0), (-1, -1), (0, -1), (1, -1)];
        let mut found: usize = 0;
        for dir in directions {
            let curr_row = row + dir.0;
            let curr_col = col + dir.1;

            // check if the current coordinates are inside the field
            if curr_row < 0 || curr_row >= self.cols as isize || curr_col < 0 || curr_col >= self.rows as isize {
                continue;
            }

            found += if self.field[curr_row as usize][curr_col as usize] { 1 } else { 0 };
        }

        return found;
    }
}