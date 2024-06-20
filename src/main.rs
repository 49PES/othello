use core::cmp::Ordering;
use rand::Rng;
use std::fmt::Display;
use tqdm::tqdm;

const ROWS: usize = 8;
const COLS: usize = 8;
const DIRS: [Dir; 8] = [
    Dir::Up,
    Dir::Down,
    Dir::Left,
    Dir::Right,
    Dir::UpLeft,
    Dir::UpRight,
    Dir::DownLeft,
    Dir::DownRight,
];
const POSNS: [Posn; ROWS * COLS] = generate_positions();

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
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

impl Display for Posn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}{}",
            ('a' as u8 + self.col as u8) as char,
            self.row + 1
        )
    }
}

// (row, col) are 0-indexed positions on the board
impl Posn {
    // "a1" -> Posn { row: 0, col: 0 }
    // "e3" -> Posn { row: 2, col: 4 }

    /*
    fn alphanumeric_to_posn(s: String) -> Posn {
        let s: Vec<char> = s.chars().collect();
        let row = s[1].to_digit(10).unwrap() as usize - 1;
        let col = s[0].to_ascii_lowercase() as usize - 'a' as usize;
        Posn { row, col }
    }
    */

    fn try_from_tuple(coords: (i32, i32)) -> Option<Self> {
        if (0..ROWS as i32).contains(&coords.0) && (0..COLS as i32).contains(&coords.1) {
            Some(Posn {
                row: coords.0 as usize,
                col: coords.1 as usize,
            })
        } else {
            None
        }
    }

    fn to_tuple(&self) -> (usize, usize) {
        (self.row, self.col)
    }

    /// Find the neighbor in the given direction, if it exists
    fn neighbor_in_dir(&self, dir: &Dir) -> Option<Self> {
        let (offset_row, offset_col) = Dir::dir_to_offset(dir);
        Posn::try_from_tuple((self.row as i32 + offset_row, self.col as i32 + offset_col))
    }

    fn is_row_edge(&self) -> bool {
        self.row == 0 || self.row == ROWS - 1
    }

    fn is_col_edge(&self) -> bool {
        self.col == 0 || self.col == COLS - 1
    }

    fn is_edge(&self) -> bool {
        self.is_row_edge() || self.is_col_edge()
    }

    fn is_corner(&self) -> bool {
        self.is_row_edge() && self.is_col_edge()
    }
}

const fn generate_positions() -> [Posn; ROWS * COLS] {
    let mut posns = [Posn { row: 0, col: 0 }; ROWS * COLS];
    let mut i = 0;
    while i < ROWS {
        let mut j = 0;
        while j < COLS {
            posns[i * COLS + j] = Posn { row: i, col: j };
            j += 1;
        }
        i += 1;
    }
    posns
}

#[derive(Debug, Clone)]
struct Board {
    squares: [[Square; COLS]; ROWS],
    turn: Color,
}

