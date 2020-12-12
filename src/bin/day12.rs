use std::str::FromStr;
use advent_of_code::get_input;


struct ShipPosition {
    ew: i32,
    ns: i32,
    facing: i32,
}

struct NavInstr {
    action: char,
    value: i32,
}

impl FromStr for NavInstr {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(NavInstr {
            action: s.chars().nth(0).expect("Could not get action char"),
            value: s[1..].parse().expect("Could not parse integer value"),
        })
    }
}

fn apply_instruction(nav_instr: &NavInstr, position: &ShipPosition) -> ShipPosition {
    match nav_instr.action {
        'N' => ShipPosition { ns: position.ns + nav_instr.value, ..*position },
        'S' => ShipPosition { ns: position.ns - nav_instr.value, ..*position },
        'E' => ShipPosition { ew: position.ew + nav_instr.value, ..*position },
        'W' => ShipPosition { ew: position.ew - nav_instr.value, ..*position },
        'L' => ShipPosition { facing: (position.facing - nav_instr.value / 90).rem_euclid(4), ..*position },
        'R' => ShipPosition { facing: (position.facing + nav_instr.value / 90).rem_euclid(4), ..*position },
        'F' => match position.facing {
            0 => ShipPosition { ns: position.ns + nav_instr.value, ..*position },
            1 => ShipPosition { ew: position.ew + nav_instr.value, ..*position },
            2 => ShipPosition { ns: position.ns - nav_instr.value, ..*position },
            3 => ShipPosition { ew: position.ew - nav_instr.value, ..*position },
            _ => panic!("Invalid ship facing")
        }
        _ => panic!("Invalid navigation instruction")
    }
}

fn main() {
    let instructions: Vec<NavInstr> = get_input();

    //part 1
    let start_pos = ShipPosition { ns: 0, ew: 0, facing: 1 };
    let end_position = instructions.iter().fold(start_pos, |pos, nav| apply_instruction(nav, &pos));
    let manhatten_distance = end_position.ns.abs() + end_position.ew.abs();
    println!("Manhatten distance from start is {}", manhatten_distance);
}