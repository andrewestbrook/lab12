# The eight neighbor offsets

The middle square is the cell at `(row, col)`. Each square shows the offset `(dr, dc)` to
add to reach that neighbor: `dr` is the change in row and `dc` is the change in column.
Rows grow downward and columns grow to the right.

```
              dc = -1        dc = 0         dc = +1
           +-------------+-------------+-------------+
 dr = -1   |  (-1, -1)   |  (-1,  0)   |  (-1, +1)   |
           |  up-left    |  up         |  up-right   |
           +-------------+-------------+-------------+
 dr =  0   |  ( 0, -1)   |  \       /  |  ( 0, +1)   |
           |  left       |    X (0,0)  |  right      |
           |             |  /       \  |             |
           +-------------+-------------+-------------+
 dr = +1   |  (+1, -1)   |  (+1,  0)   |  (+1, +1)   |
           |  down-left  |  down       |  down-right |
           +-------------+-------------+-------------+
```

The center square `(0, 0)` is **crossed out**: it is the cell itself, not a neighbor.

## The eight neighbors

| Neighbor | dr | dc |
|----------|----|----|
| up-left | -1 | -1 |
| up | -1 | 0 |
| up-right | -1 | +1 |
| left | 0 | -1 |
| right | 0 | +1 |
| down-left | +1 | -1 |
| down | +1 | 0 |
| down-right | +1 | +1 |

`count_neighbors` makes these eight offsets with two loops, `dr` in -1..=1 and `dc` in
-1..=1, and skips the one pair where both are 0.

Because `row` and `col` are `usize`, the neighbor position is computed as
`row as isize + dr` and `col as isize + dc`. A result below 0 or at/after `ROWS`/`COLS`
is off the grid and counts as dead.
