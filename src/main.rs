//! Parses one non-negative integer from the command line and creates
//! a square grid large enough to hold the values from 0 through the
//! provided number.

use std::env;

fn grid_size(value: usize) -> usize {
    ((value + 1) as f64).sqrt().ceil() as usize
}

fn build_spiral(n: usize) -> Vec<Vec<Option<usize>>> {
    let size = grid_size(n);
    let mut grid: Vec<Vec<Option<usize>>> = vec![vec![None; size]; size];

    let mut top: usize = 0;
    let mut bottom: usize = size - 1;
    let mut left: usize = 0;
    let mut right: usize = size - 1;
    let mut value: usize = 0;

    while value <= n {
        for col in left..=right {
            if value > n {
                break;
            }
            grid[top][col] = Some(value);
            value += 1;
        }
        top += 1;
        for row in top..=bottom {
            if value > n {
                break;
            }
            grid[row][right] = Some(value);
            value += 1;
        }

        if right == 0 {
            break;
        }
        right -= 1;

        for col in (left..=right).rev() {
            if value > n {
                break;
            }

            grid[bottom][col] = Some(value);
            value += 1;
        }

        if bottom == 0 {
            break;
        }
        bottom -= 1;

        for row in (top..=bottom).rev() {
            if value > n {
                break;
            }
            grid[row][left] = Some(value);
            value += 1;
        }
        left += 1;
    }

    grid
}

fn print_grid(grid: &[Vec<Option<usize>>], value: usize) {
    // Create a square grid initialized with `None` in every cell.
    // Each cell will eventually store either:
    // - `Some(value)` when a number is placed there, or
    // - `None` if the cell is still empty.
    let cell_width = value.to_string().len();

    for row in grid {
        for (col_index, cell) in row.iter().enumerate() {
            if col_index > 0 {
                print!(" ")
            }

            match cell {
                Some(number) => print!("{number:>cell_width$}"),
                None => print!("{:>cell_width$}", ""),
            }
        }

        println!();
    }
}

fn parse_value(args: &[String]) -> Result<usize, String> {
    if args.len() != 2 {
        return Err("Usage: cargo run -- <non-negative integer>".to_string());
    }

    args[1]
        .parse::<usize>()
        .map_err(|_| "Please provide a valid non-negative integer".to_string())
}

fn main() {
    // Collect all command-line arguments into a vector.
    // The first argument (index 0) is always the executable name.
    let args: Vec<String> = env::args().collect();

    let value = match parse_value(&args) {
        Ok(value) => value,
        Err(message) => {
            eprint!("{message}");
            return;
        }
    };

    let grid = build_spiral(value);
    print_grid(&grid, value);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn grid_size_returns_one_when_n_is_zero() {
        assert_eq!(grid_size(0), 1);
    }

    #[test]
    fn grid_size_returns_six_when_n_is_thirty() {
        assert_eq!(grid_size(30), 6);
    }

    #[test]
    fn build_spiral_places_values_clockwize_for_five() {
        let grid = build_spiral(5);

        assert_eq!(
            grid,
            vec![
                vec![Some(0), Some(1), Some(2)],
                vec![None, None, Some(3)],
                vec![None, Some(5), Some(4)],
            ]
        )
    }

    #[test]
    fn parse_n_returns_error_when_arument_is_missing() {
        let args = vec!["clockwork_spiral".to_string()];
        let result = parse_value(&args);

        assert!(result.is_err())
    }

    #[test]
    fn parse_n_returns_error_when_argument_is_not_a_number() {
        let args = vec!["clockwork_spiral".to_string(), "hello".to_string()];

        let result = parse_value(&args);

        assert!(result.is_err());
    }
}
