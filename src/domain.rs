
#[derive(Debug)]
pub enum Player {
    X,
    O,
}

#[derive(Clone, Copy)]
pub enum Cell {
    XMark,
    OMark,
    Coordinate(i32),
    Empty,
}

impl std::fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let message = match self {
            Cell::XMark => "X".to_string(),
            Cell::OMark => "O".to_string(),
            Cell::Empty => " ".to_string(),
            Cell::Coordinate(value) => value.to_string(),
        };

        write!(f, "{}", message)
    }
}

pub type Board = [[Cell; 3]; 3];

