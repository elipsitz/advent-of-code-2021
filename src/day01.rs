fn parse(input: &str) -> Vec<i32> {
    input.lines().map(|x| x.parse::<i32>().unwrap()).collect()
}

pub fn part1(input: String) {
    let input = parse(&input);
    let count = input.windows(2).filter(|x| x[1] > x[0]).count();
    println!("Increases: {}", count);
}
