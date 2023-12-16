use itertools::Itertools;

fn main() {
    let input = include_str!("input1.txt");

    let answer = solve(input);

    println!("{}", answer);
}

fn get_next_element(nums: &[i32]) -> i32 {
    if nums[0] == 0 && nums.iter().all_equal() {
        0
    } else {
        let seq = nums.iter().tuple_windows().map(|(a, b)| b - a).collect::<Vec<_>>();
        nums.last().expect("should always have at least one element") + get_next_element(&seq)
    }
}

fn solve(str: &str) -> i32 {
    str.lines()
        .map(|line| {
            let numbers =
                line.split(' ')
                    .map(|x| x.parse::<i32>().expect("all entries should be numbers"))
                    .collect::<Vec<_>>();
            get_next_element(&numbers)
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

        assert_eq!(answer, 114);
    }
}
