enum Command {
    Forward(usize),
    Down(usize),
    Up(usize),
}

fn parse(input: &str) -> Vec<Command> {
    input
        .lines()
        .map(|line| {
            let mut iter = line.split(" ");
            let kind = iter.next().unwrap();
            let amount = iter.next().unwrap().parse::<usize>().unwrap();
            match kind {
                "forward" => Command::Forward(amount),
                "down" => Command::Down(amount),
                "up" => Command::Up(amount),
                _ => unreachable!(),
            }
        })
        .collect()
}

pub fn part1(input: String) {
    let input = parse(&input);
    let mut horizontal = 0usize;
    let mut depth = 0usize;
    for command in input {
        match command {
            Command::Forward(x) => horizontal += x,
            Command::Down(x) => depth += x,
            Command::Up(x) => depth -= x,
        }
    }
    println!("Horizontal {}, Depth {}", horizontal, depth);
    println!("Combined: {}", horizontal * depth);
}
