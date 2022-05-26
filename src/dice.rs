use nom::{
    bytes::complete::tag,
    character::complete::{digit1, space0},
    combinator::{eof, map_res},
    IResult,
};
use rand::Rng;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
enum Piece {
    Die { count: u64, sides: u64 },
    Offset(u64),
}

#[derive(Debug, PartialEq)]
struct Dice {
    pieces: Vec<Piece>,
}

// Parses "NdM" where N, M are numbers.
fn single_die(input: &str) -> IResult<&str, Piece> {
    let (input, count) = map_res(digit1, u64::from_str)(input)?;
    let (input, _) = tag("d")(input)?;
    let (input, sides) = map_res(digit1, u64::from_str)(input)?;
    Ok((input, Piece::Die { count, sides }))
}

// Parses "N" where N is a number.
fn single_offset(input: &str) -> IResult<&str, Piece> {
    let (input, offset) = map_res(digit1, u64::from_str)(input)?;
    Ok((input, Piece::Offset(offset)))
}

fn single_piece(input: &str) -> IResult<&str, Piece> {
    if let Ok((input, dice)) = single_die(input) {
        return Ok((input, dice));
    }
    single_offset(input)
}

fn plus_single_piece(input: &str) -> IResult<&str, Piece> {
    let (input, _) = space0(input)?;
    let (input, _) = tag("+")(input)?;
    let (input, _) = space0(input)?;
    single_piece(input)
}

// Parses a full dice specification, which is a sum of dice and offsets.
fn dice_specification(input: &str) -> Result<Dice, nom::Err<nom::error::Error<&str>>> {
    let (input, _) = space0(input)?;
    let (input, first_piece) = single_piece(input)?;
    let (input, mut rest) = nom::multi::many0(plus_single_piece)(input)?;
    eof(input)?;
    let mut pieces = vec![first_piece];
    pieces.extend(rest.drain(0..));
    Ok(Dice { pieces })
}

impl Piece {
    fn validate(&self) -> Result<(), String> {
        if let Piece::Die { count: _, sides } = *self {
            if sides < 1 {
                return Err(format!("too few sides: {}", sides));
            }
        }
        Ok(())
    }
    fn roll(&self) -> u64 {
        let (count, sides) = match *self {
            Piece::Offset(offset) => return offset,
            Piece::Die { count, sides } => (count, sides),
        };
        let mut rng = rand::rngs::OsRng;
        let mut sum = 0;
        for _ in 0..count {
            sum += rng.gen_range(1..sides + 1);
        }
        sum
    }
}

impl Dice {
    fn validate(&self) -> Result<(), String> {
        self.pieces.iter().map(|p| p.validate()).collect()
    }
    fn roll(&self) -> u64 {
        let mut sum = 0;
        for piece in self.pieces.iter() {
            sum += piece.roll()
        }
        sum
    }
}

pub fn roll(spec: &str) -> Result<u64, String> {
    let dice = match dice_specification(&spec) {
        Ok(dice) => dice,
        Err(e) => return Err(format!("failed to parse '{}': {}", spec, e)),
    };
    dice.validate()?;
    Ok(dice.roll())
}

#[cfg(test)]
mod tests {
    use super::{dice_specification, roll, single_die, Dice, Piece};
    #[test]
    fn roll_dice() {
        assert!(roll("1d6").is_ok());
        assert!(roll("4d20+10").is_ok());
        assert!(roll("0d6+4d5+10+2").is_ok());
        assert!(roll("6d8 + 2 + 5d6").is_ok());
        assert!(roll("6d8 + 2 2 + 5d6").is_err());
    }
    #[test]
    fn die_specification_parsing() {
        assert_eq!(
            single_die("2d8"),
            Ok(("", Piece::Die { count: 2, sides: 8 }))
        );
        assert_eq!(
            single_die("16d20+15"),
            Ok((
                "+15",
                Piece::Die {
                    count: 16,
                    sides: 20,
                }
            ))
        );
    }
    #[test]
    fn die_specification_with_offset_parsing() {
        assert_eq!(
            dice_specification("2d8+1"),
            Ok(Dice {
                pieces: vec![Piece::Die { count: 2, sides: 8 }, Piece::Offset(1)]
            })
        );
        assert_eq!(
            dice_specification("2d8  +  1"),
            Ok(Dice {
                pieces: vec![Piece::Die { count: 2, sides: 8 }, Piece::Offset(1)]
            })
        );
    }
}
