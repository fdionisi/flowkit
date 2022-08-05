use std::{
    error::Error,
    fmt, io,
    num::{ParseFloatError, ParseIntError},
};

#[derive(Debug)]
pub enum FlowError {
    Io(io::Error),
    ParseInt(ParseIntError),
    ParseFloat(ParseFloatError),
    InvalidDataType,
}

impl fmt::Display for FlowError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl Error for FlowError {}

impl From<io::Error> for FlowError {
    fn from(error: io::Error) -> FlowError {
        FlowError::Io(error)
    }
}

impl From<ParseFloatError> for FlowError {
    fn from(error: ParseFloatError) -> FlowError {
        FlowError::ParseFloat(error)
    }
}

impl From<ParseIntError> for FlowError {
    fn from(error: ParseIntError) -> FlowError {
        FlowError::ParseInt(error)
    }
}
