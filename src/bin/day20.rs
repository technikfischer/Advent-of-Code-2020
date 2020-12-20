use std::fs;
use std::str::FromStr;
use itertools::Itertools;
use advent_of_code::bitvector::BitVector;

const WIDTH: usize = 10;
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

#[derive(Clone)]
struct Image {
    id: u32,
    pub data: Vec<bool>,
    side: usize,
}

impl Image {
    fn new(id: u32, data: Vec<bool>, side: usize) -> Self {
        Self {
            id,
            data,
            side,
        }
    }

    fn get(&self, index: (usize, usize), (fx, fy, swap): &Orientation) -> bool {
        *self.data.get(self.idx(index, fx, fy, swap)).unwrap_or_else(|| &false)
    }

    fn idx(&self, index: (usize, usize), fx: &bool, fy: &bool, swap: &bool) -> usize {
        let (mut x, mut y) = if *swap {
            (index.1, index.0)
        } else {
            index
        };
        if *fx { x = self.side - 1 - x; }
        if *fy { y = self.side - 1 - y; }
        y * self.side + x
    }

    fn set(&mut self, index: (usize, usize), (fx, fy, swap): &Orientation, val: bool) {
        let idx = self.idx(index, fx, fy, swap);
        self.data[idx] = val;
    }
}

impl FromStr for Image {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let id = lines.next().unwrap();
        let id = id[5..9].parse().expect("Could not parse tile number");
        let data = lines.join("").chars().map(|c| c == '#').collect();
        Ok(Image::new(id, data, WIDTH as usize))
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
                        image.get((i as usize, 0), orientation),
                        left.0.get((i as usize, (WIDTH - 1) as usize), &left.1)
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
                        image.get((0, i as usize), orientation),
                        up.0.get(((WIDTH - 1) as usize, i as usize), &up.1)
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

fn is_monster(monster: &[[bool; 20]; 3], image: &Image, orient: &Orientation, row: usize, col: usize) -> bool {
    for c in 0..20 {
        for r in 0..3 {
            if !image.get((r + row, c + col), orient) && monster[r][c] {
                return false;
            }
        }
    }
    true
}

fn mark_monster(monster: &[[bool; 20]; 3], monster_parts: &mut Image, orient: &Orientation, row: usize, col: usize) {
    for c in 0..20 {
        for r in 0..3 {
            if monster[r][c] {
                monster_parts.set((r + row, c + col), orient, false);
            }
        }
    }
}

fn main() {
    let input = fs::read_to_string("input").unwrap();
    let images = input.split("\n\n").map(|s| s.parse::<Image>().unwrap()).collect_vec();

    let mut placed: Vec<(&Image, Orientation)> = Vec::new();
    find_solution(&images, BitVector::new(), &mut placed);

    const CORNERS: [usize; 4] = [0, GRID_SIZE - 1, GRID_SIZE * (GRID_SIZE - 1), GRID_SIZE * GRID_SIZE - 1];
    let product: u64 = CORNERS.iter().map(|i| placed[*i].0.id as u64).product();
    println!("Product of corners is {}", product);

    const T: bool = true;
    const F: bool = false;
    let monster = [
        [F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, T, F],
        [T, F, F, F, F, T, T, F, F, F, F, T, T, F, F, F, F, T, T, T],
        [F, T, F, F, T, F, F, T, F, F, T, F, F, T, F, F, T, F, F, F]
    ];

    let mut image_data = Vec::new();
    image_data.reserve(12 * 10 - 10 * 2);

    for image_row in 0..GRID_SIZE {
        for single_row in 1..9 {
            for image_col in 0..GRID_SIZE {
                for single_col in 1..9 {
                    let tile = placed[image_row + GRID_SIZE * image_col];
                    image_data.push(tile.0.get((single_col, single_row), &tile.1));
                }
            }
        }
    }

    const SIDE: usize = GRID_SIZE * WIDTH - GRID_SIZE * 2;
    assert_eq!(image_data.len(), SIDE * SIDE);
    let image = Image::new(0, image_data, SIDE);

    for orientation in ORIENTS.iter() {
        let mut monsters = 0;
        let mut part_of_monster = image.clone();

        // swapped to calculate col maximum only once
        for col in 0..SIDE - monster[0].len() {
            for row in 0..SIDE - 3 {
                if is_monster(&monster, &image, orientation, row, col) {
                    monsters += 1;
                    mark_monster(&monster, &mut part_of_monster, orientation, row, col);
                }
            }
        }

        let roughness = part_of_monster.data.iter().filter(|p| **p).count();
        if monsters > 0 {
            println!("Orient {:?} are {} monsters", orientation, monsters);
            println!("Roughness {}", roughness);
        }
    }
}