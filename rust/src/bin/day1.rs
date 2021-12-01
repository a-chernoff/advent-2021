use itertools::Itertools;
use advent_2021::{input_data_bufreader};
use std::io::BufRead;
use std::io;

fn main() {
    println!("part 1: {}", count_increases("1").unwrap());
    println!("part 2: {}", count_window_increases("1").unwrap());
}

fn count_increases(fname: &str) -> io::Result<usize> {
    let input = input_data_bufreader(fname)?;
    Ok(input.lines()
        .filter_map(|line| line.unwrap().parse::<usize>().ok())
        .tuple_windows::<(_, _)>()
        .filter(|(a, b)| a < b)
        .count())
}

fn count_window_increases(fname: &str) -> io::Result<usize> {
    let lines = input_data_bufreader(fname)?.lines();
    let (t0, t1) = lines
        .filter_map(|line| line.unwrap().parse::<usize>().ok())
        .tuple_windows::<(_, _, _)>()
        .map(|(a, b, c)| a + b + c)
        .tee();
    Ok(t0.zip(t1.skip(1)).filter(|(a, b)| a < b).count())
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn test_part1() {
        assert_eq!(count_increases("1-test").unwrap(), 7);
    }

    #[test]
    fn test_part2() {
        assert_eq!(count_window_increases("1-test").unwrap(), 5);
    }
}