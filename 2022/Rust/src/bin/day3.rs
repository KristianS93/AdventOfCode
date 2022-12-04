fn main() {
    let now = std::time::Instant::now();
    let input: &str = include_str!("./inputDay4.txt");
    let lines = input
        .split("\n") //Split at new line
        .map(|line| line.split_at(line.len() / 2)) //map each line however as a split string 50/50
        .map(|(string_1, string_2)| {
            //take the tuple: think (k, v)  add closure
            string_2
                .chars() // get string_2 as chars that also gives us an iterator
                .filter(|b| string_1.contains(*b)) // filter out based on a new closure taking a &char from the iterator
                // then we check if a contains the dereferenced b, if this is true we continue
                .map(|b| {
                    // the value that did exist in both strings, is then matched
                    if b >= 'a' {
                        b as u32 - 96
                    } else {
                        b as u32 - 38
                    }
                })
                .next()
                .unwrap()
        })
        .sum::<u32>();

    println!("{}", lines);
    println!("Time elapsed: {:.2?}", now.elapsed());
}

// pub fn main() {
//     let now = std::time::Instant::now();
//     println!(
//         "{}",
//         include_bytes!("./inputDay3.txt")
//             .split(|b| *b == b'\n')
//             .map(|l| l.split_at(l.len() / 2))
//             .map(|(a, b)| b
//                 .iter()
//                 .filter(|b| a.contains(b))
//                 .map(|b| if *b >= b'a' {
//                     (b - b'a') as i16 + 1
//                 } else {
//                     (b - b'A') as i16 + 27
//                 })
//                 .next()
//                 .unwrap())
//             .sum::<i16>(),
//     );
//     println!("Time elapsed: {:.2?}", now.elapsed());
// }
