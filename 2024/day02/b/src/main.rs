fn main() {
    let start = std::time::Instant::now();
    let input = include_str!("../../input.txt");
    let mut safe_reports = 0;

    input.split("\n").for_each(|line| {
        if is_safe_report_with_dampener(line) {
            safe_reports += 1;
        }
    });

    println!("{}", safe_reports);
    println!("Time: {}μs", start.elapsed().as_micros()); //Average time: 3000μs
}

fn is_safe_report_with_dampener(report: &str) -> bool {
    let levels: Vec<i32> = report.split_whitespace().filter_map(|s| s.parse().ok()).collect();

    if is_safe_report(&levels) {
        return true;
    }

    for i in 0..levels.len() {
        let mut modified_levels = levels.clone();
        modified_levels.remove(i);
        if is_safe_report(&modified_levels) {
            return true;
        }
    }

    false
}

fn is_safe_report(levels: &[i32]) -> bool {
    let mut differences: Vec<i32> = levels.windows(2).map(|w| w[1] - w[0]).collect();

    if !differences.iter().all(|&diff| diff.abs() >= 1 && diff.abs() <= 3) {
        return false;
    }

    let is_increasing = differences.iter().all(|&diff| diff > 0);
    let is_decreasing = differences.iter().all(|&diff| diff < 0);

    is_increasing || is_decreasing
}