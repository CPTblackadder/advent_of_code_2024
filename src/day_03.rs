use crate::TaskCompleter;

pub struct Task3;

enum MultiplierConsumer {
    Empty,
    M,
    U,
    L,
    FirstBracket,
    FirstNumbers(u32),
    Comma(u32),
    SecondNumbers(u32, u32),
    Complete(u32, u32),
}

impl MultiplierConsumer {
    pub fn consume_next(&mut self, char: char) {
        *self = match self {
            MultiplierConsumer::Empty | Self::Complete(_, _) => {
                if char == 'm' {
                    Self::M
                } else {
                    Self::Empty
                }
            }
            MultiplierConsumer::M => {
                if char == 'u' {
                    Self::U
                } else {
                    Self::Empty
                }
            }

            MultiplierConsumer::U => {
                if char == 'l' {
                    Self::L
                } else {
                    Self::Empty
                }
            }
            MultiplierConsumer::L => {
                if char == '(' {
                    Self::FirstBracket
                } else {
                    Self::Empty
                }
            }
            MultiplierConsumer::FirstBracket => {
                if char.is_ascii_digit() {
                    Self::FirstNumbers(char.to_digit(10).unwrap())
                } else {
                    Self::Empty
                }
            }
            MultiplierConsumer::FirstNumbers(x) => {
                if char.is_ascii_digit() {
                    Self::FirstNumbers((*x * 10) + char.to_digit(10).unwrap())
                } else if char == ',' {
                    Self::Comma(*x)
                } else {
                    Self::Empty
                }
            }
            MultiplierConsumer::Comma(x) => {
                if char.is_ascii_digit() {
                    Self::SecondNumbers(*x, char.to_digit(10).unwrap())
                } else {
                    Self::Empty
                }
            }
            MultiplierConsumer::SecondNumbers(x, y) => {
                if char.is_ascii_digit() {
                    Self::SecondNumbers(*x, (*y * 10) + char.to_digit(10).unwrap())
                } else if char == ')' {
                    Self::Complete(*x, *y)
                } else {
                    Self::Empty
                }
            }
        }
    }

    pub fn get_value(&self) -> u32 {
        if let Self::Complete(x, y) = self {
            x * y
        } else {
            0
        }
    }
}

enum MultiplierConsumerPart2 {
    Empty,
    M,
    U,
    L,
    FirstBracket,
    FirstNumbers(u32),
    Comma(u32),
    SecondNumbers(u32, u32),
    Complete(u32, u32),
    D0,
    O0,
    N,
    Apostrophe,
    T,
    DontFirstBracket,
    NotActive,
    D1,
    O1,
    DoFirstBracket,
    DoSecondBracket,
}

impl MultiplierConsumerPart2 {
    pub fn consume_next(&mut self, char: char) {
        *self = match self {
            Self::Empty | Self::DoSecondBracket | Self::Complete(_, _) => {
                if char == 'm' {
                    Self::M
                } else if char == 'd' {
                    Self::D0
                } else {
                    Self::Empty
                }
            }
            Self::M => {
                if char == 'u' {
                    Self::U
                } else {
                    Self::Empty
                }
            }

            Self::U => {
                if char == 'l' {
                    Self::L
                } else {
                    Self::Empty
                }
            }
            Self::L => {
                if char == '(' {
                    Self::FirstBracket
                } else {
                    Self::Empty
                }
            }
            Self::FirstBracket => {
                if char.is_ascii_digit() {
                    Self::FirstNumbers(char.to_digit(10).unwrap())
                } else {
                    Self::Empty
                }
            }
            Self::FirstNumbers(x) => {
                if char.is_ascii_digit() {
                    Self::FirstNumbers((*x * 10) + char.to_digit(10).unwrap())
                } else if char == ',' {
                    Self::Comma(*x)
                } else {
                    Self::Empty
                }
            }
            Self::Comma(x) => {
                if char.is_ascii_digit() {
                    Self::SecondNumbers(*x, char.to_digit(10).unwrap())
                } else {
                    Self::Empty
                }
            }
            Self::SecondNumbers(x, y) => {
                if char.is_ascii_digit() {
                    Self::SecondNumbers(*x, (*y * 10) + char.to_digit(10).unwrap())
                } else if char == ')' {
                    Self::Complete(*x, *y)
                } else {
                    Self::Empty
                }
            }
            Self::D0 => {
                if char == 'o' {
                    Self::O0
                } else {
                    Self::Empty
                }
            }
            Self::O0 => {
                if char == 'n' {
                    Self::N
                } else {
                    Self::Empty
                }
            }
            Self::N => {
                if char == '\'' {
                    Self::Apostrophe
                } else {
                    Self::Empty
                }
            }
            Self::Apostrophe => {
                if char == 't' {
                    Self::T
                } else {
                    Self::Empty
                }
            }
            Self::T => {
                if char == '(' {
                    Self::DontFirstBracket
                } else {
                    Self::Empty
                }
            }
            Self::DontFirstBracket => {
                if char == ')' {
                    Self::NotActive
                } else {
                    Self::Empty
                }
            }
            Self::NotActive => {
                if char == 'd' {
                    Self::D1
                } else {
                    Self::NotActive
                }
            }
            Self::D1 => {
                if char == 'o' {
                    Self::O1
                } else {
                    Self::NotActive
                }
            }
            Self::O1 => {
                if char == '(' {
                    Self::DoFirstBracket
                } else {
                    Self::NotActive
                }
            }
            Self::DoFirstBracket => {
                if char == ')' {
                    Self::DoSecondBracket
                } else {
                    Self::NotActive
                }
            }
        }
    }

    pub fn get_value(&self) -> u32 {
        if let Self::Complete(x, y) = self {
            x * y
        } else {
            0
        }
    }
}

impl TaskCompleter for Task3 {
    fn do_task_1(&self) -> String {
        let chars = include_str!("../input/day_03/input").chars();
        let mut consumer = MultiplierConsumer::Empty;
        let mut sum = 0;
        for c in chars {
            consumer.consume_next(c);
            sum += consumer.get_value();
        }
        sum.to_string()
    }

    fn do_task_2(&self) -> String {
        let chars = include_str!("../input/day_03/input").chars();
        let mut consumer = MultiplierConsumerPart2::Empty;
        let mut sum = 0;
        for c in chars {
            consumer.consume_next(c);
            sum += consumer.get_value();
        }
        sum.to_string()
    }

    fn task_1_result(&self) -> Option<String> {
        Some("168539636".to_string())
    }

    fn task_2_result(&self) -> Option<String> {
        Some("97529391".to_string())
    }
}
