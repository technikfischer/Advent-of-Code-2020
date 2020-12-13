use std::fs;
use itertools::{Itertools};

fn main() {
    let lines = fs::read_to_string("input").expect("Could not read file");
    let (departure, busses) = lines.lines().collect_tuple::<(&str, &str)>().unwrap();
    let departure: isize = departure.parse().expect("Could not parse departure time");
    let busses = busses.split(',')
        .filter(|s| *s != "x")
        .map(|s| s.parse::<isize>().expect("Could not parse bus id"))
        .sorted()
        .collect_vec();

    println!("Dep : {} Busses: {:?}", departure, busses);

    // part 1
    let first_available_bus = busses.iter().copied().min_by_key(|&id| (id - departure).rem_euclid(id)).unwrap();
    let waiting_time = (first_available_bus - departure).rem_euclid(first_available_bus);
    println!("Waiting time * bus id = {}", waiting_time * first_available_bus);

    // part 2
    // just read the input again and parse it appropriate
    let lines = fs::read_to_string("input").expect("Could not read file");
    let (_, busses) = lines.lines().collect_tuple::<(&str, &str)>().unwrap();
    let busses = busses.split(',')
        .enumerate()
        .filter(|(_, id)| *id != "x")
        .map(|(idx, id)| (idx, id.parse::<i64>().unwrap()))
        .collect_vec();

    // homepages.math.uic.edu/~leon/mcs425-s08/handouts/chinese_remainder.pdf
    let m = busses.iter().fold(1i64, |acc, elem| acc * elem.1);

    let mut z: Vec<i64> = Vec::new();
    let mut y: Vec<i64> = Vec::new();
    let mut w: Vec<i64> = Vec::new();
    let mut a: Vec<i64> = Vec::new();

    for (idx, mj) in busses {
        // first step, calculate z
        let zj = m / mj;
        z.push(zj);

        // second step, define y
        let yj: i64 = modinverse::modinverse(zj, mj).unwrap();
        y.push(yj);

        let wj = yj * zj;
        w.push(wj);

        // calculate a from
        // (t + idx) === a (mod mj)
        // t === (a - idx) (mod mj)
        // t === (a - idx) % mj (mod mj) to make a positive, but I suspect thats not even needed
        let aj = (mj - (idx as i64)).rem_euclid(mj);
        a.push(aj);
    }

    let t: i64 = a.iter()
        .zip(w)
        .map(|(a, w)| a * w)
        .sum::<i64>()
        .rem_euclid(m);
    println!("First timestamp to mach criteria{}", t);
}