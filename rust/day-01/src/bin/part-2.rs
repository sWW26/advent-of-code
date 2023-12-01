fn main() {
    let input = include_str!("input2.txt");
    let answer = solve(input);
    println!("{}", answer);
}

fn find_first_number(str_bytes: &[u8], num_strings: &[Box<str>]) -> i32 {
    let mut num = 0_i32;
    let mut i = 0;
    while i < str_bytes.len() {
        let b = str_bytes[i];
        if b.is_ascii_digit() {
            num = (b - ('0' as u8)) as i32;
            break;
        }

        if let Some(string_num) = &num_strings
            .iter()
            .position(|x| str_bytes[i..].starts_with(x.as_bytes()))
        {
            num = (string_num + 1) as i32;
            break;
        }

        i += 1;
    }
    num
}

fn solve(str: &str) -> i32 {
    let num_strings = vec![
        "one".into(),
        "two".into(),
        "three".into(),
        "four".into(),
        "five".into(),
        "six".into(),
        "seven".into(),
        "eight".into(),
        "nine".into(),
    ];
    let num_strings_rev = &num_strings
        .iter()
        .map(|x: &Box<str>| x.chars().rev().collect::<String>().into_boxed_str())
        .collect::<Vec<_>>();
    str.split('\n')
        .map(|line| {
            let forward_bytes = line.as_bytes();
            let reverse_bytes = {
                let mut x = forward_bytes.to_vec();
                x.reverse();
                x
            };
            let first = find_first_number(forward_bytes, &num_strings);
            let last = find_first_number(&reverse_bytes, &num_strings_rev);
            first * 10 + last
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example() {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

        let res = solve(&input);

        assert_eq!(res, 281);
    }
}
