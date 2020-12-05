use std::fs;

fn convert_to_seat_index(seat_string: &str) -> usize {
    let binary = seat_string.replace('F', "0").replace('B', "1").replace('L', "0").replace('R', "1");
    usize::from_str_radix(binary.as_str(), 2).expect("Could not parse binary data")
}

fn main() {
    let data = fs::read_to_string("input").expect("Unable to read file");
    let data = data.lines();

    let seat_ids = data.map(convert_to_seat_index).collect::<Vec<usize>>();

    // part 1
    let highest_seat_id = seat_ids.iter().max().unwrap();
    println!("Highest seat ID is {}", highest_seat_id);

    //part 2

    for seat_id in 1..*highest_seat_id {
        if seat_ids.contains(&(seat_id - 1)) && !seat_ids.contains(&seat_id) && seat_ids.contains(&(seat_id + 1)) {
            printls!("Empty seat surounded with used seats: {}", seat_id);
        }
    }
}
