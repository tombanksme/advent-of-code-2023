use nom::bytes::complete::tag;
use nom::character::complete::{char, i32, space0};
use nom::combinator::map;
use nom::multi::separated_list0;
use nom::sequence::tuple;
use nom::IResult;
use std::fs::File;
use std::io;
use std::io::BufRead;
use nom::branch::alt;

fn main() -> io::Result<()> {
    let file = File::open("input.txt")?;
    let result: i32 = io::BufReader::new(file)
        .lines()
        .into_iter()
        .map(|line| {
            let (_, game) = parse_game(&*line.unwrap()).unwrap();

            if let Some(_) = game
                .rounds
                .iter()
                .find(|round| round.red > 12 || round.green > 13 || round.blue > 14)
            {
                0
            } else {
                game.id
            }
        })
        .sum();

    println!("Result: {:?}", result);
    Ok(())
}

#[derive(Debug)]
struct Round {
    red: i32,
    green: i32,
    blue: i32,
}

#[derive(Debug)]
struct Game {
    id: i32,
    rounds: Vec<Round>,
}

fn parse_game(input: &str) -> IResult<&str, Game> {
    let (remaining, (_, id, _, rounds)) = tuple((
        tag("Game "),
        i32,
        char(':'),
        separated_list0(
            char(';'),
            map(tuple((space0, parse_round)), |(_, round)| round),
        ),
    ))(input)?;

    Ok((remaining, Game { id, rounds }))
}

fn parse_round(input: &str) -> IResult<&str, Round> {
    let (remaining, items) = separated_list0(char(','), tuple((space0, parse_color)))(input)?;

    let mut round = Round {
        red: 0,
        green: 0,
        blue: 0,
    };

    for (_, (num, color)) in items {
        match color {
            "red" => round.red = num,
            "green" => round.green = num,
            "blue" => round.blue = num,
            _ => {}
        }
    }

    Ok((remaining, round))
}

fn parse_color(input: &str) -> IResult<&str, (i32, &str)> {
    let (remaining, (num, _, color)) =
        tuple((i32, space0, alt((tag("red"), tag("green"), tag("blue")))))(input)?;

    Ok((remaining, (num, color)))
}
