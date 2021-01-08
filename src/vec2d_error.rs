use std::fmt::Display;

#[derive(Debug)]
pub enum Vec2dError {
    WidthOrHeightIs0 { width: usize, height: usize },
    WidthOrInputLenIs0 { width: usize, input_len: usize },
    InputNotDivisibleByWidth { width: usize, input_len: usize },
}

impl Display for Vec2dError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Vec2dError::WidthOrHeightIs0 { width, height } => {
                if *width == 0 {
                    write!(f, "Width should be non-zero. Width: {}, height: {}", width, height)
                } else {
                    write!(f, "Height should be non-zero. Width: {}, height: {}", width, height)
                }
            },
            Vec2dError::WidthOrInputLenIs0 { width, input_len } => {
                if *width == 0 {
                    write!(f, "Width should be non-zero. Width: {}, input_len: {}", width, input_len)
                } else {
                    write!(f, "input_len should be non-zero. Width: {}, height: {}", width, input_len)
                }
            },
            Vec2dError::InputNotDivisibleByWidth { width, input_len } => {
                write!(f, "The input_len is not divisible by the width: {} % {} = {}", input_len, width, input_len % width)
            }
        }
    }
}