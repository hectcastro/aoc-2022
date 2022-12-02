use aoc::read_file_input;

fn main() {
    let input = read_file_input("01.txt".to_string());

    println!("  Part 1: {}", part1(&input));
    println!("  Part 2: {}", part2(&input));
}

fn part1(input: &str) -> u32 {
    let calorie_count_batches = input.split("\n\n");

    let calorie_count_batch_sums = calorie_count_batches.map(|calorie_counts| {
        calorie_counts
            .lines()
            .map(|calorie_count| calorie_count.parse::<u32>().unwrap())
            .sum::<u32>()
    });

    calorie_count_batch_sums.max().unwrap()
}

fn part2(input: &str) -> u32 {
    let calorie_count_batches = input.split("\n\n");

    let mut calorie_count_batch_sums = calorie_count_batches
        .map(|calorie_counts| {
            calorie_counts
                .lines()
                .map(|calorie_count| calorie_count.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .collect::<Vec<_>>();

    // Sort calorie count sums in descending order.
    calorie_count_batch_sums.sort_by(|a, b| b.cmp(a));

    calorie_count_batch_sums.iter().take(3).sum()
}
