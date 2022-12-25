pub mod board;

pub use board::{HexagonalBoard, render};

#[doc = include_str!("../README.md")]
#[cfg(doctest)]
pub struct ReadmeDoctests;