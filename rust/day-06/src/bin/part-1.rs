fn main() {
    let input = include_str!("input1.txt");

    let answer = solve(input);

    println!("{}", answer);
}

fn solve(str: &str) -> u32 {
    let mut lines = str.lines();
    let times = lines
        .next()
        .expect("has at least one line")
        .split_once(':')
        .expect("starts 'Time:'")
        .1
        .split_ascii_whitespace()
        .map(|s| s.parse::<u32>().expect("times are numbers"));
    let distances = lines
        .next()
        .expect("has at least two lines")
        .split_once(':')
        .expect("starts 'Distance'")
        .1
        .split_ascii_whitespace()
        .map(|s| s.parse::<u32>().expect("distances are numbers"));
    times
        .zip(distances)
        .map(|(t, d)| {
            let tf = t as f32;
            let df = d as f32;
            let min = ((tf - (tf.powf(2.0) - (4.0 * df)).sqrt()) / 2.0).ceil() as u32;
            let max = ((tf + (tf.powf(2.0) - (4.0 * df)).sqrt()) / 2.0).floor() as u32;
            let min = if min * (t - min) == d { min + 1 } else { min };
            let max = if max * (t - max) == d { max - 1 } else { max };
            1 + max - min
        })
        .product()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let input = "Time:      7  15   30
Distance:  9  40  200";

        let answer = solve(input);

        assert_eq!(answer, 288);
    }
}
