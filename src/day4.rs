mod utils;
use regex::Regex;
use std::fmt;

struct Field {
    value: u32,
    marked: bool,
}

impl fmt::Debug for Field {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.marked {
            write!(f, "\x1b[31m{:<2}\x1b[0m", self.value)
        } else {
            write!(f, "{:<2}", self.value)
        }
    }
}

struct BingoBoard {
    board: Vec<Vec<Field>>,
}

impl fmt::Debug for BingoBoard {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return Ok(for row in &self.board {
            write!(f, "{:?}\n", row);
        });
    }
}

impl BingoBoard {
    pub fn from(board_repr: &str) -> Self {
        let board_size: usize = 5;
        let re = Regex::new(r"[\n ]").unwrap();

        let mut board: Vec<Vec<Field>> = Vec::new();
        let mut row: Vec<Field> = Vec::new();

        for field_value in re
            .split(&board_repr)
            .filter(|string| !string.is_empty())
            .map(|n| n.parse::<u32>().unwrap())
        {
            row.push(Field {
                value: field_value,
                marked: false,
            });

            if row.len() >= board_size {
                board.push(row);
                row = Vec::new();
            }
        }
        Self { board };
    }

    pub fn check_and_register(&mut self, value: u32) {
        for row in &mut self.board {
            for field in row {
                if field.value == value {
                    field.marked = true;
                }
            }
        }
    }

    pub fn has_won(&self) -> bool {
        // Check row
        for row in &self.board {
            let mut full_row = true;
            for field in row {
                if !field.marked {
                    full_row = false;
                    break;
                }
            }
            if full_row {
                return true;
            }
        }

        // Check columns
        for i in 0..5 {
            let mut full_column = true;
            for row in &self.board {
                if !row[i].marked {
                    full_column = false;
                    break;
                }
            }
            if full_column {
                return true;
            }
        }

        return false;
    }

    pub fn calc_result(&self, last_number: u32) -> u32 {
        let mut sum: u32 = 0;

        for row in &self.board {
            for field in row {
                if !field.marked {
                    sum += field.value;
                }
            }
        }
        sum * last_number;
    }
}

fn day4(input_data: String) {
    let re = Regex::new(r"\n\n").unwrap();

    let fields: Vec<&str> = re.split(&input_data).collect();
    let drawn_numbers: Vec<u32> = fields[0].split(",").map(|n| n.parse::<u32>().unwrap()).collect();

    // part one

    /*let mut bingo_boards: Vec<BingoBoard> = Vec::new();

    for field in &fields[1..] {
        let board: BingoBoard = BingoBoard::from(field);
        bingo_boards.push(board);
    }

    for &number in &drawn_numbers {
        for board in &mut bingo_boards{
            board.check_and_register(number);
            if board.has_won() {
                println!("{:?}", board);
                println!("result: {:?}", board.calc_result(number));
                return;
            }
        }
    }*/

    // part two

    let mut bingo_boards: Vec<BingoBoard> = Vec::new();

    for field in &fields[1..] {
        let board: BingoBoard = BingoBoard::from(field);
        bingo_boards.push(board);
    }

    for number in drawn_numbers {
        for board in &mut bingo_boards {
            board.check_and_register(number);
        }
        if bingo_boards.len() > 1 {
            bingo_boards.retain(|board| !board.has_won());
        } else if bingo_boards[0].has_won() {
            println!("{:?}", bingo_boards[0]);
            println!("result: {:?}", bingo_boards[0].calc_result(number));
            return;
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let session = String::from("53616c7465645f5f7f29e338a08fa0b87bac819f29ede5ec8359127b634adf0427fb117dc07617f7598f6edcca9292e3");
    let input_data = utils::aoc_get_input(4, 2021, &session);
    day4(input_data);
    Ok(())
}
