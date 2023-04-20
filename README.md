# Conway's game of life


# Implementation

- Possible extension
- Instead of 1 neighbours killing the cell, the cell remains alive but instead 'stagnant'
- This means that the cell is not considered to "reproduce" for other adjacent cells.
- If a cell is already stagnant and continues stagnant then it dies

## Possibilities

- Dead -> Alive (3 adjacent)

- Alive -> Dead (0 adjacent or >=4 adjacent)
- Alive -> Stagnant (1 adjacent)
- Alive -> Alive (2 or 3 adjacent)

- Stagnant -> Dead (<=1 adjacent or >=4 adjacent)
- Stagnant -> Alive (3 adjacent)
- Stagnant -> Stagnant (2 adjacent)

## Analysis on the algorithm

- We must have a grid of cells, with a state of dead/alive. 

- Also, we need to make the checks with adjacent alive. There are multiple ways to do this. As we want to be as performant, we can only do that 8 way adjacent check (vertically, horizonatally and diagonally) on squares that have the possibility of changing. We can keep a queue(or similar DS) that we will iterate through every frame and determine the update. This queue can be of all the alive cells. On a more complex system, one can utilize the concept of Dynamic Programming to memoize the state at an index to stop further calculation of that "subproblem" on subsequent checks. However, since we are already working on low-level primitive types with constant time complexity to solve this "subproblem" then there is no use. Though I might implement it.

- We have this check_state vector that stores all the Cells that we want to check. Initially, we want all the cells that have been turned on to be within this. Then we also want to push the 8 adjacents to this check_state vector as if we only had Cells that were alive in this check_state vector, we would only be considering deaths. i.e. from alive the only option is Alive -> Dead.

- This is like a BFS algorithm in which we push states we want to search into, and similarly to BFS, we want to handle repeated Cells as else we will forever keep pushing to our check_state. So we can keep a hashSet of all the included Cells using its position as the unique identifier / type for this hashSet. Then on the search outwards we make that check that it has not been visited before.


- A core feature that must be implemented is a temporary changed state. What I mean by this is that we want to make the changes to a cell's state after all the checks for changes of state have been done. Else, for example, if one cell should become alive and we set it as such, now another cell won't be able to differentiate if it is alive from the previous frame or current change. Hence, we can keep a vector array storing all the grid positions that will change and update the state after going through our previous checking of state.


Putting everything together at a very high level purely to understand flow, it would look something like this: (PS. This definitely doesn't compile! Just pseudo code for brainstorming purpose)


# The core logic

### Type declarations:

```rust

enum CellState {
    Alive,
    Dead,
}

// 2d array all elements type Cell
struct GridType([[Cell; GRID_SIZE]; GRID_SIZE]);

type Cell {
    state: CellState,
}

```

### Core logic:

```rust

fn main(){
    // fill up  / track with all alive
    let alive_cells: Vec<Vec2> = vec![];

    // we want the code below to loop per frame.
    // we want alive_cells to persist data across frames but everything else must clear out.
    // though I think optimally, we would want this check_state dynamically change during runtime instead of computing it every time.

    let check_state: Vec<Vec2> = vec![];
    


    for alive in alive_cells {
        check_state.push(alive);

        let up = ...
        // Same for all directions

        check_state.push(up);
        // Push all directions
    }


    // this toUpdate keeps track of the cells that have changed and we need to loop after doing all the check_states
    let mut toUpdate: Vec<Vec2> = vec![];

    // now we iterate through the cells we want to see if there is a change
    for toCheck in check_state {

        // this stops repeated BFS search loops
        if toCheck in visited {
            continue;
        }
        visited.push(toCheck);

        //if there is a change then we want to keep track of this in our toUpdate
        if state_should_change(&grid, &toCheck) {
            toUpdate.push(toUpdate);
        }
    }

    for position in toUpdate{
        update_grid(&grid, &position); 
    }

}

// returns whether, through the current cell state, should the state change
fn state_should_change(grid: &GridType, position: &Vec2) -> bool {
    let mut num_alive = 0;

    let cell = grid[position.x][position.y];

    // Here we do our checks vertically, horizonatally and diagonally
    // ...

    // Now depending on the cell's alive state we do different checks 
    match cell {
        Cell::Alive =>{
            // do alive condition checks 
        }
        Cell::Dead =>{
            // do dead condition checks 
        }
    }

}

fn update_grid(grid: &mut GridType, position: &Vec2){
    let cell = grid[position.x][position.y];

    // as there are only two states, we can just alternate if it is alive and vice versa
    match cell.state {
        Cell::Alive => cell.state = Cell::Dead,
        Cell::Dead => cell.state => Cell::Alive,
    }
}
```

# building a visual representation of this server code



