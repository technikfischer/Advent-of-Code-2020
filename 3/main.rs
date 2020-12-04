use std::fs;

fn line_to_vector(line: &str) -> Vec<bool> {
    line.chars().map(|c| c == '#').collect()
}

fn trees_in_the_way(forest: &Vec<Vec<bool>>, slope_down: usize, slope_right: usize) -> i32 {
    let mut line = 0;
    let mut col = 0;
    let mut trees = 0;

    while line < forest.len() {
        if forest[line][col % forest[line].len()] {
            trees += 1;
        }

        line += slope_down;
        col += slope_right;
    }
    trees
}

fn main() {
    let data = fs::read_to_string("input").expect("Unable to read file");
    let forest: Vec<Vec<bool>> = data.lines()
        .map(line_to_vector)
        .collect();

    // exercise 1
    let mut trees = 0;
    for (line_idx, line) in forest.iter().enumerate() {
        // move 3 for each line, and repeat pattern
        if line[(line_idx * 3) % line.len()] {
            trees += 1;
        }
    }

    println!("Trees on the way {}", trees);

    //exercise 2
    let trees = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)].iter()
        .map(|&(right, down)| trees_in_the_way(&forest, down, right))
        .fold(1, |a, b| a * b);

    println!("Trees on all ways {}", trees);
}
