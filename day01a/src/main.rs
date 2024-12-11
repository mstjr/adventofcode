pub fn main() {
    let start = std::time::Instant::now();
    let (mut a, mut b) = (Vec::with_capacity(1000), Vec::with_capacity(1000));
    let string = include_str!("../input.txt");

    for line in string.lines() {
        let whitespace = line.split_whitespace();
        //a b
        let mut iter = whitespace;
        a.push(iter.next().unwrap().parse::<i32>().unwrap());
        b.push(iter.next().unwrap().parse::<i32>().unwrap());
    }

    a.sort();
    b.sort();

    let mut i = 0;

    for count in 0..1000 {
        if a[count] > b[count] {
            i += (a[count] - b[count]);
        } else {
            i += (b[count] - a[count]);
        }
    }


    println!(
        "{}",
        i
    );

    println!("Time: {}μs", start.elapsed().as_micros()); //Average time: 1300μs
}