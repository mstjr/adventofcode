use std::collections::HashMap;
pub fn main(){
    let start = std::time::Instant::now();
    let mut sum = 0;

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

    let lines = split.next().unwrap();
    for s in lines.lines() {
        let pages = s.split(",");
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

        if !valid {
            sum += middle(correctly_order(pages_vec, &rules));
        }else {
            continue;
        }
    }

    println!("{}", sum);
    println!("Time: {}μs", start.elapsed().as_micros()); //Average time: 2300μs

}

fn correctly_order(vec: Vec<usize>, rules: &HashMap<usize, Vec<usize>>) -> Vec<usize> {

    //reorder the vec to match the rules (if i in vec is before all of values in the vector of i in rules) then ok, other need to be before

    let mut correct_order = vec![];
    for x in vec.clone() {
        let mut pos = correct_order.len();
        if let Some(orders) = rules.get(&x) {
            for (i, page) in correct_order.iter().enumerate() {
                if orders.contains(page) {
                    pos = i;
                    break;
                }
            }
        }
        correct_order.insert(pos, x);
    }

    correct_order
}

fn middle(vec: Vec<usize>) -> usize {
    vec[vec.len() / 2]
}
