fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> i32 {
    let mut sum: i32 = 0;

    let lines = input.split("\n");

    for coordinate in lines {
        let numbers: Vec<_> = coordinate
            .chars()
            .filter(|char| char.is_digit(10))
            .collect();

        let last = numbers.last().unwrap();
        let first = numbers.first().unwrap();
        let result = format!("{}{}", first, last);

        let to_add: i32 = result.parse().unwrap();

        sum += to_add;
    }

    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = part1(include_str!("./test1.txt"));
        assert_eq!(result, 142)
    }
}
