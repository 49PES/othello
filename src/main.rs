use anyhow::anyhow;
use anyhow::Result;
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
    fn dir_to_offset(dir: &Dir) -> (i32, i32) {
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

    fn dirs() -> Vec<Dir> {
        vec![
            Dir::Up,
            Dir::Down,
            Dir::Left,
            Dir::Right,
            Dir::UpLeft,
            Dir::UpRight,
            Dir::DownLeft,
            Dir::DownRight,
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Square {
    Unoccupied,
    Occupied(Color),
}

#[derive(Debug, Copy, Clone)]
struct Posn {
    row: usize,
    col: usize,
}

// (row, col) are 0-indexed positions on the board
impl Posn {
    // "a1" -> Posn { row: 0, col: 0 }
    // "e3" -> Posn { row: 2, col: 4 }
    fn alphanumeric_to_posn(s: String) -> Posn {
        let s: Vec<char> = s.chars().collect();
        let row = s[1].to_digit(10).unwrap() as usize - 1;
        let col = s[0].to_ascii_lowercase() as usize - 'a' as usize;
        Posn { row, col }
    }

    fn try_from_tuple(coords: (i32, i32)) -> Result<Self> {
        if coords.0 < 0 || coords.0 >= 8 || coords.1 < 0 || coords.1 >= 8 {
            return Err(anyhow!("Position out of bounds: {:?}", coords));
        }
        Ok(Posn {
            row: coords.0 as usize,
            col: coords.1 as usize,
        })
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
                    Square::Occupied(Color::Black) => '○',
                    Square::Occupied(Color::White) => '●',
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
    fn piece_at(&self, posn: &Posn) -> Square {
        self.squares[posn.row][posn.col]
    }

    fn set_piece_at(&mut self, posn: &Posn, square: Square) {
        self.squares[posn.row][posn.col] = square;
    }

    fn count_pieces(&self, color: Color) -> usize {
        let mut count = 0;
        for row in 0..8 {
            for col in 0..8 {
                let position = Posn { row, col };
                if self.piece_at(&position) == Square::Occupied(color) {
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

        let flipped_pieces = self.potential_flipped_pieces(&posn);

        for posn in flipped_pieces {
            board.set_piece_at(&posn, Square::Occupied(board.turn));
        }

        board.turn = next_color(board.turn);
        board
    }

    fn is_legal(&self, posn: &Posn) -> bool {
        self.piece_at(posn) == Square::Unoccupied && self.potential_flipped_pieces(posn).len() > 0
    }

    fn legal_moves(&self) -> Vec<Posn> {
        let mut moves: Vec<Posn> = vec![];
        for row in 0..8 {
            for col in 0..8 {
                let posn = Posn { row, col };
                if self.is_legal(&posn) {
                    moves.push(posn);
                }
            }
        }
        moves
    }
    fn potential_flipped_pieces_in_dir(&self, posn: &Posn, dir: Dir) -> Vec<Posn> {
        let mut line: Vec<Posn> = vec![];
        let (offset_row, offset_col) = Dir::dir_to_offset(&dir);
        let neighbor_coordinates = (posn.row as i32 + offset_row, posn.col as i32 + offset_col);
        match Posn::try_from_tuple(neighbor_coordinates) {
            Ok(mut neighbor) => match self.piece_at(&neighbor) {
                Square::Unoccupied => vec![],
                Square::Occupied(color) => {
                    if color == self.turn {
                        vec![]
                    } else {
                        let mut next_neighbor_coordinates: (i32, i32) = (
                            neighbor_coordinates.0 + offset_row,
                            neighbor_coordinates.1 + offset_col,
                        );
                        while let Ok(next_neighbor) =
                            Posn::try_from_tuple(next_neighbor_coordinates)
                        {
                            line.push(neighbor);
                            match self.piece_at(&next_neighbor) {
                                Square::Unoccupied => break,
                                Square::Occupied(color) => {
                                    if color == self.turn {
                                        return line;
                                    }
                                    neighbor = next_neighbor;
                                    next_neighbor_coordinates = (
                                        next_neighbor_coordinates.0 + offset_row,
                                        next_neighbor_coordinates.1 + offset_col,
                                    );
                                }
                            }
                        }
                        vec![]
                    }
                }
            },
            Err(_) => vec![],
        }
    }

    // Returns the positions of pieces that can be flipped
    fn potential_flipped_pieces(&self, posn: &Posn) -> Vec<Posn> {
        let mut potential_flipped_pieces: Vec<Posn> = vec![];
        for dir in Dir::dirs() {
            potential_flipped_pieces.append(&mut self.potential_flipped_pieces_in_dir(posn, dir));
        }
        potential_flipped_pieces
    }
}
fn main() {
    let mut board = initialize_board();
    println!("{}", board);
    /*
        let alpha_moves = ["a1", "b1", "h2"];
        let moves: Vec<Posn> = alpha_moves
            .iter()
            .map(|m| Posn::alphanumeric_to_posn(m.to_string()))
            .collect();
    */

    while board.legal_moves().len() > 0 {
        // let mut input = String::new();
        // std::io::stdin().read_line(&mut input).unwrap();
        // let posn = Posn::alphanumeric_to_posn(input.trim().to_string());
        // board = board.play_move(posn);
        // println!("{}", board);
        let moves = board.legal_moves();
        let posn = moves[0];
        board = board.play_move(posn);
        println!("{}", board);
    }
}
