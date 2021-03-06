//! Find a secret code from criteria

#![deny(missing_docs, unsafe_code, unused_extern_crates, warnings)]

use std::default::Default;
use std::fmt;

/// Criteria information
#[derive(Debug)]
pub enum Data {
    /// None of numbers is in the final code
    None,
    /// Multiple number is in the code
    Good(Position, u8)
}

/// Criteria numbers, where are they ?
#[derive(Debug)]
pub enum Position {
    /// It's at the right place !
    Good,
    /// Not in the right digit
    Bad
}

/// Every Values available
#[derive(Debug)]
pub struct Values(Vec<Value>);

/// A possible code
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Value(u8, u8, u8);

impl Default for Values {
    fn default () -> Self {
        let mut values: Vec<Value> = Vec::with_capacity(10 * 10 * 10);

        for x in 0..10 {
            for y in 0..10 {
                for z in 0..10 {
                    values.push(Value(x, y, z))
                }
            }
        }

        Values(values)
    }
}

impl Value {
    /// Find if your code contain a number
    pub fn has_some (&self, number: u8) -> bool {
        self.0 == number || self.1 == number || self.2 == number
    }

    /// Find if your code contain a number at same position or in other
    pub fn occurences(&self, position: &Position, numbers: Value) -> u8 {
        let &Value(a, b, c) = self;
        let Value(x, y, z) = numbers;

        match position {
            &Position::Good => {
                vec![a == x, b == y, c == z].iter().filter(|&b| *b).count() as u8
            },
            &Position::Bad => {
                vec![a == y || a == z, b == x || b == z || c == x || c == y].iter()
                    .filter(|&b| *b).count() as u8
            }
        }
    }
}

impl Values {
    /// Apply criteria to your values
    pub fn apply (self, numbers: Value, data: Data) -> Self {
        let Value(x, y, z) = numbers;
        let Values(values) = self;

        match data {
            Data::None => {
                let values: Vec<Value> = values.into_iter()
                    .filter(|&value| !(value.has_some(x) || value.has_some(y) || value.has_some(z)))
                    .collect();

                Values(values)
            },
            Data::Good(position, number) => {
                let values = values.into_iter()
                    .filter(|&value| value.occurences(&position, numbers) == number)
                    .collect();

                Values(values)
            }
        }
    }
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}{}", self.0, self.1, self.2)
    }
}

impl fmt::Display for Values {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "The following values correspond to your criteria: {}",
               self.0.iter()
               .fold(String::new(), |acc, &value| format!("{}{} ", acc, value)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let values = Values::default()
            .apply(Value(9,4,2), Data::None)
            .apply(Value(0,8,4), Data::Good(Position::Good, 1))
            .apply(Value(4,7,9), Data::Good(Position::Bad, 1))
            .apply(Value(8,3,7), Data::Good(Position::Bad, 2))
            .apply(Value(2,6,0), Data::Good(Position::Bad, 1));

        println!("{}", values);
        assert!(values.0[0] == Value(7, 8, 6))
    }
}
