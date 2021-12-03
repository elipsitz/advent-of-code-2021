const BITS: usize = 12;
const MASK: u32 = (1 << BITS) - 1;

fn parse(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(|line| u32::from_str_radix(line, 2).unwrap())
        .collect()
}

fn calculate_gamma_epsilon(input: &[u32]) -> (u32, u32) {
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
            let bit = (2 * count >= input.len()) as u32;
            bit << i
        })
        .fold(0, |a, b| (a | b));
    let epsilon = (!gamma) & MASK;

    (gamma, epsilon)
}

pub fn part1(input: String) {
    let input = parse(&input);

    let (gamma, epsilon) = calculate_gamma_epsilon(&input);
    println!("Combined: {}", gamma * epsilon);
}

pub fn part2(input: String) {
    let input = parse(&input);

    let mut oxygen = input.clone();
    for bit in (0..BITS).rev() {
        let (gamma, _) = calculate_gamma_epsilon(&oxygen);
        oxygen.retain(|&x| ((x >> bit) & 1) == ((gamma >> bit) & 1));
        if oxygen.len() == 1 {
            break;
        }
    }

    let mut co2 = input.clone();
    for bit in (0..BITS).rev() {
        let (_, epsilon) = calculate_gamma_epsilon(&co2);
        co2.retain(|&x| ((x >> bit) & 1) == ((epsilon >> bit) & 1));
        if co2.len() == 1 {
            break;
        }
    }

    assert_eq!(oxygen.len(), 1);
    assert_eq!(co2.len(), 1);
    println!("combined: {}", oxygen[0] * co2[0]);
}
