// Game of Life, Week 12: grid, display, neighbor counts, the rule for one cell.
//
// TODO: remove the next line once every function below is used.
#![allow(dead_code)]

const ROWS: usize = 10;
const COLS: usize = 20;
type Grid = [[bool; COLS]; ROWS];

// All cells dead.
pub fn new_grid() -> Grid {
    [[false; COLS]; ROWS]
}

// '#' for a live cell, '.' for a dead cell, one line per row.
pub fn print_grid(grid: &Grid) {
    for row in grid.iter() {
        let line: String = row
            .iter()
            .map(|&alive| if alive { '#' } else { '.' })
            .collect();
        println!("{}", line);
    }
}

// Live neighbors of (row, col): the eight surrounding cells, excluding the cell itself.
// Cells beyond the edge count as dead. Careful: row - 1 on a usize of 0 panics.
pub fn count_neighbors(grid: &Grid, row: usize, col: usize) -> u8 {
    let mut count: u8 = 0;
    for dr in -1..=1_isize {
        for dc in -1..=1_isize {
            // The cell itself is not its own neighbor.
            if dr == 0 && dc == 0 {
                continue;
            }
            // Work in isize so row - 1 at row 0 gives -1 instead of panicking.
            let r = row as isize + dr;
            let c = col as isize + dc;
            // Off the grid counts as dead: only index when inside the grid.
            if r >= 0 && r < ROWS as isize && c >= 0 && c < COLS as isize {
                if grid[r as usize][c as usize] {
                    count += 1;
                }
            }
        }
    }
    count
}

// The rule for one cell: does it live in the next generation?
pub fn next_state(alive: bool, neighbors: u8) -> bool {
    match (alive, neighbors) {
        (true, 2) | (true, 3) => true, // a live cell with 2 or 3 neighbors survives
        (false, 3) => true,            // a dead cell with exactly 3 neighbors is born
        _ => false,                    // every other cell is dead next generation
    }
}

// Debug view: each cell's neighbor count as a digit, one line per row.
pub fn print_neighbor_counts(_grid: &Grid) {
    todo!("Implement print_neighbor_counts")
}

// Set a few cells alive (hardcoded, or entered as `row col` pairs until a blank line),
// print the grid, then the neighbor counts.
pub fn play() {
    todo!("Implement play")
}

#[cfg(test)]
mod tests {
    use super::*;

    // Supplied by the instructor.
    #[test]
    fn live_1_dies() {
        assert!(!next_state(true, 1));
    }

    #[test]
    fn live_2_survives() {
        assert!(next_state(true, 2));
    }

    #[test]
    fn dead_3_born() {
        assert!(next_state(false, 3));
    }

    // Remaining rows of the truth table (design/truth_table.md), one test per row.
    #[test]
    fn dead_0_stays_dead() {
        assert!(!next_state(false, 0));
    }

    #[test]
    fn dead_1_stays_dead() {
        assert!(!next_state(false, 1));
    }

    #[test]
    fn dead_2_stays_dead() {
        assert!(!next_state(false, 2));
    }

    #[test]
    fn dead_4_stays_dead() {
        assert!(!next_state(false, 4));
    }

    #[test]
    fn dead_5_stays_dead() {
        assert!(!next_state(false, 5));
    }

    #[test]
    fn dead_6_stays_dead() {
        assert!(!next_state(false, 6));
    }

    #[test]
    fn dead_7_stays_dead() {
        assert!(!next_state(false, 7));
    }

    #[test]
    fn dead_8_stays_dead() {
        assert!(!next_state(false, 8));
    }

    #[test]
    fn live_0_dies() {
        assert!(!next_state(true, 0));
    }

    #[test]
    fn live_3_survives() {
        assert!(next_state(true, 3));
    }

    #[test]
    fn live_4_dies() {
        assert!(!next_state(true, 4));
    }

    #[test]
    fn live_5_dies() {
        assert!(!next_state(true, 5));
    }

    #[test]
    fn live_6_dies() {
        assert!(!next_state(true, 6));
    }

    #[test]
    fn live_7_dies() {
        assert!(!next_state(true, 7));
    }

    #[test]
    fn live_8_dies() {
        assert!(!next_state(true, 8));
    }

    // count_neighbors: middle, edges, and corners.
    fn full_grid() -> Grid {
        [[true; COLS]; ROWS]
    }

    #[test]
    fn empty_grid_has_no_neighbors() {
        let grid = new_grid();
        assert_eq!(count_neighbors(&grid, 5, 10), 0);
        assert_eq!(count_neighbors(&grid, 0, 0), 0);
    }

    #[test]
    fn cell_is_not_its_own_neighbor() {
        let mut grid = new_grid();
        grid[5][10] = true;
        assert_eq!(count_neighbors(&grid, 5, 10), 0);
    }

    #[test]
    fn middle_of_full_grid_has_8() {
        assert_eq!(count_neighbors(&full_grid(), 5, 10), 8);
    }

    #[test]
    fn corners_of_full_grid_have_3() {
        let grid = full_grid();
        assert_eq!(count_neighbors(&grid, 0, 0), 3);
        assert_eq!(count_neighbors(&grid, 0, COLS - 1), 3);
        assert_eq!(count_neighbors(&grid, ROWS - 1, 0), 3);
        assert_eq!(count_neighbors(&grid, ROWS - 1, COLS - 1), 3);
    }

    #[test]
    fn edges_of_full_grid_have_5() {
        let grid = full_grid();
        assert_eq!(count_neighbors(&grid, 0, 10), 5);
        assert_eq!(count_neighbors(&grid, ROWS - 1, 10), 5);
        assert_eq!(count_neighbors(&grid, 5, 0), 5);
        assert_eq!(count_neighbors(&grid, 5, COLS - 1), 5);
    }

    #[test]
    fn vertical_line_in_middle() {
        let mut grid = new_grid();
        grid[4][10] = true;
        grid[5][10] = true;
        grid[6][10] = true;
        assert_eq!(count_neighbors(&grid, 5, 10), 2); // center of the line
        assert_eq!(count_neighbors(&grid, 4, 10), 1); // top of the line
        assert_eq!(count_neighbors(&grid, 5, 9), 3); // beside the center
        assert_eq!(count_neighbors(&grid, 3, 10), 1); // just above the line
    }
}
