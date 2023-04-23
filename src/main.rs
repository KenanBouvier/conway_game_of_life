use macroquad::prelude::*;
use std::collections::HashSet;

const GRID_SIZE: usize = 200;
const SQUARE_WIDTH: f32 = 20f32;
const PADDING: f32 = 1f32;
const MULTIPLIER: i32 = (SQUARE_WIDTH + PADDING) as i32;

#[derive(Copy, Clone, Debug)]
struct GridType([[Cell; GRID_SIZE]; GRID_SIZE]);

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
struct Cell {
    state: CellState,
    position: IVec2,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum CellState {
    Dead,
    Alive,
}

impl Cell {
    fn new(state: CellState, position: IVec2) -> Self {
        Self { state, position }
    }

    fn set_state(&mut self, state: CellState) {
        self.state = state;
    }

    fn draw(&self) {
        let color = match self.state {
            CellState::Alive => YELLOW,
            CellState::Dead => BLACK,
        };
        draw_rectangle(
            (self.position.x * MULTIPLIER) as f32,
            (self.position.y * MULTIPLIER) as f32,
            SQUARE_WIDTH,
            SQUARE_WIDTH,
            color,
        );
    }

    fn update(&self) {
        self.draw();
    }
}

fn window_conf() -> Conf {
    Conf {
        window_title: "Breakout".to_owned(),
        fullscreen: true,
        // window_height: 800,
        // window_width: 1000,
        ..Default::default()
    }
}

fn init_grid(grid: &mut GridType) {
    // TODO must go through grid and set the positions as correcgt
    for i in 0..GRID_SIZE {
        for j in 0..GRID_SIZE {
            grid.0[i][j].position = IVec2 {
                x: i as i32,
                y: j as i32,
            };
            grid.0[i][j].state = CellState::Dead;
        }
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    let mut grid =
        GridType([[(Cell::new(CellState::Dead, IVec2 { x: 0, y: 0 })); GRID_SIZE]; GRID_SIZE]);

    init_grid(&mut grid);
    let mut alive_cells: HashSet<Cell> = HashSet::new();

    loop {
        clear_background(WHITE);
        for i in 0..GRID_SIZE {
            for j in 0..GRID_SIZE {
                let cell = grid.0[i][j];
                cell.update();
            }
        }

        if is_key_pressed(KeyCode::Space) {
            do_iteration(&mut grid, &mut alive_cells);
        }

        if is_key_pressed(KeyCode::R) {
            init_grid(&mut grid);
            alive_cells.clear();
        }

        if is_mouse_button_pressed(MouseButton::Left) {
            let (mouse_x, mouse_y) = mouse_position();

            let (x_div, y_div) = (mouse_x / (MULTIPLIER as f32), mouse_y / (MULTIPLIER as f32));
            let (x_round, y_round) = (x_div.floor() as usize, y_div.floor() as usize);

            let mut cell = grid.0[x_round][y_round];

            match cell.state {
                CellState::Alive => {
                    cell.set_state(CellState::Dead);
                    alive_cells.remove(&cell);
                }
                CellState::Dead => {
                    cell.set_state(CellState::Alive);
                    alive_cells.insert(cell);
                }
            };
            grid.0[x_round][y_round] = cell;
        }
        next_frame().await
    }
}

fn check_out_bounds(res: IVec2) -> bool {
    if res.x < 0 || res.x > GRID_SIZE as i32 {
        return true;
    }
    if res.y < 0 || res.y > GRID_SIZE as i32 {
        return true;
    }
    return false;
}

fn do_iteration(grid: &mut GridType, alive_cells: &mut HashSet<Cell>) {
    let all_adjacents: Vec<IVec2> = vec![
        IVec2 { x: 0, y: -1 },                         //up
        IVec2 { x: 0, y: 1 },                          //down
        IVec2 { x: -1, y: 0 },                         //left
        IVec2 { x: 1, y: 0 },                          //right
        IVec2 { x: -1, y: 0 } + IVec2 { x: 0, y: -1 }, // left + up
        IVec2 { x: 1, y: 0 } + IVec2 { x: 0, y: -1 },  // right + up
        IVec2 { x: 1, y: 0 } + IVec2 { x: 0, y: 1 },   // right + down
        IVec2 { x: -1, y: 0 } + IVec2 { x: 0, y: 1 },  // left + down
    ];

    let mut to_check: HashSet<IVec2> = HashSet::new();

    // add all adjacents
    for alive in alive_cells.iter() {
        to_check.insert(alive.position);

        for adjacent in &all_adjacents {
            let res = (*adjacent) + alive.position;
            if check_out_bounds(res) {
                continue;
            }
            to_check.insert(res);
        }
    }
    let mut set_to_change: Vec<Cell> = vec![];

    for position_to_check in to_check {
        let cell: Cell = grid.0[position_to_check.x as usize][position_to_check.y as usize];

        let mut num_alive = 0;
        // Getting number of adjacent alive
        for translation in &all_adjacents {
            let adj_cell = position_to_check + *translation;
            if adj_cell.x < 0i32
                || adj_cell.x > GRID_SIZE as i32
                || adj_cell.y < 0
                || adj_cell.y > GRID_SIZE as i32
            {
                continue;
            }

            match grid.0[adj_cell.x as usize][adj_cell.y as usize].state {
                CellState::Alive => num_alive += 1,
                CellState::Dead => (),
            }
        }
        match cell.state {
            CellState::Alive => {
                match num_alive {
                    2 | 3 => {
                        // stays alive
                    }
                    _ => {
                        //death
                        let to_change_item: Cell = Cell {
                            position: position_to_check,
                            state: CellState::Dead,
                        };
                        set_to_change.push(to_change_item);
                        alive_cells.remove(&to_change_item);
                    }
                }
            }
            CellState::Dead => {
                match num_alive {
                    3 => {
                        //born
                        let to_change_item: Cell = Cell {
                            position: position_to_check,
                            state: CellState::Alive,
                        };
                        set_to_change.push(to_change_item);
                        alive_cells.insert(to_change_item);
                    }
                    _ => {
                        // stays dead
                    }
                }
            }
        }
    }
    for to_change_item in set_to_change.iter() {
        grid.0[to_change_item.position.x as usize][to_change_item.position.y as usize]
            .set_state(to_change_item.state);
    }
}
