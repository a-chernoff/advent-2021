use nom::sequence::separated_pair;

use nom::{Err as NomErr};
use nom::character::complete::space1;
use nom::error::VerboseError;
use nom::IResult;
use nom::error::context;
use nom::bytes::complete::tag_no_case;
use nom::character::complete::digit1;
use nom::branch::alt;

use std::io::{self, BufRead};
use advent_2021::{input_data_bufreader};

fn main() {
    println!("part 1: {}", position_product_part1("2").unwrap());
    println!("part 2: {}", position_product_part2("2").unwrap());
}

type Res<T, U> = IResult<T, U, VerboseError<T>>;

#[derive(Debug, PartialEq, Eq)]
struct Position {
    horizontal: i32,
    depth: i32,
    aim: i32,
}

#[derive(Debug, PartialEq, Eq)]
struct Command {
    dir: Direction,
    len: i32
}

#[derive(Debug, PartialEq, Eq)]
enum Direction {
    Forward, Down, Up
}

impl From<&str> for Direction {
    fn from(i: &str) -> Self {
        match i.to_lowercase().as_str() {
            "forward" => Direction::Forward,
            "down" => Direction::Down,
            "up" => Direction::Up,
            _ => unimplemented!("not a valid direction")
        }
    }
}

fn direction(input: &str) -> Res<&str, Direction> {
    context(
        "direction",
        alt((tag_no_case("forward"), tag_no_case("down"), tag_no_case("up"))),
    )(input)
    .map(|(next_input, res)| (next_input, res.into()))
}

fn cmd_length(input: &str) -> Res<&str, i32> {
    context(
        "cmd_length",
        digit1
    )(input)
    .and_then(|(next_input, result)| {
        match result.parse::<i32>() {
            Ok(n) => Ok((next_input, n)),
            Err(_) => Err(NomErr::Error(VerboseError { errors: vec![] })),
        }
    })
}

fn command(input: &str) -> Res<&str, Command> {
    context(
        "command",
        separated_pair(
            direction,
            space1,
            cmd_length,
        ),
    )(input)
    .map(|(next_input, (dir, num))| {
        (
            next_input, 
            Command {
                dir: dir,
                len: num
            }
        )
    })
}

fn position_product_part1(fname: &str) -> io::Result<i32> {
    let lines = input_data_bufreader(fname)?.lines();
    
    let final_position = lines.map(|line| {
        let line = line.unwrap();
        let line = line.as_str();
        match command(line) {
            Ok(command) => command.1,
            Err(_) => panic!("invalid command")
        }
    })
    .fold(
        Position {
            horizontal: 0,
            depth: 0,
            aim: 0, // unused for part 1
        },
        |mut a, cmd| {
            match cmd.dir {
                Direction::Forward => a.horizontal += cmd.len,
                Direction::Down => a.depth += cmd.len,
                Direction::Up => a.depth -= cmd.len,
            }
            a
        }
    );

    Ok(final_position.horizontal * final_position.depth)
}

fn position_product_part2(fname: &str) -> io::Result<i32> {
    let lines = input_data_bufreader(fname)?.lines();
    let final_position = lines.map(|line| {
        let line = line.unwrap();
        let line = line.as_str();
        match command(line) {
            Ok(cmd) => cmd.1,
            Err(_) => panic!("invalid command"),
        }
    })
    .fold(
        Position {
            horizontal: 0,
            depth: 0,
            aim: 0,
        },
        |mut a, cmd| {
            match cmd.dir {
                Direction::Forward => {
                    a.horizontal += cmd.len;
                    a.depth += a.aim * cmd.len;
                },
                Direction::Down => a.aim += cmd.len,
                Direction::Up => a.aim -= cmd.len,
            }
            a
        }
    );

    Ok(final_position.horizontal * final_position.depth)
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::{
        error::{ErrorKind, VerboseError, VerboseErrorKind},
        Err as NomErr,
    };

    #[test]
    fn test_day2_part1() {
        assert_eq!(position_product_part1("2-test").unwrap(), 150);
    }

    #[test]
    fn test_day2_part2() {
        assert_eq!(position_product_part2("2-test").unwrap(), 900);
    }

    #[test]
    fn test_direction() {
        assert_eq!(direction("forward"), Ok(("", Direction::Forward)));
        assert_eq!(direction("downasdf"), Ok(("asdf", Direction::Down)));
        assert_eq!(
            direction("garbage"),
            Err(NomErr::Error(VerboseError {
                errors: vec![
                    ("garbage", VerboseErrorKind::Nom(ErrorKind::Tag)),
                    ("garbage", VerboseErrorKind::Nom(ErrorKind::Alt)),
                    ("garbage", VerboseErrorKind::Context("direction")),
                ]
            }))
        );
    }

    #[test]
    fn test_command() {
        assert_eq!(
            command("forward 10"),
            Ok(("", Command {
                dir: Direction::Forward,
                len: 10
            }))
        );
    }
}