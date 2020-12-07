use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug)]
pub struct Policy {
    pub min: u8,
    pub max: u8,
    pub letter: u8,
    pub pass: [u8; 24],
}

#[aoc_generator(day2)]
pub fn generator(input: &str) -> Vec<Policy> {
    fn make_number(chars: &mut dyn Iterator<Item = &u8>, sigil: u8) -> u8 {
        let mut value = 0u8;
        while let Some(c) = chars.next() {
            if c.is_ascii_digit() {
                value = (value * 10) + (c - 48);
            } else if *c == sigil {
                break;
            }
        }

        value
    }

    input
        .split("\n")
        .map(|line| {
            let mut chars = line.as_bytes().into_iter();

            let min = make_number(&mut chars, b'-');
            let max = make_number(&mut chars, b' ');
            let letter = chars.next().unwrap();
            
            chars.next();
            chars.next();

            let mut pass = [0u8; 24];
            let mut i = 0;
            while let Some(c) = chars.next() {
                pass[i] = *c;
                i += 1;
            }

            Policy {
                min,
                max,
                letter: *letter,
                pass,
            }
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn part1(input: &Vec<Policy>) -> usize {
    input
        .iter()
        .filter(|policy| {
            let count = policy.pass.iter().filter(|c| **c == policy.letter).count();

            count >= policy.min.into() && count <= policy.max.into()
        })
        .count()
}

#[aoc(day2, part2)]
pub fn part2(input: &Vec<Policy>) -> usize {
    input
        .iter()
        .filter(|policy| {
            let first = policy.pass[policy.min as usize - 1].eq(&policy.letter);
            let last = policy.pass[policy.max as usize - 1].eq(&policy.letter);

            first ^ last
        })
        .count()
}
