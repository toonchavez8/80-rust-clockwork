//! Parses one non-negative integer from the command line and creates
//! a square grid large enough to hold the values from 0 through the
//! provided number.

use std::env;

fn main() {
    // Collect all command-line arguments into a vector.
    // The first argument (index 0) is always the executable name.
    let args: Vec<String> = env::args().collect();

    // Expect exactly one user-provided argument.
    // Example:
    //   cargo run -- 5
    if args.len() != 2 {
        eprintln!("Usage: cargo run -- <non-negative int>");
        return;
    }

    // Borrow the user-provided argument.
    let raw_n = &args[1];

    // Try to parse the argument as an unsigned integer (`usize`).
    // This automatically rejects:
    // - negative numbers
    // - non-numeric input
    let Ok(n) = raw_n.parse::<usize>() else {
        eprintln!("Please provide a valid non-negative integer.");
        return;
    };

    // Determine how many values need to be stored.
    // Since the range is inclusive (0..=n), the total count is `n + 1`.
    let value_count = n + 1;

    // Compute the side length of the smallest square that can hold
    // every value. For example:
    // - 9 values -> 3 × 3
    // - 10 values -> 4 × 4
    let size = (value_count as f64).sqrt().ceil() as usize;

    // Create a square grid initialized with `None` in every cell.
    // Each cell will eventually store either:
    // - `Some(value)` when a number is placed there, or
    // - `None` if the cell is still empty.
    let grid: Vec<Vec<Option<usize>>> = vec![vec![None; size]; size];

    // Display the computed dimensions and the initial empty grid.
    println!("grid size = {size} x {size}");
    println!("{grid:?}");
}
