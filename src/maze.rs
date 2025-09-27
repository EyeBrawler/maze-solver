use crate::location::Location;
use crate::maze_data_structures::Agenda;
use crate::square::Square;
use colored::Colorize;
use std::collections::HashSet;
use std::fmt;
use std::io::Write;
use std::thread::sleep;
use std::time::Duration;

pub struct Maze {
    width: usize,
    height: usize,

    //Layout is essentially a 2D array (A vector filled with square vectors)
    layout: Vec<Vec<Square>>,
}

//Methods that can be called on a maze
impl Maze {
    //Maze Constructor that reads maze dimensions and layout from a string input
    pub fn new(input: &str) -> Result<Maze, String> {
        //.lines looks for newline characters and splits slices according to that.
        let mut lines = input.lines();

        //Read the first line to acquire the dimensions
        //First dimension is width and the second is height
        let dimensions = lines
            .next() //Returns the first line.
            .ok_or("Missing dimensions")? //Missing dimensions if no line.
            .split_whitespace() //Splits a string slice by whitespace.
            .collect::<Vec<&str>>(); //Adding that string to a vector of string slices.

        //More than two elements in the list means the dimensions are not in a valid format.
        if dimensions.len() != 2 {
            return Err("Invalid dimensions format".to_string());
        }

        //Parsing the dimensions from the elements
        let width = dimensions[0].parse::<usize>().map_err(|_| "Invalid width");
        let height = dimensions[1].parse::<usize>().map_err(|_| "Invalid height");

        //Read thee rest of the lines to construct the layout of the maze.
        let mut layout = Vec::new();

        for (i, line) in lines.enumerate() {
            let row: Result<Vec<Square>, _> = line.chars().map(Square::from_char).collect();

            //? Means we don't know exactly what type the vector will contain
            let row = row?;

            //If the row does not match the set width
            if row.len() != width? {
                //Send Appropriate Error Message
                return Err(format!(
                    "Row {} does not match expected width of {}",
                    i + 1,
                    width?
                ));
            }

            //Add the row to the layput vector
            layout.push(row);
        }

        //Check if there are too many or too little rows based on maze dimensions
        if layout.len() != height? {
            return Err(format!(
                "Number of rows dooes not match expected height of {}",
                height?
            ));
        }

        //In the case that everything went okay, return a maze struct with the collected dimensions and
        //layout.
        Ok(Maze {
            width: width?,
            height: height?,
            layout,
        })
    }

    //Method to find the starting location of the a maze.
    //Returns a Location struct wrapped in an option (in case there is no start)
    fn find_start(&self) -> Option<Location> {
        //Nested for loop to look through all locations
        for y in 0..self.height {
            for x in 0..self.width {
                if self.layout[y][x] == Square::Start {
                    return Some(Location { x, y });
                }
            }
        }
        None
    }

    //Method to find the finish location of a maze.
    //Returns a Location stuct wrapped in an option (just in case there is no finish).
    fn find_finish(&self) -> Option<Location> {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.layout[y][x] == Square::Finish {
                    return Some(Location { x, y });
                }
            }
        }
        None
    }

    //Retuns if a given spot in the maze is open (or walkable)
    pub fn is_open(&self, location: Location) -> bool {
        let x = location.x;
        let y = location.y;

        if x >= self.width || y >= self.height {
            return false; // Out of bounds
        }

        matches!(
            self.layout[y][x],
            Square::Open | Square::Start | Square::Finish
        )
    }
    //A function to solve a maze. It can accept any kind of stucture that implements the agenda trait as a
    //parameter for the function. This allows both MyStack and MyQueue can share the same function.
    //It returns a boolean for if the maze can be solved or not.

    pub fn solve<A: Agenda<Location>>(&self, mut agenda: A) -> bool {
        // Find the start and finish locations.
        let start = self.find_start().expect("No start location found.");
        let finish = self.find_finish().expect("No finish location found.");

        // A hash set to store visited locations.
        let mut visited = HashSet::new();

        // Add the start location to the agenda to begin exploration.
        agenda.add(start);

        // Loop through the Agenda while there are items in it.
        while !agenda.is_empty() {
            println!("Agenda is not empty"); // Debugging

            if let Some(current) = agenda.remove() {
                if visited.contains(&current) {
                    continue; // Skip already visited locations.
                }

                if current == finish {
                    self.print_maze_with_highlight(Some(current), &visited, true);
                    sleep(Duration::from_millis(1500)); // Pause at the end for clarity
                    return true; // The maze has been solved!
                }

                // Mark the current location as visited.
                visited.insert(current);

                //Show the current maze state with the current position highlighted
                //False as the last parameter indicates to the function the maze has not yet been
                //solved. Running sleep so that the frame can be seen.
                self.print_maze_with_highlight(Some(current), &visited, false);
                sleep(Duration::from_millis(500)); // Pause between each move

                // Explore all valid neighbors.
                for neighbor in current.neighbors(self.width, self.height) {
                    if self.is_open(neighbor) && !visited.contains(&neighbor) {
                        agenda.add(neighbor); // Add valid neighbors to the agenda.
                    }
                }
            }
        }
        //Priting the final state of the maze when no solution has been found
        false // No solution was found.
    }

    fn print_maze_with_highlight(
        &self,
        current: Option<Location>,
        visited: &HashSet<Location>,
        solved: bool,
    ) {
        //Escape Codes to clear the terminal
        print!("\x1B[2J\x1B[1;1H");

        for y in 0..self.height {
            for x in 0..self.width {
                let location = Location { x, y };

                if Some(location) == current {
                    let symbol = "@".bright_cyan();
                    print!("{}", symbol); //Current Position
                } else if visited.contains(&location) {
                    let symbol = "+".yellow().dimmed();
                    print!("{}", symbol); //visited location
                } else {
                    match self.layout[y][x] {
                        Square::Wall => {
                            let symbol = "#".red().on_bright_red();
                            print!("{}", symbol);
                        }
                        Square::Open => {
                            let symbol = ".".white().dimmed();
                            print!("{}", symbol);
                        }
                        Square::Start => {
                            let symbol = "o".green();
                            print!("{}", symbol);
                        }
                        Square::Finish => {
                            let symbol = "*".green();
                            print!("{}", symbol);
                        }
                    }
                }
            }

            //Printing a newline at the end of each row
            println!();
        }

        if solved {
            println!("Solution found!");
        }

        // Explicitly flush the output to force it to display immediately
        std::io::stdout().flush().expect("Failed to flush stdout");
    }
}

//Implementing Display for a maze so it can easily be printed out to the terminal when calling
//its to_string() method
impl fmt::Display for Maze {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        //Writing the width and height first
        writeln!(f, "{} {}", self.width, self.height)?;

        //Writing each row of the maze
        for row in &self.layout {
            let line: String = row.iter().map(Square::to_char).collect();
            writeln!(f, "{}", line)?;
        }

        //Returning okay because the return type for this function is a result.
        Ok(())
    }
}
