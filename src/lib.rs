#![warn(missing_docs)]

//! Welcome to Termgrid.
//! Termgrid is a Rust library allowing for the easy managment of printing a grid to the console.
//! 
//! Termgrid has two primary functions:
//!     Allow for easy printing of a grid to the console
//!     Do so efficiently and by accessing the terminal directly
//! 
//! Using Termgrid makes building fluid & fast console games and simulations a breeze. 

/// Functions and objects for creating and manipulating the grid 
pub mod grid;

pub mod print;
pub mod cursor;