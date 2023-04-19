const GRID_SIZE: usize = 4;
enum CellState {
    Dead,
    Alive,
}

struct GridType([[Cell; GRID_SIZE]; GRID_SIZE]);

struct Cell {
    state: CellState,
    adjacent_alive: usize,
}

impl Cell {
    fn new(state: CellState) -> Self {
        Self {
            state: state,
            adjacent_alive: 0,
        }
    }

    fn change_state(&mut self) {
        self.state = match self.state {
            CellState::Dead => CellState::Alive,
            CellState::Alive => CellState::Dead,
        };
    }
}

struct Vec2 {
    x: i32,
    y: i32,
}

fn get_adjacent_alive(alive_cells: &GridType, position: &Vec2) -> usize {
    let adj_count = 0;

    adj_count
}

fn main() {
    let cell = Cell::new(CellState::Dead);
    let cell2 = Cell::new(CellState::Alive);
    let mut alive_cells = GridType([
        [cell, cell, cell2, cell],
        [cell2, cell, cell2, cell],
        [cell2, cell, cell2, cell],
        [cell, cell, cell, cell],
    ]);
    // [[0, 0, 1, 0], [1, 0, 1, 0], [1, 0, 1, 0], [0, 0, 0, 0]]
    println!("Hello, world!");
}
