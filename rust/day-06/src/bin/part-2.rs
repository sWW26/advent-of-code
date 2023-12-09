fn main() {
    let input = include_str!("input1.txt");

    let answer = solve(input);

    println!("{}", answer);
}

fn solve(str: &str) -> u64 {
    let mut lines = str.lines();
    let t = lines
        .next()
        .expect("has at least one line")
        .split_once(':')
        .expect("starts 'Time:'")
        .1
        .split_ascii_whitespace()
        .collect::<String>()
        .parse::<u64>()
        .expect("time is a number");
    let d = lines
        .next()
        .expect("has at least two lines")
        .split_once(':')
        .expect("starts 'Distance'")
        .1
        .split_ascii_whitespace()
        .collect::<String>()
        .parse::<u64>()
        .expect("distance is a number");
    let tf = t as f64;
    let df = d as f64;
    let min = ((tf - (tf.powf(2.0) - (4.0 * df)).sqrt()) / 2.0).ceil() as u64;
    let max = ((tf + (tf.powf(2.0) - (4.0 * df)).sqrt()) / 2.0).floor() as u64;
    let min = if min * (t - min) == d { min + 1 } else { min };
    let max = if max * (t - max) == d { max - 1 } else { max };
    1 + max - min
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let input = "Time:      7  15   30
Distance:  9  40  200";

        let answer = solve(input);

        assert_eq!(answer, 71503);
    }
}
