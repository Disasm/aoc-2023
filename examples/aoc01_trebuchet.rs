use aoc_2023::{solve, Assignment};

struct Task;

impl Assignment for Task {
    type Input = Vec<String>;
    type R1 = u32;
    type R2 = u32;

    fn parse_input(string: &str) -> Self::Input {
        string.lines().map(|s| s.to_string()).collect()
    }

    fn solve_task1(input: &Self::Input) -> Self::R1 {
        input
            .iter()
            .map(|line| {
                let p1 = line.find(|c: char| c.is_ascii_digit()).unwrap();
                let p2 = line.rfind(|c: char| c.is_ascii_digit()).unwrap();
                let d1 = line.as_bytes()[p1] - b'0';
                let d2 = line.as_bytes()[p2] - b'0';
                u32::from(d1) * 10 + u32::from(d2)
            })
            .sum()
    }

    fn solve_task2(input: &Self::Input) -> Self::R2 {
        input
            .iter()
            .map(|line| {
                let (_, d1) = MAP
                    .into_iter()
                    .filter_map(|(s, d)| line.find(s).map(|pos| (pos, d)))
                    .min()
                    .unwrap();
                let (_, d2) = MAP
                    .into_iter()
                    .filter_map(|(s, d)| line.rfind(s).map(|pos| (pos, d)))
                    .max()
                    .unwrap();
                d1 * 10 + d2
            })
            .sum()
    }
}

const MAP: [(&str, u32); 19] = [
    ("0", 0),
    ("1", 1),
    ("2", 2),
    ("3", 3),
    ("4", 4),
    ("5", 5),
    ("6", 6),
    ("7", 7),
    ("8", 8),
    ("9", 9),
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

fn main() {
    const INPUT: &str = include_str!("../input/01");
    solve::<Task>(INPUT);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../input/01e");
        let input = Task::parse_input(input);
        assert_eq!(Task::solve_task1(&input), 142);

        let input = include_str!("../input/01");
        let input = Task::parse_input(input);
        assert_eq!(Task::solve_task1(&input), 53974);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../input/01e2");
        let input = Task::parse_input(input);
        assert_eq!(Task::solve_task2(&input), 281);

        let input = include_str!("../input/01");
        let input = Task::parse_input(input);
        assert_eq!(Task::solve_task2(&input), 52840);
    }
}
