mod location;
mod maze;
mod maze_data_structures;
mod square;

use maze::Maze;
use maze_data_structures::MyQueue;
use maze_data_structures::MyStack;

use std::fs;
use std::thread::sleep;
use std::time::Duration;

//This main function creates looks at all directories in the mazes folder and runs both maze
//algorithms on each maze.
fn main() {
    //Defining the mazes directory
    let mazes_dir = "mazes";

    //Reading all files in the directory and storing them in a variable called entries.
    let entries = fs::read_dir(mazes_dir).expect("Failed to Read the Directory");

    for entry in entries {
        //Unwrap the entry and get its path
        let entry = entry.expect("Failed to read an entry");
        let path = entry.path();

        //Only processing the files and of all the files in the directory, accept only text files
        //with the .txt extension
        if path.is_file() && path.extension().and_then(|ext| ext.to_str()) == Some("txt") {
            println!("Loading maze from file: {:?}", path);

            //Reading the files contents
            let input = fs::read_to_string(&path).expect("Failed to read the maze file");

            //Attempt to create a maze from the file's contents
            //Solving the maze with each algorithm
            match Maze::new(&input) {
                Ok(maze) => {
                    println!("Maze loaded successfully!");
                    sleep(Duration::from_millis(2000));

                    let solved_my_stack = maze.solve(MyStack::new());
                    println!("Solved with a stack: {}", solved_my_stack);

                    sleep(Duration::from_millis(1000));

                    let solved_my_queue = maze.solve(MyQueue::new());
                    println!("Solved with a queue: {}", solved_my_queue);
                }
                Err(e) => {
                    eprintln!("Failed to load maze from {:?}: {}", path, e);
                }
            }
        }
    }
}
