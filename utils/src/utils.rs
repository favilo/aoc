use ndarray::{Array2, Axis};

pub mod file;

pub fn print_array(array: &Array2<usize>) {
    for row in array.axis_iter(Axis(0)) {
        for c in row {
            print!("{c}");
        }
        println!();
    }
}
