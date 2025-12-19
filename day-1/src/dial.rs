use std::str::FromStr;
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub enum Direction {
    Left,
    Right,
}

pub struct Dial {
    dial_location: i64,
}

#[derive(Debug)]
pub struct DialTurn {
    direction: Direction,
    n_clicks: i64,
}

#[derive(Debug)]
pub enum ParseDialTurnError {
    EmptyInput,
    InvalidDirectionChar(char),
    InvalidClickCount(std::num::ParseIntError),
}

impl FromStr for DialTurn {
    type Err = ParseDialTurnError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let dir = s.chars().next().ok_or(ParseDialTurnError::EmptyInput)?;

        let n_clicks: i64 = s[1..]
            .parse::<i64>()
            .map_err(ParseDialTurnError::InvalidClickCount)?;

        let direction = match dir {
            'R' => Direction::Right,
            'L' => Direction::Left,
            c => return Err(ParseDialTurnError::InvalidDirectionChar(c)),
        };

        Ok(DialTurn {
            direction,
            n_clicks,
        })
    }
}

impl Dial {
    pub fn new(dial_start: i64) -> Dial {
        Dial {
            dial_location: dial_start,
        }
    }

    fn turn_right(&mut self, n_clicks: i64) {
        self.dial_location = (self.dial_location + n_clicks).rem_euclid(100)
    }

    fn turn_left(&mut self, n_clicks: i64) {
        self.dial_location = (self.dial_location - n_clicks).rem_euclid(100)
    }

    pub fn turn_dial(&mut self, dt: &DialTurn) {
        match dt.direction {
            Direction::Right => self.turn_right(dt.n_clicks),
            Direction::Left => self.turn_left(dt.n_clicks),
        }
    }
}

pub fn get_real_password(d: &mut Dial, dial_turns: Vec<DialTurn>) -> i64 {
    dial_turns
        .iter()
        .filter(|dt| {
            d.turn_dial(dt);
            d.dial_location == 0
        })
        .count() as i64
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::instructions::get_input_instructions;

    #[test]
    fn test_example_from_site() {
        let mut d = Dial::new(50);

        let directions: Vec<&str> = vec![
            "L68", "L30", "R48", "L5", "R60", "L55", "L1", "L99", "R14", "L82",
        ];

        let t: Vec<DialTurn> = directions
            .iter()
            .map(|v| {
                v.parse::<DialTurn>()
                    .expect("could not parse to dial turn.")
            })
            .collect();

        assert_eq!(3, get_real_password(&mut d, t))
    }

    #[test]
    fn test_parse_example_from_site_as_text() {
        let mut d: Dial = Dial::new(50);
        let instructions: Vec<String> = get_input_instructions("data/test_input.txt");

        let parsed_instructions: Vec<DialTurn> = instructions
            .iter()
            .map(|v| {
                v.parse::<DialTurn>()
                    .expect("could not parse to dial turn.")
            })
            .collect();

        assert_eq!(3, get_real_password(&mut d, parsed_instructions))
    }
}
