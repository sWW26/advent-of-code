use itertools::Itertools;

fn main() {
    let input = include_str!("input2.txt");

    let answer = solve(input);

    println!("{}", answer);
}

fn get_prev_element(nums: &[i32]) -> i32 {
    if nums[0] == 0 && nums.iter().all_equal() {
        0
    } else {
        let seq = nums.iter().tuple_windows().map(|(a, b)| b - a).collect::<Vec<_>>();
        nums.first().expect("should always have at least one element") - get_prev_element(&seq)
    }
}

fn solve(str: &str) -> i32 {
    str.lines()
        .map(|line| {
            let numbers =
                line.split(' ')
                    .map(|x| x.parse::<i32>().expect("all entries should be numbers"))
                    .collect::<Vec<_>>();
            get_prev_element(&numbers)
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_one() {
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";

        let answer = solve(input);

        assert_eq!(answer, 2);
    }
}
