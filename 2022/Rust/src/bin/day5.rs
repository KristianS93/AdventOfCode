struct Results {
    column_1
}
let mut COLUMN_1: Vec<&str> = Vec::new();
let mut column_2: Vec<&str> = Vec::new();
let mut column_3: Vec<&str> = Vec::new();

fn main() {
    let now = std::time::Instant::now();

    //test


    //adding initial values
    column_1.push("Z");
    column_1.push("N");

    column_2.push("M");
    column_2.push("C");
    column_2.push("D");

    column_3.push("P");

    let input: &str = include_str!("./inputDay5.txt");

    //move x from y to x
    // split "move "
    //1 from 2 to 1
    // split " from "
    //1 2 to 1

    // let mut temp_part1: Vec<i32> = Vec::new();

    let values: Vec<(i32, i32, i32)> = input
        .split("\n")
        .map(|line| line.trim().split_once("move ").unwrap())
        .map(|(part1, part2)| {
            // println!("{}", part2);
            let (num1, x) = part2.split_once(" from ").unwrap();
            // println!("{}", num1);
            let (num2, num3) = x.split_once(" to ").unwrap();
            // println!("{}", num2);
            // println!("{}", num3);
            (
                num1.parse::<i32>().unwrap(),
                num2.parse::<i32>().unwrap(),
                num3.parse::<i32>().unwrap(),
            )
        })
        .collect();

    for x in values {
        println!("{}, {}, {}", x.0, x.1, x.2);
    }
    println!("Time elapsed: {:.2?}", now.elapsed());
}

//index 1 = move number (ie move 2, means move 2 entries)
//index 2 = from column 
//index 3 = to column
fn match_moves(val: Vec<(i32, i32, i32)>){

    let mut numerator = 0;
    for entry in val{

    }

}

fn matching(from: i32, to: i32){
    match from {
        1 => {
            let temp = column_1.pop();
            
        }
        _ => unreachable!(),
    }
}
