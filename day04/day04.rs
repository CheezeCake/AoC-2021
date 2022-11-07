use std::io;

struct BingoBoard {
    board: [[u32; 5]; 5],
    marked: [[bool; 5]; 5],
}

impl BingoBoard {
    fn new() -> Self {
        Self {
            board: [[0; 5]; 5],
            marked: [[false; 5]; 5],
        }
    }

    fn set_number(&mut self, row: usize, col: usize, n: u32) {
        self.board[row][col] = n;
    }

    fn mark(&mut self, n: u32) -> bool {
        for row in 0..5 {
            for col in 0..5 {
                if self.board[row][col] == n {
                    self.marked[row][col] = true;
                    if self.win_row(row) || self.win_col(col) {
                        return true;
                    }
                }
            }
        }
        false
    }

    fn win_row(&self, row: usize) -> bool {
        (0..5).all(|dx| self.marked[row][dx])
    }

    fn win_col(&self, col: usize) -> bool {
        (0..5).all(|dy| self.marked[dy][col])
    }

    fn win(&self) -> bool {
        (0..5).any(|n| self.win_row(n) || self.win_col(n))

    }

    fn unmarked_numbers_sum(&self) -> u32 {
        self.board
            .iter()
            .enumerate()
            .map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .filter(move |&(x, _)| !self.marked[y][x])
                    .map(|(_, n)| *n)
            })
            .flatten()
            .sum()
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let random_numbers: Vec<u32> = input
        .split(',')
        .map(|n| {
            n.trim()
                .parse()
                .expect("error parsing random numbers in input")
        })
        .collect();
    let mut boards = Vec::new();

    loop {
        input.clear();
        match io::stdin().read_line(&mut input) {
            Ok(0) => break,
            Ok(1) => (),
            Ok(_) => panic!("expected empty line in input"),
            Err(_) => panic!("error while reading input"),
        }

        let mut board = BingoBoard::new();

        for y in 0..5 {
            input.clear();
            io::stdin()
                .read_line(&mut input)
                .expect("error while reading board from input");
            let nums: Vec<u32> = input
                .split(' ')
                .map(|n| n.trim())
                .filter(|n| n.len() > 0)
                .map(|n| n.parse().expect("error parsing number from board in input"))
                .collect();
            assert!(nums.len() == 5);
            for (x, n) in nums.iter().enumerate() {
                board.set_number(y, x, *n);
            }
        }

        boards.push(board);
    }

    let mut first_to_win_score: Option<u32> = None;
    let mut last_to_win_score: Option<u32> = None;

    for random_number in random_numbers {
        for board in boards.iter_mut().filter(|board| !board.win()) {
            if board.mark(random_number) {
                let score = board.unmarked_numbers_sum() * random_number;
                if let None = first_to_win_score {
                    first_to_win_score = Some(score);
                }
                last_to_win_score = Some(score);
            }
        }
    }

    println!("part 1: {}", first_to_win_score.unwrap());
    println!("part 2: {}", last_to_win_score.unwrap());
}
