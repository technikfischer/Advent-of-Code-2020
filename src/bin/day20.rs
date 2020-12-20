use std::fs;
use std::str::FromStr;
use itertools::Itertools;
use advent_of_code::bitvector::BitVector;

const WIDTH: i32 = 10;
const GRID_SIZE: usize = 12;

type Orientation = (bool, bool, bool);

const ORIENTS: [Orientation; 8] = [
    (false, false, false),
    (false, false, true),
    (false, true, false),
    (false, true, true),
    (true, false, false),
    (true, false, true),
    (true, true, false),
    (true, true, true)
];

struct Image {
    id: u32,
    data: Vec<bool>,
    orientation: Orientation,
}

impl Image {
    fn new(id: u32, data: Vec<bool>) -> Self {
        Self {
            id,
            data,
            orientation: (false, false, false),
        }
    }

    fn get(&self, index: (i32, i32), (fx, fy, swap): &Orientation) -> bool {
        let (mut x, mut y) = if *swap {
            (index.1, index.0)
        } else {
            index
        };
        if *fx { x = WIDTH - 1 - x; }
        if *fy { y = WIDTH - 1 - y; }
        *self.data.get((y * WIDTH + x) as usize).unwrap()
    }
}

impl FromStr for Image {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let id = lines.next().unwrap();
        let id = id[5..9].parse().expect("Could not parse tile number");
        let data = lines.join("").chars().map(|c| c == '#').collect();
        Ok(Image::new(id, data))
    }
}

fn find_solution<'a>(images: &'a Vec<Image>, used: BitVector, placed: &mut Vec<(&'a Image, Orientation)>) -> bool {
    // try each image not already used
    let index = placed.len();

    // check for anchor if all images have been placed
    if index == images.len() {
        return true;
    }

    let row = index / GRID_SIZE;
    let col = index % GRID_SIZE;

    for (idx, image) in images.iter().enumerate() {
        if used.get(idx) { continue; }
        let b = used.set(idx);

        for orientation in ORIENTS.iter() {
            // left
            if col != 0 {
                let left = placed[index - 1];
                if (0..WIDTH)
                    .map(|i| (
                        image.get((i, 0), orientation),
                        left.0.get((i, WIDTH - 1), &left.1)
                    ))
                    .any(|(a, b)| a != b) {
                    continue;
                }
            }

            // up
            if row != 0 {
                let up = placed[index - GRID_SIZE]; // ultra hack
                if (0..WIDTH)
                    .map(|i| (
                        image.get((0, i), orientation),
                        up.0.get((WIDTH - 1, i), &up.1)
                    ))
                    .any(|(a, b)| a != b) {
                    continue;
                }
            }

            // tile fits
            placed.push((image, *orientation));
            if find_solution(images, b, placed) {
                return true;
            }
            placed.pop();
        }
    }

    false
}

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let images = input.split("\n\n").map(|s| s.parse::<Image>().unwrap()).collect_vec();

    let mut placed: Vec<(&Image, Orientation)> = Vec::new();
    find_solution(&images, BitVector::new(), &mut placed);

    const CORNERS: [usize; 4] = [0, GRID_SIZE - 1, GRID_SIZE * (GRID_SIZE - 1), GRID_SIZE * GRID_SIZE - 1];
    let product : u64 = CORNERS.iter().map(|i| placed[*i].0.id as u64).product();
    println!("Product of corners is {}", product);
}