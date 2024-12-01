use std::{
    collections::{BTreeMap, BTreeSet},
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    let file = File::open("data/day01.txt").unwrap();
    let reader = BufReader::new(file);
    let mut a = BTreeSet::<usize>::new();
    let mut b = BTreeMap::<usize, usize>::new();
    for line in reader.lines().map_while(Result::ok) {
        let mut entries = line.split_whitespace();
        a.insert(entries.next().unwrap().parse().unwrap());
        *b.entry(entries.next().unwrap().parse().unwrap())
            .or_default() += 1;
    }
    let similarity_score: usize = a
        .into_iter()
        .map(|a| a * b.get(&a).cloned().unwrap_or_default())
        .sum();
    println!("Similarity Score: {similarity_score}");
}
