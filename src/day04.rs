use std::collections::HashSet;

const SIZE: usize = 5;

#[derive(Default, Debug)]
struct Board {
    /// Row-major list of numbers.
    numbers: Vec<u32>,
}

impl Board {
    fn has_bingo(&self, numbers: &HashSet<u32>) -> bool {
        for row in 0..SIZE {
            if self
                .numbers
                .iter()
                .skip(row * SIZE)
                .take(SIZE)
                .all(|x| numbers.contains(x))
            {
                return true;
            }
        }
        for col in 0..SIZE {
            if self
                .numbers
                .iter()
                .skip(col)
                .step_by(SIZE)
                .all(|x| numbers.contains(x))
            {
                return true;
            }
        }

        false
    }
}

fn parse(input: &str) -> (Vec<u32>, Vec<Board>) {
    let mut iter = input.lines();

    let numbers: Vec<u32> = iter
        .next()
        .unwrap()
        .split(",")
        .map(|x| x.parse::<u32>().unwrap())
        .collect();
    let mut boards = vec![];

    let mut temp_board = Board::default();
    for line in iter {
        temp_board.numbers.extend(
            line.trim()
                .split_whitespace()
                .map(|x| x.parse::<u32>().unwrap()),
        );

        assert!(temp_board.numbers.len() <= SIZE * SIZE);
        if temp_board.numbers.len() == SIZE * SIZE {
            boards.push(std::mem::take(&mut temp_board));
        }
    }

    assert_eq!(temp_board.numbers.len(), 0);
    (numbers, boards)
}

pub fn part1(input: String) {
    let (numbers, boards) = parse(&input);

    let mut picked = HashSet::<u32>::new();
    for number in numbers {
        picked.insert(number);

        for board in &boards {
            if board.has_bingo(&picked) {
                // Calculate score.
                let unmarked: u32 = board.numbers.iter().filter(|x| !picked.contains(x)).sum();
                let score = unmarked * number;
                println!("BINGO! Score = {}", score);
                return;
            }
        }
    }

    panic!("No bingo.");
}

pub fn part2(_input: String) {}
