use std::fmt::Display;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Color {
    Black,
    White,
}
fn next_color(color: Color) -> Color {
    match color {
        Color::Black => Color::White,
        Color::White => Color::Black,
    }
}

enum Dir {
    Up,
    Down,
    Left,
    Right,
    UpLeft,
    UpRight,
    DownLeft,
    DownRight,
}

impl Dir {
    fn dir_to_tuple(dir: Dir) -> (i32, i32) {
        match dir {
            Dir::Up => (0, 1),
            Dir::Down => (0, -1),
            Dir::Left => (-1, 0),
            Dir::Right => (1, 0),
            Dir::UpLeft => (-1, 1),
            Dir::UpRight => (1, 1),
            Dir::DownLeft => (-1, -1),
            Dir::DownRight => (1, -1),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Square {
    Unoccupied,
    Occupied(Color),
}

struct Posn {
    row: usize,
    col: usize,
}

// (row, col) are 0-indexed positions on the board
impl Posn {
    fn new(row: usize, col: usize) -> Posn {
        Posn { row, col }
    }

    // "a1" -> Posn { row: 0, col: 0 }
    // "e3" -> Posn { row: 2, col: 4 }
    fn alphanumeric_to_posn(s: String) -> Posn {
        let s: Vec<char> = s.chars().collect();
        let row = s[1].to_digit(10).unwrap() as usize - 1;
        let col = s[0].to_ascii_lowercase() as usize - 'a' as usize;
        Posn { row, col }
    }
}

#[derive(Debug, Clone, Copy)]
struct Board {
    squares: [[Square; 8]; 8],
    turn: Color,
}

// Othello board — put initial pieces on the board at the center
fn initialize_board() -> Board {
    let mut board = [[Square::Unoccupied; 8]; 8];
    board[3][3] = Square::Occupied(Color::Black);
    board[3][4] = Square::Occupied(Color::White);
    board[4][3] = Square::Occupied(Color::White);
    board[4][4] = Square::Occupied(Color::Black);

    Board {
        squares: board,
        turn: Color::Black,
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut grid: Vec<Vec<char>> = vec![vec!['_'; 8]; 8];
        for row in 0..8 {
            for col in 0..8 {
                grid[row][col] = match self.squares[row][col] {
                    Square::Unoccupied => '_',
                    Square::Occupied(Color::Black) => 'X',
                    Square::Occupied(Color::White) => 'O',
                }
            }
        }

        for row in &grid {
            for ch in row {
                write!(f, "{}", ch)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl Board {
    fn piece_at(&self, posn: Posn) -> Square {
        self.squares[posn.row][posn.col]
    }

    fn count_pieces(&self, color: Color) -> usize {
        let mut count = 0;
        for row in 0..8 {
            for col in 0..8 {
                let position = Posn::new(row, col);
                if self.piece_at(position) == Square::Occupied(color) {
                    count += 1;
                }
            }
        }
        count
    }

    // board -> (black, white)
    fn score(&self) -> (usize, usize) {
        (
            self.count_pieces(Color::Black),
            self.count_pieces(Color::White),
        )
    }

    fn play_move(self, posn: Posn) -> Board {
        let mut board = self;
        board.squares[posn.row][posn.col] = Square::Occupied(board.turn);

        // TODO: flip pieces

        board.turn = next_color(board.turn);
        board
    }
}

fn main() {
    let mut board = initialize_board();
    println!("{}", board);

    let alpha_moves = ["a1", "b1", "h2"];
    let moves: Vec<Posn> = alpha_moves
        .iter()
        .map(|m| Posn::alphanumeric_to_posn(m.to_string()))
        .collect();

    for m in moves {
        board = board.play_move(m);
        println!("{}", &board);
    }
}
