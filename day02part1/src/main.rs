use nom::{
    character::complete::{digit1, newline, space1},
    combinator::map_res,
    multi::separated_list1,
    IResult,
};
use std::{
    fs::File,
    io::{BufReader, Read},
    str::FromStr,
};

fn parse_report(row: &str) -> IResult<&str, Vec<u32>> {
    separated_list1(space1, map_res(digit1, FromStr::from_str))(row)
}

fn parse_manifest(manifest: &str) -> IResult<&str, Vec<Vec<u32>>> {
    separated_list1(newline, parse_report)(manifest)
}

fn main() {
    let file = File::open("data/day02.txt").unwrap();
    let mut reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents).unwrap();
    let (_, manifest) = parse_manifest(&contents).unwrap();
    let num_safe = manifest
        .iter()
        .filter(|report| {
            (report.iter().is_sorted() || report.iter().rev().is_sorted())
                && report
                    .windows(2)
                    .all(|window| (1..=3).contains(&window[0].abs_diff(window[1])))
        })
        .count();
    println!("Num Safe: {num_safe}");
}
