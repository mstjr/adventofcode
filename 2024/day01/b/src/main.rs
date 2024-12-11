use std::collections::HashMap;

pub fn main() {
    let start = std::time::Instant::now();
    let (mut a, mut b) = (Vec::with_capacity(1000), Vec::with_capacity(1000));
    let string = include_str!("../../input.txt");

    for line in string.lines() {
        let mut iter = line.split_whitespace();
        a.push(iter.next().unwrap().parse::<i32>().unwrap());
        b.push(iter.next().unwrap().parse::<i32>().unwrap());
    }

    // Comptez les occurrences de chaque valeur dans `b`
    let mut b_count = HashMap::new();
    for &value in &b {
        *b_count.entry(value).or_insert(0) += 1;
    }

    // Calculez le résultat avec une seule boucle
    let mut i = 0;
    for &value in &a {
        if let Some(&count_b) = b_count.get(&value) {
            i += value * count_b;
        }
    }

    println!("{}", i);
    println!("Time: {}μs", start.elapsed().as_micros()); //Average time: 1400μs
}
