//Defining the enum itself. It has the PartialEq and Eq traits so that squares can be compared. It
//has the copy and clone traits exist so that that Square's are implicitly copied when being passed
//around in code.
#[derive(PartialEq, Eq)]
pub enum Square {
    Wall,   // '#'
    Open,   // '.'
    Start,  // 'o'
    Finish, // '*'
}

impl Square {
    // Converts a `Square` enum variant to its corresponding character.
    pub fn to_char(&self) -> char {
        match self {
            Square::Wall => '#',
            Square::Open => '.',
            Square::Start => 'o',
            Square::Finish => '*',
        }
    }

    // Converts a character to its corresponding `Square` enum variant.
    // Returns an `Option<Square>`, or `None` for invalid input.
    pub fn from_char(ch: char) -> Result<Square, String> {
        match ch {
            '#' => Ok(Square::Wall),
            '.' => Ok(Square::Open),
            'o' => Ok(Square::Start),
            '*' => Ok(Square::Finish),
            _ => Err(format!("Invalid character: {}", ch)),
        }
    }
}
