const BITS: usize = 12;
const MASK: u32 = (1 << BITS) - 1;

fn parse(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(|line| u32::from_str_radix(line, 2).unwrap())
        .collect()
}

pub fn part1(input: String) {
    let input = parse(&input);

    // Count of 1 bits in each position.
    let counts = (0..BITS).map(|index| {
        input
            .iter()
            .map(|&num| (num >> index) & 1)
            .filter(|&x| x != 0)
            .count()
    });

    let gamma = counts
        .enumerate()
        .map(|(i, count)| {
            let bit = (count >= (input.len() / 2)) as u32;
            bit << i
        })
        .fold(0, |a, b| (a | b));

    let epsilon = (!gamma) & MASK;

    println!("Combined: {}", gamma * epsilon);
}

pub fn part2(_input: String) {}
