use std::vec;

extern crate nom;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, anychar, digit1, line_ending},
    combinator::map_res,
    multi::{many1, separated_list1},
    sequence::{delimited, terminated, tuple},
    IResult,
};

#[derive(Debug, PartialEq, Eq)]
pub struct PuzzleInput {
    pub stacks: Vec<Vec<char>>,
    pub moves: Vec<Move>,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Move {
    pub amount: usize,
    pub from_index: usize,
    pub to_index: usize,
}

impl PuzzleInput {
    pub fn parse(input: &str) -> IResult<&str, PuzzleInput> {
        let (input, stacks) = stacks(input)?;
        let (input, _indices) = indices(input)?;
        let (input, _empty) = empty_line(input)?;
        let (input, moves) = moves(input)?;

        assert!(input.is_empty());

        Ok((input, PuzzleInput { stacks, moves }))
    }
}

fn stacks(input: &str) -> IResult<&str, Vec<Vec<char>>> {
    let (input, stack_lines) = many1(stack)(input)?;

    let mut stacks = transpose(stack_lines);
    for stack in &mut stacks {
        stack.retain_mut(|e| *e != ' '); // remove empty entries
        stack.reverse()
    }

    Ok((input, stacks))
}

fn stack(input: &str) -> IResult<&str, Vec<char>> {
    terminated(
        separated_list1(complete::char(' '), stack_entry),
        line_ending,
    )(input)
}

fn stack_entry(input: &str) -> IResult<&str, char> {
    alt((emtpy_entry, occupied_entry))(input)
}

fn emtpy_entry(input: &str) -> IResult<&str, char> {
    let (input, _) = tag("   ")(input)?;
    Ok((input, ' '))
}

fn occupied_entry(input: &str) -> IResult<&str, char> {
    delimited(complete::char('['), anychar, complete::char(']'))(input)
}

fn indices(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, indices) = terminated(
        separated_list1(
            complete::char(' '),
            delimited(
                complete::char(' '),
                map_res(digit1, str::parse::<u32>),
                complete::char(' '),
            ),
        ),
        line_ending,
    )(input)?;

    Ok((input, indices))
}

fn empty_line(input: &str) -> IResult<&str, ()> {
    let (input, _) = line_ending(input)?;
    Ok((input, ()))
}

fn moves(input: &str) -> IResult<&str, Vec<Move>> {
    separated_list1(line_ending, a_move)(input)
}

fn a_move(input: &str) -> IResult<&str, Move> {
    let (input, (_, amount, _, from_index, _, to_index)) = tuple((
        tag("move "),
        map_res(digit1, str::parse::<usize>),
        tag(" from "),
        map_res(digit1, str::parse::<usize>),
        tag(" to "),
        map_res(digit1, str::parse::<usize>),
    ))(input)?;

    Ok((
        input,
        Move {
            amount,
            from_index: from_index - 1,
            to_index: to_index - 1,
        },
    ))
}

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    if v.is_empty() {
        return vec![];
    }
    let len = v[0].len();
    let mut iters: Vec<_> = v.into_iter().map(|n| n.into_iter()).collect();
    (0..len)
        .map(|_| {
            iters
                .iter_mut()
                .map(|n| n.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing() {
        assert_eq!(emtpy_entry("   "), Ok(("", ' ')));
        assert_eq!(occupied_entry("[A]"), Ok(("", 'A')));
        assert_eq!(stack_entry("   "), Ok(("", ' ')));
        assert_eq!(stack_entry("[A]"), Ok(("", 'A')));
        assert_eq!(
            stack("    [A]     [X]\n"),
            Ok(("", vec![' ', 'A', ' ', 'X']))
        );
        assert_eq!(
            stacks("    [A]     [X]\n    [A]     [X]\n"),
            Ok(("", vec![vec![], vec!['A', 'A'], vec![], vec!['X', 'X']]))
        );
        assert_eq!(indices(" 1   2   3 \n"), Ok(("", vec![1, 2, 3])));
        assert_eq!(empty_line("\n"), Ok(("", ())));
        assert_eq!(
            a_move("move 1 from 2 to 3X"),
            Ok((
                "X",
                Move {
                    amount: 1,
                    from_index: 1,
                    to_index: 2
                }
            ))
        );
    }
}
