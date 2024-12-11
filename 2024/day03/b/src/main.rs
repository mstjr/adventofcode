use regex::Regex;

fn main() {
    let start = std::time::Instant::now();
    let input = include_str!("../../input.txt");
    let regex = Regex::new(r"(mul\(\d{1,3},\d{1,3}\)|don't\(\)|do\(\))").unwrap();
    let mut doing = true;
    let mut result = 0;
    regex.find_iter(input).for_each(|x| {
        if(x.as_str() == "do()") {
            doing = true;
            return;
        } else if(x.as_str() == "don't()") {
            doing = false;
            return;
        }

        if !doing {
            return;
        }

        //get the digits left and multiply to right
        let mut iter = x.as_str().split(",");
        let left = iter.next().unwrap().split("(").collect::<Vec<&str>>()[1].parse::<i32>().unwrap();
        let right = iter.next().unwrap().split(")").collect::<Vec<&str>>()[0].parse::<i32>().unwrap();
        result += left * right;
    });
    println!("{}", result);
    println!("Time: {:2?}", start.elapsed()); //Average time: 5ms
}
