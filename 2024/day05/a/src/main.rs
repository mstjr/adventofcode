use std::collections::HashMap;

pub fn main() {
    let start = std::time::Instant::now();
    let input = include_str!("../../input.txt");
    let mut split = input.split("\n\n");

    let mut rules = HashMap::new();

    let lines = split.next().unwrap();
    for s in lines.lines() {
        let mut split = s.split("|");
        let key = split.next().unwrap().parse::<usize>().unwrap();
        let value = split.next().unwrap().parse::<usize>().unwrap();
        rules.entry(key).or_insert(vec![]).push(value);
    }

    for (_, v) in rules.iter_mut() {
        v.sort();
    }

    let mut sum = 0;

    let lines = split.next().unwrap();
    for s in lines.lines() {
        let mut pages = s.split(",");
        let mut pages_vec = vec![];
        for page in pages {
            pages_vec.push(page.parse::<usize>().unwrap());
        }

        let mut valid = true;

        for (i, page) in pages_vec.iter().enumerate() {
            if let Some(orders) = rules.get(page) {
                if pages_vec[0..i].iter().any(|&page| orders.binary_search(&page).is_ok()) {
                    valid = false;
                    break;
                }
            }
        }

        if valid {
            sum += pages_vec[pages_vec.len() / 2];
        }
    }

    println!("{}", sum);
    println!("Time: {}μs", start.elapsed().as_micros()); //Average time: 2300μs
}