impl Board {
    fn new() -> Self {
        let mut board = [[Square::Unoccupied; COLS]; ROWS];
        board[ROWS / 2 - 1][COLS / 2 - 1] = Square::Occupied(Color::Black);
        board[ROWS / 2 - 1][COLS / 2] = Square::Occupied(Color::White);
        board[ROWS / 2][COLS / 2 - 1] = Square::Occupied(Color::White);
        board[ROWS / 2][COLS / 2] = Square::Occupied(Color::Black);

        Self {
            squares: board,
            turn: Color::Black,
        }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // +1 Padding to apply alphanumeric descriptions along the top row & columns
        let mut grid: Vec<Vec<char>> = vec![vec!['_'; COLS + 1]; ROWS + 1];

        // Letters along the top row to describe columns, numbers along the left column to describe rows
        grid[0][0] = ' ';
        for row in 0..ROWS {
            grid[row + 1][0] = (row + 1).to_string().chars().next().unwrap();
        }

        for col in 0..COLS {
            grid[0][col + 1] = ('a' as u8 + col as u8) as char;
        }

        for posn in POSNS {
            match self.piece_at(&posn) {
                Square::Unoccupied => grid[posn.row + 1][posn.col + 1] = '_',
                Square::Occupied(Color::Black) => grid[posn.row + 1][posn.col + 1] = '○',
                Square::Occupied(Color::White) => grid[posn.row + 1][posn.col + 1] = '●',
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

    fn count_color_pieces(&self, color: Color) -> usize {
        POSNS
            .into_iter()
            .filter(|posn| self.piece_at(posn) == Square::Occupied(color))
            .count()
    }

    fn is_not_full(&self) -> bool {
        POSNS
            .into_iter()
            .any(|posn| self.piece_at(&posn) == Square::Unoccupied)
    }

    /// Board → # of White pieces - # of Black pieces
    fn score(&self) -> i32 {
        self.count_color_pieces(Color::White) as i32 - self.count_color_pieces(Color::Black) as i32
    }

    fn play_move(self, posn: Posn) -> Board {
        let mut board = self.clone();

        let flipped_pieces = board.potential_flipped_pieces(&posn);
        for posn in flipped_pieces {
            board.set_piece_at(&posn, Square::Occupied(board.turn));
        }
        board.set_piece_at(&posn, Square::Occupied(board.turn));

        board.turn = next_color(board.turn);
        board
    }

    fn is_legal(&self, posn: &Posn) -> bool {
        self.piece_at(posn) == Square::Unoccupied && !self.potential_flipped_pieces(posn).is_empty()
    }

    fn legal_moves(&self) -> Vec<Posn> {
        POSNS
            .into_iter()
            .filter(|posn| self.is_legal(posn))
            .collect()
    }

    fn potential_flipped_pieces_in_dir(&self, posn: &Posn, dir: Dir) -> Vec<Posn> {
        let mut line: Vec<Posn> = vec![];
        let mut curr_neighbor = posn.neighbor_in_dir(&dir);

        // Keep going until we run off the board or find an unoccupied square (no pieces to flip),
        // or find a piece of the same color (we've found a flip)
        while let Some(curr) = curr_neighbor {
            match self.piece_at(&curr) {
                Square::Occupied(color) if color == self.turn => {
                    return line;
                }
                Square::Occupied(_other_color) => {
                    line.push(curr);
                }
                Square::Unoccupied => {
                    return vec![];
                }
            }
            curr_neighbor = curr.neighbor_in_dir(&dir);
        }

        // We've run off the board: if we haven't already returned, then there's no second tile to
        // surround any of the current line, and there's no flips in this direction
        vec![]
    }

    fn potential_flipped_pieces(&self, posn: &Posn) -> Vec<Posn> {
        DIRS.into_iter()
            .flat_map(|dir| self.potential_flipped_pieces_in_dir(posn, dir))
            .collect()
    }
}

fn standard_heuristic(board: &Board) -> i32 {
    board.score()
}

/// Heuristic that favors edge and corner positions (corners/edges/else = 4/2/1)
fn edge_corner_heuristic(board: &Board) -> i32 {
    fn color_weighted_score(board: &Board, color: Color) -> i32 {
        let mut total = 0;
        for posn in POSNS {
            if board.piece_at(&posn) == Square::Occupied(color) {
                total += if posn.is_corner() {
                    4
                } else if posn.is_edge() {
                    2
                } else {
                    1
                }
            }
        }
        total
    }

    color_weighted_score(board, Color::White) - color_weighted_score(board, Color::Black)
}

// Random agent that chooses a random legal move
fn random_agent(board: &Board) -> Posn {
    let legal_moves = board.legal_moves();
    legal_moves[rand::thread_rng().gen_range(0..legal_moves.len())]
}

// Greedy agent that chooses the move with the most potential flips
fn greedy_agent(board: &Board) -> Posn {
    let legal_moves = board.legal_moves();
    *legal_moves
        .iter()
        .max_by(|a, b| {
            board
                .potential_flipped_pieces(a)
                .len()
                .cmp(&board.potential_flipped_pieces(b).len())
        })
        .unwrap()
}

fn main() {
    let mut white_wins = 0;
    let mut black_wins = 0;
    let mut num_ties = 0;
    let num_iterations = 10000;
    for _ in tqdm(0..num_iterations) {
        let mut board = Board::new();

        while board.is_not_full() {
            // If player has no legal moves, play moves to opponent
            if board.legal_moves().is_empty() {
                board.turn = next_color(board.turn);
                // If opponent also has no legal moves, game is over
                if board.legal_moves().is_empty() {
                    break;
                }
                continue;
            }
            match board.turn {
                Color::White => {
                    let posn = greedy_agent(&board);
                    board = board.play_move(posn);
                }
                Color::Black => {
                    let posn = greedy_agent(&board);
                    board = board.play_move(posn);
                }
            }
        }

        match board.score().cmp(&0) {
            Ordering::Less => black_wins += 1,
            Ordering::Greater => white_wins += 1,
            Ordering::Equal => num_ties += 1,
        }
    }

    // Of statistical interest:
    // Given two opposing random model, the ratio of black wins to white wins to ties is ~45:50:5
    // Given greedy vs random, the ratio of black wins to white wins to ties is ~64:33:3
    // Since greedy is deterministic, greedy vs greedy, white wins 100% of the time (fixed game)
    println!(
        "Black wins: {}, White wins: {}, Ties: {}",
        black_wins, white_wins, num_ties,
    );

    /*
    println!("Enter a legal alphanumeric position (e.g. \"e4\") to play a move");
    println!("Enter \"moves\" to see all legal moves");
    println!("Enter \"quit\" to quit the game");

    let mut board = Board::new();
    println!("{}", board);

    while !board.legal_moves().is_empty() {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();

        if input.trim() == "moves" {
            let mut moves = String::new();
            match board.turn {
                Color::Black => {
                    moves.push_str("Black's moves: ");
                }
                Color::White => {
                    moves.push_str("White's moves: ");
                }
            }

            for posn in &board.legal_moves() {
                moves.push_str(&format!("{}, ", posn));
            }
            // Remove trailing comma and space
            moves.pop();
            moves.pop();
            println!("{}", moves);
            continue;
        }

        if input.trim() == "quit" {
            break;
        }

        if input.trim().len() != 2
            || !input.trim().chars().nth(0).unwrap().is_alphabetic()
            || !input.trim().chars().nth(1).unwrap().is_numeric()
        {
            println!("Invalid input");
            continue;
        }
        let posn = Posn::alphanumeric_to_posn(input.trim().to_string());
        if !board.is_legal(&posn) {
            println!("Invalid move");
            continue;
        }
        board = board.play_move(posn);
        println!("{}", board);
    }

    println!("Score: {:?}", board.score());
    match board.score() {
        (black, white) if black > white => println!("Black wins!"),
        (black, white) if white > black => println!("White wins!"),
        _ => println!("Tie!"),
    }
    */
}
