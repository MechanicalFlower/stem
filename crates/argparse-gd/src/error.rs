// use std::fmt;

// #[derive(Debug)]
// pub enum ArgumentError {
//   Invalid(String),
//   MissingPositional(String),
//   MissingValue(String),
// }

// impl std::error::Error for ArgumentError {}

// impl fmt::Display for ArgumentError {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         match self {
//             ArgumentError::Invalid(msg) => write!(f, "Invalid argument: '{}'.", msg),
//             ArgumentError::MissingPositional(msg) => write!(f, "Missing required argument: '{}'.", msg),
//             ArgumentError::MissingValue(msg) => write!(f, "Missing value for argument: '{}'.", msg),
//         }
//     }
// }
