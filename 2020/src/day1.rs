use aoc_runner_derive::{aoc_generator, aoc};

#[aoc_generator(day1)]
pub fn generator(input: &str) -> Vec<u32> {
    input.lines().map(|line| line.parse::<u32>().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn part1(input: &Vec<u32>) -> u32 {
    for i in input.iter() {
        for j in input.iter() {
            if i + j == 2020 {
                return i * j
            }
        }
    }

    0
}

#[aoc(day1, part2)]
pub fn part2(input: &Vec<u32>) -> u32 {
    for i in input.iter() {
        for j in input.iter() {
            for k in input.iter() {
                if i + j + k == 2020 {
                    return i * j * k
                }
            }
        }
    }

    0
}
