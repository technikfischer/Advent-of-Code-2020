use std::fs;
use crate::AxialDirection::*;
use itertools::Itertools;
use std::collections::HashMap;
use std::ops::Add;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Coordinate {
    q: i32,
    r: i32,
}

impl Coordinate {
    fn new() -> Self {
        Self {
            q: 0,
            r: 0,
        }
    }
}

impl Add for Coordinate {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Coordinate {
            q: self.q + rhs.q,
            r: self.r + rhs.r,
        }
    }
}

enum AxialDirection {
    NW,
    NE,
    E,
    W,
    SE,
    SW,
}

impl AxialDirection {
    fn coord(&self) -> Coordinate {
        match self {
            AxialDirection::NW => Coordinate { q: 0, r: -1 },
            AxialDirection::NE => Coordinate { q: 1, r: -1 },
            AxialDirection::E => Coordinate { q: 1, r: 0 },
            AxialDirection::W => Coordinate { q: -1, r: 0 },
            AxialDirection::SE => Coordinate { q: 0, r: 1 },
            AxialDirection::SW => Coordinate { q: -1, r: 1 }
        }
    }
}

fn parse_line(line: &str) -> Vec<AxialDirection> {
    let mut line = &line.chars().collect_vec()[..];
    let mut coords = Vec::new();
    while line.len() > 0 {
        match line {
            ['n', 'w', rest @ ..] => {
                coords.push(NW);
                line = rest;
            }
            ['n', 'e', rest @ ..] => {
                coords.push(NE);
                line = rest;
            }
            ['s', 'w', rest @ ..] => {
                coords.push(SW);
                line = rest;
            }
            ['s', 'e', rest @ ..] => {
                coords.push(SE);
                line = rest;
            }
            ['e', rest @ ..] => {
                coords.push(E);
                line = rest;
            }
            ['w', rest @ ..] => {
                coords.push(W);
                line = rest;
            },
            _ => panic!("Invalid pattern")
        }
    }
    coords
}

fn main() {
    let input = fs::read_to_string("input").expect("Could not open file");
    let tiles_to_flip = input.lines().map(parse_line).collect_vec();

    // part 1
    let mut tiles = HashMap::new();
    for tile in tiles_to_flip.iter() {
        let flip = tile.iter().fold(Coordinate::new(), |coord, inst| coord + inst.coord());
        let entry = tiles.entry(flip).or_insert(false);
        *entry = !*entry;
    }

    let black_tiles = tiles.values().filter(|v| **v).count();
    println!("Count of black tiles {}", black_tiles);
}