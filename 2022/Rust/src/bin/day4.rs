fn main() {
    let now = std::time::Instant::now();

    let input: &str = include_str!("./inputDay4.txt");

    let y = input
        .split("\n")
        .map(|line| line.trim().split_once(",").unwrap())
        .map(|(range_1, range_2)| {
            (
                (
                    range_1.split_once("-").unwrap().0.parse::<i32>().unwrap(),
                    range_1.split_once("-").unwrap().1.parse::<i32>().unwrap(),
                ),
                (
                    range_2.split_once("-").unwrap().0.parse::<i32>().unwrap(),
                    range_2.split_once("-").unwrap().1.parse::<i32>().unwrap(),
                ),
            )
        })
        .map(
            |((first_lower, first_upper), (second_lower, second_upper))| {
                //part 1
                if (first_lower <= second_lower && first_upper >= second_upper)
                    || (second_lower <= first_lower && second_upper >= first_upper)
                {
                    1
                } else if (second_upper >= first_upper && second_lower <= first_upper) //part2
                    || (first_upper >= second_upper && first_lower <= second_upper)
                {
                    1
                } else {
                    0
                }
            },
        )
        .sum::<i32>();

    println!("{:?}", y);

    println!("Time elapsed: {:.2?}", now.elapsed());
}
