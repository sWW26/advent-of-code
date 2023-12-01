fn main() {
    let input = include_str!("input.txt");
    let answer = solve(input);
    println!("{}", answer);
}

fn solve(str: &str) -> i32 {
    str.split('\n')
        .map(|line| {
            let first = line
                .chars()
                .find_map(|x| x.to_digit(10))
                .expect("always at least two numbers");
            let last = line
                .chars()
                .rev()
                .find_map(|x| x.to_digit(10))
                .expect("always at least two numbers");
            (first * 10 + last) as i32
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

        let res = solve(&input);

        assert_eq!(res, 142);
    }
}
