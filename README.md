# Conway's game of life

This is an implementation of Conway's famous game of life.

You can interact with the cells, to turn them on/off.
Press <Space> to simulate a generation.
Press <R> to reset map;


## The Logic

- We must have a grid of cells, with a state of dead/alive. 

- Also, we need to make the checks with adjacent alive. There are multiple ways to do this. As we want to be as performant, we can only do that 8 way adjacent check (vertically, horizonatally and diagonally) on squares that have the possibility of changing. We can keep a queue(or similar DS) that we will iterate through every frame and determine the update. This queue can be of all the alive cells. On a more complex system, one can utilize the concept of Dynamic Programming to memoize the state at an index to stop further calculation of that "subproblem" on subsequent checks. However, since we are already working on low-level primitive types with constant time complexity to solve this "subproblem" then there is no use.

- We have a check_state vector that stores all the Cells that we want to check. Initially, we want all the cells that have been turned on to be within this. Then we also want to push the 8 adjacents to this check_state vector as if we only had Cells that were alive in this check_state vector, we would only be considering deaths. i.e. from alive the only option is Alive -> Dead.

- This is like a BFS algorithm in which we push states we want to search into, and similarly to BFS, we want to handle repeated Cells as else we will forever keep pushing to our check_state. So we can keep a hashSet of all the included Cells using its position as the unique identifier / type for this hashSet. Then on the search outwards we make that check that it has not been visited before.


- A core feature that must be implemented is a temporary changed state. What I mean by this is that we want to make the changes to a cell's state after all the checks for changes of state have been done. Else, for example, if one cell should become alive and we set it as such, now another cell won't be able to differentiate if it is alive from the previous frame or current change. Hence, we can keep a vector array storing all the grid positions that will change and update the state after going through our previous checking of state.

# Building the UI

Used Macroquad to render the shapes as it's pretty minimal.

### Handling intersections && Grid UI

As we have a uniform grid of squares, we can use math to get the exact button in constant time. First, to allow for dynamic parameters in our program, such as number of buttons, the widths of these buttons, I used the indexes of the squares as the position in world. Since the values are way to small for the squares to not overlap, I created a multiplier constant that will scale our position to then spread it out. Now that we have this as the type structure, we can now use this multiplier and have it divide our mouse position for x and y respectively. This will give us a f32 value representing our point back into index format. We are just reversing the function:

f(x){x*multiplier} where x is the index position and f(x) is world position. The reverse would simply the division f(y){y/multiplier} where the input and output for index and world position is reversed.

Using this transformation makes the changing of our previous variables completely flexible allowing us to scale our 2d grid to much bigger sizes.


# Possible extension/variation of program

- Instead of 1 neighbours killing the cell, the cell remains alive but instead 'stagnant'
- This means that the cell is not considered to "reproduce" for other adjacent cells.
- If a cell is already stagnant and continues stagnant then it dies

