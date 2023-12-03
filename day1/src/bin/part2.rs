use std::collections::HashMap;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> i32 {
    let mut sum: i32 = 0;

    let lines = input.lines().map(|line| {
        line.replace("one", "1")
            .replace("two", "2")
            .replace("three", "3")
            .replace("four", "4")
            .replace("five", "5")
            .replace("six", "6")
            .replace("seven", "7")
            .replace("eight", "8")
            .replace("nine", "9")
    });

    for coordinate in lines {
        let numbers: Vec<_> = coordinate
            .chars()
            .filter(|char| char.is_digit(10))
            .collect();

        let last = numbers.last().unwrap();
        let first = numbers.first().unwrap();
        let result = format!("{}{}", first, last);

        dbg!(numbers);

        let to_add: i32 = result.parse().unwrap();

        sum += to_add;
    }

    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let result = part2(include_str!("./test2.txt"));
        assert_eq!(result, 281);
    }
}
