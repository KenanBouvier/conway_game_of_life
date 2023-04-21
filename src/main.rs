use macroquad::prelude::*;
use std::collections::HashSet;

const GRID_SIZE: usize = 50;

#[derive(Copy, Clone, Debug)]
struct GridType([[Cell; GRID_SIZE]; GRID_SIZE]);

#[derive(Copy, Clone, Debug)]
struct Cell {
    state: CellState,
}

#[derive(Copy, Clone, Debug)]
enum CellState {
    Dead,
    Alive,
}

impl Cell {
    fn new(state: CellState) -> Self {
        Self { state }
    }

    fn change_state(&mut self, state: CellState) {
        self.state = state;
    }
}
fn main() {
    let mut grid = GridType([[Cell::new(CellState::Dead); GRID_SIZE]; GRID_SIZE]);

    grid.0[10][10].state = CellState::Alive;
    grid.0[11][10].state = CellState::Alive;
    grid.0[12][10].state = CellState::Alive;
    grid.0[12][9].state = CellState::Alive;
    grid.0[11][8].state = CellState::Alive;

    let mut alive_cells: HashSet<IVec2> = HashSet::new();

    alive_cells.insert(IVec2 { x: 10, y: 10 });
    alive_cells.insert(IVec2 { x: 11, y: 10 });
    alive_cells.insert(IVec2 { x: 12, y: 10 });
    alive_cells.insert(IVec2 { x: 12, y: 9 });
    alive_cells.insert(IVec2 { x: 11, y: 8 });

    // println!("{:#?}", alive_cells);
    do_iteration(&mut grid, &mut alive_cells);
}

fn do_iteration(grid: &mut GridType, alive_cells: &mut HashSet<IVec2>) {
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

    for alive in alive_cells.iter() {
        to_check.insert(*alive);

        for adjacent in &all_adjacents {
            let res = (*adjacent) + *alive;
            to_check.insert(res);
        }
    }
    dbg!(&to_check.len());
    // diagnostics(&to_check);

    let mut set_to_change: Vec<(IVec2, CellState)> = vec![];

    for position_to_check in to_check {
        let cell: Cell = grid.0[position_to_check.x as usize][position_to_check.y as usize];

        let mut num_alive = 0;
        // Getting number of adjacent alive
        for translation in &all_adjacents {
            let adj_cell = position_to_check + *translation;
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
                        let to_change_item: (IVec2, CellState) =
                            (position_to_check, CellState::Dead);
                        set_to_change.push(to_change_item);
                        //TODO erase from alive_cells
                        alive_cells.remove(&position_to_check);
                    }
                }
            }
            CellState::Dead => {
                match num_alive {
                    3 => {
                        //born
                        let to_change_item: (IVec2, CellState) =
                            (position_to_check, CellState::Alive);
                        set_to_change.push(to_change_item);

                        //TODO append to alive_cells
                        alive_cells.insert(position_to_check);
                    }
                    _ => {
                        // stays dead
                    }
                }
            }
        }
    }
    for to_change_item in set_to_change.iter() {
        grid.0[to_change_item.0.x as usize][to_change_item.0.y as usize]
            .change_state(to_change_item.1);
    }
    println!("{:#?}", alive_cells);
}
