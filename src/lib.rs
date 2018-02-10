use std::default::Default;
use std::fmt;

pub enum Data {
    None,
    Good(Position, u8)
}

pub enum Position {
    Good,
    Bad
}

#[derive(Debug)]
pub struct Values(Vec<Value>);
#[derive(Debug, Copy, Clone)]
pub struct Value(u8, u8, u8);

impl Default for Values {
    fn default () -> Self {
        let mut values: Vec<Value> = Vec::new();

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

impl Values {
    pub fn apply (self, numbers: Value, data: Data) -> Self {
        let Value(x, y, z) = numbers;
        let Values(values) = self;

        match data {
            Data::None => {
                let values: Vec<Value> = values.into_iter()
                    .filter(|&Value(a, b, c)| !(a == x || a == y || a == z
                                                || b == x || b == y || b == z
                                                || c == x || c == y || c == z))
                    .collect();

                Values(values)
            },
            Data::Good(Position::Good, number) => {
                let values = values.into_iter()
                    .filter(|&Value(a, b, c)| {
                        let mut occurence = 0u8;

                        if a == x {
                            occurence += 1
                        }

                        if b == y {
                            occurence += 1
                        }

                        if c == z {
                            occurence += 1
                        }

                        occurence == number
                    })
                .collect();

                Values(values)
            },
            Data::Good(Position::Bad, number) => {
                let values = values.into_iter()
                    .filter(|&Value(a, b, c)| {
                        let mut occurence = 0u8;

                        if a == y || a == z {
                            occurence += 1
                        }

                        if b == x || b == z {
                            occurence += 1
                        }

                        if c == x || c == y {
                            occurence += 1
                        }

                        occurence == number
                    })
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
        write!(f, "The following values correspond to your criteria:\n{}", 
               self.0.iter()
               .fold(String::new(), |acc, &value| format!("{}{} ", acc, value)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fmt::Display;

    #[test]
    fn it_works() {
        let values = Values::default()
            .apply(Value(9,4,2), Data::None)
            .apply(Value(0,8,4), Data::Good(Position::Good, 1))
            .apply(Value(4,7,9), Data::Good(Position::Bad, 1))
            .apply(Value(8,3,7), Data::Good(Position::Bad, 2))
            .apply(Value(2,6,0), Data::Good(Position::Bad, 1));

        println!("{}", values);
    }
}
