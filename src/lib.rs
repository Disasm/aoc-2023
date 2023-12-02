use std::fmt::Display;

/// https://adventofcode.com/

pub trait Assignment {
    type Input;
    type R1: Display;
    type R2: Display;

    fn parse_input(string: &str) -> Self::Input;

    fn solve_task1(input: &Self::Input) -> Self::R1;

    fn solve_task2(input: &Self::Input) -> Self::R2;
}

pub fn solve<A: Assignment>(input: &str) {
    let input = A::parse_input(input);

    let r1 = A::solve_task1(&input);
    println!("Assignment1 result: {}", r1);

    let r2 = A::solve_task2(&input);
    println!("Assignment1 result: {}", r2);
}
