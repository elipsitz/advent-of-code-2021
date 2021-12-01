fn parse(input: &str) -> Vec<i32> {
    input.lines().map(|x| x.parse::<i32>().unwrap()).collect()
}

pub fn part1(input: String) {
    let input = parse(&input);
    let count = input.windows(2).filter(|x| x[1] > x[0]).count();
    println!("Increases: {}", count);
}

pub fn part2(input: String) {
    let input = parse(&input);
    let groups: Vec<i32> = input.windows(3).map::<i32, _>(|x| x.iter().sum()).collect();
    let count = groups.windows(2).filter(|x| x[1] > x[0]).count();
    println!("Increases: {}", count);
}
