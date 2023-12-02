use aoc_2023::{solve, Assignment};
use std::cmp::max;
use std::str::FromStr;

struct Task;

impl Assignment for Task {
    type Input = Vec<Game>;
    type R1 = usize;
    type R2 = usize;

    fn parse_input(string: &str) -> Self::Input {
        string.lines().map(|line| line.parse().unwrap()).collect()
    }

    fn solve_task1(input: &Self::Input) -> Self::R1 {
        input
            .iter()
            .filter_map(|game| {
                game.sets
                    .iter()
                    .all(|s| s.is_valid())
                    .then_some(game.number)
            })
            .sum()
    }

    fn solve_task2(input: &Self::Input) -> Self::R2 {
        input
            .iter()
            .map(|game| {
                game.sets
                    .iter()
                    .cloned()
                    .reduce(|s1, s2| s1.max(&s2))
                    .unwrap()
                    .power()
            })
            .sum()
    }
}

struct Game {
    number: usize,
    sets: Vec<CubeSet>,
}

impl FromStr for Game {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(':');

        let game_name = parts.next().unwrap();
        let number = game_name.split(' ').nth(1).unwrap().parse().unwrap();

        let game_sets = parts.next().unwrap();
        let mut sets = Vec::new();
        for game_set in game_sets.trim().split(';') {
            let set = game_set.trim().parse().unwrap();
            sets.push(set);
        }

        Ok(Game { number, sets })
    }
}

#[derive(Clone, Default, PartialEq, Eq)]
struct CubeSet {
    red: usize,
    green: usize,
    blue: usize,
}

impl CubeSet {
    fn is_valid(&self) -> bool {
        self.red <= 12 && self.green <= 13 && self.blue <= 14
    }

    fn max(&self, other: &Self) -> Self {
        Self {
            red: max(self.red, other.red),
            green: max(self.green, other.green),
            blue: max(self.blue, other.blue),
        }
    }

    fn power(&self) -> usize {
        self.red * self.green * self.blue
    }
}

impl FromStr for CubeSet {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut set = CubeSet::default();
        for colors in s.split(',') {
            let mut parts = colors.trim().split(' ');
            let n: usize = parts.next().unwrap().parse().unwrap();
            let color = parts.next().unwrap();
            match color {
                "red" => set.red = n,
                "green" => set.green = n,
                "blue" => set.blue = n,
                _ => panic!("Invalid color {color}"),
            }
        }
        Ok(set)
    }
}

fn main() {
    const INPUT: &str = include_str!("../input/01");
    solve::<Task>(INPUT);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../input/02e");
        let input = Task::parse_input(input);
        assert_eq!(Task::solve_task1(&input), 8);

        let input = include_str!("../input/02");
        let input = Task::parse_input(input);
        assert_eq!(Task::solve_task1(&input), 2331);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../input/02e");
        let input = Task::parse_input(input);
        assert_eq!(Task::solve_task2(&input), 2286);

        let input = include_str!("../input/02");
        let input = Task::parse_input(input);
        assert_eq!(Task::solve_task2(&input), 71585);
    }
}
