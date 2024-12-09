#![feature(allocator_api)]
#![feature(impl_trait_in_assoc_type)]
#![feature(debug_closure_helpers)]
#![warn(clippy::all)]
//#![warn(clippy::pedantic)]
pub mod collections;
pub mod graph;
pub mod math;
pub mod parse;
pub mod traits;
pub mod utils;

use ndarray::{Array2, Axis};

pub fn print_array(array: &Array2<usize>) {
    for row in array.axis_iter(Axis(0)) {
        for c in row {
            print!("{c}");
        }
        println!();
    }
}

#[must_use]
pub fn trim_ascii_whitespace(x: &[u8]) -> &[u8] {
    let from = match x.iter().position(|x| !x.is_ascii_whitespace()) {
        Some(i) => i,
        None => return &x[0..0],
    };
    let to = x.iter().rposition(|x| !x.is_ascii_whitespace()).unwrap();
    &x[from..=to]
}
