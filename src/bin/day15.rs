use std::collections::HashMap;

fn main() {
    let input = "13,0,10,12,1,5,8";

    // part 1 and 2
    let mut spoken: HashMap<usize, usize> = input
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .enumerate()
        .map(|(idx, number)| (number, idx))
        .collect();

    let mut last_result = None;
    for turn in spoken.len()..30000000 {
        let new_number = match last_result {
            Some(t) => turn - 1 - t,
            None => 0
        };
        last_result = spoken.insert(new_number, turn);

        if turn == 2020 - 1 {
            println!("2020th number is {}", new_number);
        }

        if turn == 30000000 - 1 {
            println!("30000000th number is {}", new_number);
        }
    }
}