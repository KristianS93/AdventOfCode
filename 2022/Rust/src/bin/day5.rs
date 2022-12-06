fn main() {
    let input: &str = include_str!("./inputDay5.txt");

    //test vectors

    let column_1 = vec!["F", "T", "C", "L", "R", "P", "G", "Q"];
    let column_2 = vec!["N", "Q", "H", "W", "R", "F", "S", "J"];
    let column_3 = vec!["F", "B", "H", "W", "P", "M", "Q"];
    let column_4 = vec!["V", "S", "T", "D", "F"];
    let column_5 = vec!["Q", "L", "D", "W", "V", "F", "Z"];
    let column_6 = vec!["Z", "C", "L", "S"];
    let column_7 = vec!["Z", "B", "M", "V", "D", "F"];
    let column_8 = vec!["T", "J", "B"];
    let column_9 = vec!["Q", "N", "B", "G", "L", "S", "P", "H"];

    let mut columns = vec![
        column_1, column_2, column_3, column_4, column_5, column_6, column_7, column_8, column_9,
    ];

    let xxx = input.split('\n').map(|line| {
        line.split_whitespace()
            .filter_map(|s| s.parse::<usize>().ok())
            .collect::<Vec<usize>>()
    });

    // column_1.pop()
    // columns[1].pop()

    for inst in xxx {
        //move 1 from 2 to 1
        for _ in 0..inst[0] {
            let from = inst[1] - 1;
            let to = inst[2] - 1;
            let temp = columns[from].pop().unwrap();
            columns[to].push(temp);
        }
    }

    println!(
        "{}{}{}{}{}{}{}{}{}",
        columns[0].pop().unwrap(),
        columns[1].pop().unwrap(),
        columns[2].pop().unwrap(),
        columns[3].pop().unwrap(),
        columns[4].pop().unwrap(),
        columns[5].pop().unwrap(),
        columns[6].pop().unwrap(),
        columns[7].pop().unwrap(),
        columns[8].pop().unwrap()
    );
}
