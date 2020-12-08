use std::io::{BufReader, BufRead};
use std::fs::{File};

extern crate regex;

use regex::{Regex};

struct Passport {
    byr: Option<usize>,
    iyr: Option<usize>,
    eyr: Option<usize>,
    hgt: Option<String>,
    hcl: Option<String>,
    ecl: Option<String>,
    pid: Option<String>,
    cid: Option<usize>,
}

struct PassportReader {
    buffered_reader: BufReader<File>
}

impl PassportReader {
    fn new(reader: BufReader<File>) -> PassportReader {
        PassportReader {
            buffered_reader: reader
        }
    }
}

impl Iterator for PassportReader {
    type Item = Passport;

    fn next(&mut self) -> Option<Self::Item> {
        return read_passport_from_reader(&mut self.buffered_reader);
    }
}

fn read_passport_from_reader(buf: &mut BufReader<File>) -> Option<Passport> {
    let mut passport = Passport {
        byr: None,
        iyr: None,
        eyr: None,
        hgt: None,
        hcl: None,
        ecl: None,
        pid: None,
        cid: None,
    };

    let mut read_property = false;

    loop {
        let mut buffer: String = String::new();
        if buf.read_line(&mut buffer).unwrap() == 0 {
            break;
        }

        if buffer == "\n" {
            // check for newline after some properties - skip if read starts with newline
            if read_property { break; } else { continue; }
        }

        // newline is defined as whitespace in ascii
        for property in buffer.split_ascii_whitespace() {
            let mut property_iter = property.split(':');
            let name = property_iter.next().expect("could not get property name");
            let value = property_iter.next().expect("could not get property value");
            let value_int = value.parse::<usize>();

            match name {
                "byr" => passport.byr = Some(value_int.expect("Could not parse byr")),
                "iyr" => passport.iyr = Some(value_int.expect("Could not parse iyr")),
                "eyr" => passport.eyr = Some(value_int.expect("Could not parse eyr")),
                "hgt" => passport.hgt = Some(value.to_string()),
                "hcl" => passport.hcl = Some(value.to_string()),
                "ecl" => passport.ecl = Some(value.to_string()),
                "pid" => passport.pid = Some(value.to_string()),
                "cid" => passport.cid = Some(value_int.expect("Could not parse cid")),
                prop @ _ => panic!(format!("Invalid property name {}", prop))
            }

            read_property = true;
        }
    };

    if read_property {
        Some(passport)
    } else {
        None
    }
}

fn in_bounds(min: usize, val: usize, max: usize) -> bool {
    return min <= val && val <= max;
}

fn main() {
    let passport_reader = PassportReader::new(BufReader::new(File::open("input").unwrap()));
    let passports: Vec<Passport> = passport_reader.collect();

    // part 1
    let mut valid = 0;
    for passport in passports.iter() {
        if let Passport { byr: Some(_), cid: _, ecl: Some(_), eyr: Some(_), hcl: Some(_), iyr: Some(_), pid: Some(_), hgt: Some(_) } = passport {
            valid += 1;
        }
    }

    println!("Valid passports {}", valid);

    // part 2
    let hair_color_regex = Regex::new(r"^#[0-9a-f]{6}$").unwrap();
    let height_regex = Regex::new(r"^(?P<value>\d+)(?P<unit>cm|in)$").unwrap();
    let eye_color_regex = Regex::new("^(amb|blu|brn|gry|grn|hzl|oth)$").unwrap();
    let passport_id_regex = Regex::new(r"^\d{9}$").unwrap();

    let mut valid = 0;
    for passport in passports {
        match passport {
            Passport { byr: Some(byr), cid: _, ecl: Some(ecl), eyr: Some(eyr), hcl: Some(hcl), iyr: Some(iyr), pid: Some(pid), hgt: Some(hgt) }
            if in_bounds(1920, byr, 2002)
                && in_bounds(2010, iyr, 2020)
                && in_bounds(2020, eyr, 2030)
                && eye_color_regex.is_match(ecl.as_ref())
                && passport_id_regex.is_match(pid.as_ref())
                && hair_color_regex.is_match(hcl.as_ref())
            => {
                // check height
                if let Some(group) = height_regex.captures(hgt.as_ref()) {
                    match (group.name("value").map(|m| m.as_str().parse()), group.name("unit").map(|m| m.as_str())) {
                        (Some(Ok(height)), Some("cm")) if in_bounds(150, height, 193) => {}
                        (Some(Ok(height)), Some("in")) if in_bounds(59, height, 76) => {}
                        _ => { continue; }
                    }
                } else {
                    continue;
                }

                valid += 1;
            }
            _ => ()
        }
    }
    println!("Valid passports to new criteria {}", valid);
}
