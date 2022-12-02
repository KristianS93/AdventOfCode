fn main() {
    let now = std::time::Instant::now();
    let input: &str = include_str!("./inputDay2.txt");

    let lines = input.split("\n");

    let mut result: i32 = 0;
    let mut result_of_play: i32 = 0;
    for line in lines {
        let each_play: Vec<&str> = line.split(" ").collect();
        result += check_win(each_play[0], each_play[1]);
        result_of_play += check_play(each_play[0], each_play[1]);
    }
    println!("{}", result);
    println!("{}", result_of_play);

    println!("Time elapsed: {:.2?}", now.elapsed());
}

fn check_win(play: &str, counter_play: &str) -> i32 {
    if play == "A" {
        match counter_play {
            x if x == "X" => 3 + 1, // draw 3 + rock gives 1
            x if x == "Y" => 6 + 2, // win 6 + paper gives 2
            x if x == "Z" => 0 + 3, // loose 0 + scissor gives 3
            _ => 0,
        }
    } else if play == "B" {
        match counter_play {
            x if x == "X" => 0 + 1, //  rock gives 1
            x if x == "Y" => 3 + 2, // paper gives 2
            x if x == "Z" => 6 + 3, // scissor gives 3
            _ => 0,
        }
    } else if play == "C" {
        match counter_play {
            x if x == "X" => 6 + 1, //win 6 + rock 1 = 7
            x if x == "Y" => 0 + 2, // loose 0 + paper 2 = 2
            x if x == "Z" => 3 + 3, // draw 3 + scissor 3 = 6
            _ => 0,
        }
    } else {
        println!("Xd");
        0
    }
}

// X means need to lose 0 point
// Y means need to draw 3 points
// Z means need to win  6 points

// A rock 1 point
// B paper 2 points
// C scissor 3 points

fn check_play(play: &str, counter_play: &str) -> i32 {
    if counter_play == "X" {
        match play {
            x if x == "A" => check_win(&x, &"Z"), // draw 3 + rock gives 1
            x if x == "B" => check_win(&x, &"X"), // win 6 + paper gives 2
            x if x == "C" => check_win(&x, &"Y"), // loose 0 + scissor gives 3
            _ => 0,
        }
    } else if counter_play == "Y" {
        match play {
            x if x == "A" => check_win(&x, &"X"), //  rock gives 1
            x if x == "B" => check_win(&x, &"Y"), // paper gives 2
            x if x == "C" => check_win(&x, &"Z"), // scissor gives 3
            _ => 0,
        }
    } else if counter_play == "Z" {
        match play {
            x if x == "A" => check_win(&x, &"Y"), //win 6 + rock 1 = 7
            x if x == "B" => check_win(&x, &"Z"), // loose 0 + paper 2 = 2
            x if x == "C" => check_win(&x, &"X"), // draw 3 + scissor 3 = 6
            _ => 0,
        }
    } else {
        println!("Xd");
        0
    }
}
