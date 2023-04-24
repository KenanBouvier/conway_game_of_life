# Conway's game of life

This is an implementation of Conway's game of life.

<img width="500" alt="Screenshot 2023-04-24 at 00 34 18" src="https://user-images.githubusercontent.com/65245574/233872283-f3f8cd49-f525-4acb-8835-48d346ae6f73.png">

I came across the model of Cellular automaton from which I saw Conway's game of life. 

Some links:
https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life
https://en.wikipedia.org/wiki/Cellular_automaton



## Installation

```bash
git clone https://github.com/KenanBouvier/conway_game_of_life.git
cd conway_game_of_life
cargo run
```

## Usage
- [Click] on cells by clicking to turn them on/off.
- [Space] to simulate a generation.
- [R] to reset map.

Main params to edit:
- GRID_SIZE -> nxn
- SQUARE_WIDTH -> size of cell
- PADDING -> How the cells are spread (set to 0 for solid background).


## Logic
NB: Just personal implementation so reference proper source code if needed.

- We must have a grid of cells, with a state of dead/alive. 

- We need to make the checks with adjacent alive. There are multiple ways to do this. As we want to be as performant, we can only do that 8 way adjacent check (vertically, horizonatally and diagonally) on squares that have the possibility of changing. We can keep a queue(or similar DS) that we will iterate through every frame and determine the update. This queue can be of all the alive cells. On a more complex system, one can utilize the concept of Dynamic Programming to memoize the state at an index to stop further calculation of that "subproblem" on subsequent checks. However, since we are already working on low-level primitive types with constant time complexity to solve this "subproblem" then there is no use.

- We have a vector that stores all the Cells that we want to check. Then we want to push the 8 adjacents to this  vector

- This is like a BFS algorithm in which we push states we want to search into, and similarly to BFS, we want to handle repeated Cells as else we will forever keep pushing to our check_state. We can just push to a HashSet that will remove duplicated values.

- A core feature that must be implemented is a temporary changed state. What this means is that we want to make the changes to a cell's state after all the checks for changes of state have been done. Else, for example, if one cell should become alive and we set it as such, now another cell won't be able to differentiate if it is alive from the previous frame or current change. Hence, we can keep another vector storing all the grid positions that will change and after we update the grid at the positions with their respective new state.

## UI

https://github.com/not-fl3/macroquad

## Possible extension/variation of program

- Instead of 1 neighbours killing the cell, the cell remains alive but instead 'stagnant'
- This means that the cell is not considered to "reproduce" for other adjacent cells.
- If a cell is already stagnant and continues stagnant then it dies


