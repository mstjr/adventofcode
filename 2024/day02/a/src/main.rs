fn main() {
    let start = std::time::Instant::now();
    let input = include_str!("../../input.txt");
    let mut safe_reports = 0;

    input.split("\n").for_each(|line| {
        if is_safe_report(line) {
            safe_reports += 1;
        }
    });

    println!("{}", safe_reports);
    println!("Time: {}μs", start.elapsed().as_micros()); //Average time: 2000μs
}

fn is_safe_report(report: &str) -> bool {
    let levels: Vec<i32> = report.split_whitespace().filter_map(|s| s.parse().ok()).collect();

    let mut differences: Vec<i32> = levels.windows(2).map(|w| w[1] - w[0]).collect();

    if !differences.iter().all(|&diff| diff.abs() >= 1 && diff.abs() <= 3) {
        return false;
    }

    let is_increasing = differences.iter().all(|&diff| diff > 0);
    let is_decreasing = differences.iter().all(|&diff| diff < 0);

    is_increasing || is_decreasing
}