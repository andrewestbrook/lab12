# Truth table for `next_state`

Inputs: whether the cell is alive now (`alive`) and how many of its eight neighbors are
alive (`neighbors`, 0 to 8).

Rows needed: 2 states × 9 neighbor counts = **18 rows**.

Rules used:
- A live cell with 2 or 3 live neighbors stays alive.
- A dead cell with exactly 3 live neighbors becomes alive.
- Every other cell is dead in the next generation.

| # | alive now | neighbors | alive next | Why |
|---|-----------|-----------|------------|-----|
| 1 | false | 0 | false | dead, not exactly 3 |
| 2 | false | 1 | false | dead, not exactly 3 |
| 3 | false | 2 | false | dead, not exactly 3 |
| 4 | false | 3 | **true** | dead with exactly 3 → born |
| 5 | false | 4 | false | dead, not exactly 3 |
| 6 | false | 5 | false | dead, not exactly 3 |
| 7 | false | 6 | false | dead, not exactly 3 |
| 8 | false | 7 | false | dead, not exactly 3 |
| 9 | false | 8 | false | dead, not exactly 3 |
| 10 | true | 0 | false | live with fewer than 2 → dies |
| 11 | true | 1 | false | live with fewer than 2 → dies |
| 12 | true | 2 | **true** | live with 2 → survives |
| 13 | true | 3 | **true** | live with 3 → survives |
| 14 | true | 4 | false | live with more than 3 → dies |
| 15 | true | 5 | false | live with more than 3 → dies |
| 16 | true | 6 | false | live with more than 3 → dies |
| 17 | true | 7 | false | live with more than 3 → dies |
| 18 | true | 8 | false | live with more than 3 → dies |

## Pattern

Only three rows give a live cell in the next generation:

- `(true, 2)`
- `(true, 3)`
- `(false, 3)`

The other 15 rows give a dead cell.
