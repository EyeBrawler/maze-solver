//Struct to store information about a maze location.
//Each Location has an x and y coordinate

//Location traits (and their reasons for existing) are very similar to traits for Squares.
//Hash is derived so the .contains method can be used on locations.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Location {
    pub x: usize,
    pub y: usize,
}

impl Location {
    //Function to generate a vector of locations of all neighbors a given location has.
    pub fn neighbors(&self, width: usize, height: usize) -> Vec<Location> {
        let mut neighbors = Vec::new();

        if self.x > 0 {
            neighbors.push(Location {
                x: self.x - 1,
                y: self.y,
            });
        }
        if self.x < width - 1 {
            neighbors.push(Location {
                x: self.x + 1,
                y: self.y,
            });
        }
        if self.y > 0 {
            neighbors.push(Location {
                x: self.x,
                y: self.y - 1,
            });
        }
        if self.y < height - 1 {
            neighbors.push(Location {
                x: self.x,
                y: self.y + 1,
            });
        }
        neighbors
    }
}
