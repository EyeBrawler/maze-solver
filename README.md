# Maze Solver

A Rust-based maze solver that demonstrates **depth-first search (DFS)** and **breadth-first search (BFS)** using custom stack and queue implementations.  
The program loads mazes from `.txt` files, visualizes the solving process in the terminal with colored output, and shows whether each maze can be solved.

## Features

- Parse and validate maze definitions from text files.
- Solve mazes using:
    - **Stack (DFS)** via `MyStack`
    - **Queue (BFS)** via `MyQueue`
- Step-by-step **animated visualization** in the terminal:
    - `@` → current position
    - `+` → visited squares
    - `#` → walls
    - `.` → open space
    - `o` → start
    - `*` → finish
- Modular design with traits (`Agenda`), generics, and custom data structures.

## Project Structure
```bash
src/
├── main.rs # Entry point: loads mazes and runs solvers
├── location.rs # Maze coordinates and neighbor calculation
├── maze.rs # Maze parsing, solving, and visualization
├── maze_data_structures.rs # Agenda trait, stack, and queue implementations
└── square.rs # Maze cell definitions (wall, open, start, finish)
mazes/ # Place your maze .txt files here
```

## Requirements

- [Rust](https://www.rust-lang.org/tools/install) (latest stable)
- A terminal that supports ANSI escape codes for colored output (e.g., most Linux/macOS terminals, or Windows Terminal)

## Setup & Usage

### 1. Clone this repository:
   ```bash
   git clone https://github.com/your-username/maze-solver.git
   cd maze-solver
   ```

### 2. Add your maze files in the `mazes/` folder. 
**Each maze file must:**
* Start with two integers: **width height**
* Followed by exactly `height` rows of `width` characters
* Valid characters:
  * `#` → wall
  * `.` → open
  * `o`→ start
  * `*`→ finish

Example (`mazes/example.txt`):
```
7 5
#######
#o...*#
#.#.#.#
#.....#
#######
```
### 3. Run the solver:
```bash
cargo run
```

### 4. The program will:
* Load each .txt maze in mazes/
* Solve it once with a stack (DFS)
* Solve it again with a queue (BFS)

### Example Output
```
Loading maze from file: "mazes/example.txt"
Maze loaded successfully!
Solved with a stack: true
Solved with a queue: true
```
## Technologies Used
* Rust → safe systems programming
* Traits & Generics → reusable solver interface (Agenda)
* Custom Data Structures → MyStack (DFS) and MyQueue (BFS)
* HashSet → track visited locations
* ANSI terminal colors via colored

## License
This project is for educational purposes.
Feel free to fork and modify.