use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("data/day01.txt").unwrap();
    let reader = BufReader::new(file);
    let (mut a, mut b) = reader
        .lines()
        .map_while(Result::ok)
        .map(|line| {
            let mut entries = line.split_whitespace();
            (
                entries.next().unwrap().parse::<usize>().unwrap(),
                entries.next().unwrap().parse::<usize>().unwrap(),
            )
        })
        .unzip::<_, _, Vec<_>, Vec<_>>();
    a.sort_unstable();
    b.sort_unstable();
    let total_distance: usize = a.into_iter().zip(b).map(|(a, b)| a.abs_diff(b)).sum();
    println!("Total Distance: {total_distance}");
}
