# Hand counts for `count_neighbors`

I worked each count out by hand first, by looking at the eight squares around a cell
(see `offsets.md`), counting the live ones, and treating any square beyond the edge as
dead. Then I compared with `print_neighbor_counts` (menu option 6 shows both patterns on
the same grid; they are far enough apart that they do not touch).

Coordinates are array indices `grid[row][col]`, counting from 0.

## 1. Middle: vertical line of three

Live cells: `grid[4][10]`, `grid[5][10]`, `grid[6][10]`.

Grid around the line (rows 3–7, columns 8–12), `#` = live:

```
          col  8  9 10 11 12
row 3          .  .  .  .  .
row 4          .  .  #  .  .
row 5          .  .  #  .  .
row 6          .  .  #  .  .
row 7          .  .  .  .  .
```

Hand-worked neighbor counts:

```
          col  8  9 10 11 12
row 3          0  1  1  1  0
row 4          0  2  1  2  0
row 5          0  3  2  3  0
row 6          0  2  1  2  0
row 7          0  1  1  1  0
```

How I got the key ones:
- `(5,10)` center of the line: up `(4,10)` and down `(6,10)` are live → **2**. It does not count itself.
- `(4,10)` top of the line: only down `(5,10)` is live → **1**. Same for `(6,10)`.
- `(5,9)` beside the center: up-right `(4,10)`, right `(5,10)`, down-right `(6,10)` → **3**. Same for `(5,11)`.
- `(4,9)` beside the top: right `(4,10)` and down-right `(5,10)` → **2**. Same for `(4,11)`, `(6,9)`, `(6,11)`.
- `(3,9)`, `(3,10)`, `(3,11)` above the line: each touches only `(4,10)` → **1**. Same below at row 7.
- Columns 8 and 12 are two columns away from the line → **0**.

Program output (`print_neighbor_counts`, rows 3–7, columns 8–12):

```
row 3   01110
row 4   02120
row 5   03230
row 6   02120
row 7   01110
```

**Agreed: yes.** Every cell matches the hand count.

## 2. Corner: vertical line of three down the left edge

Live cells: `grid[0][0]`, `grid[1][0]`, `grid[2][0]` (starts at the top-left corner).

Grid in the corner (rows 0–4, columns 0–2):

```
          col  0  1  2
row 0          #  .  .
row 1          #  .  .
row 2          #  .  .
row 3          .  .  .
row 4          .  .  .
```

Hand-worked neighbor counts:

```
          col  0  1  2
row 0          1  2  0
row 1          2  3  0
row 2          1  2  0
row 3          1  1  0
row 4          0  0  0
```

How I got them (anything at row -1 or column -1 is off the grid and counts as dead):
- `(0,0)` the corner: only 3 of its 8 neighbor squares are on the grid: `(0,1)`, `(1,0)`, `(1,1)`. Only `(1,0)` is live → **1**.
- `(1,0)` middle of the line: up `(0,0)` and down `(2,0)` are live; the three squares to its left are off the grid → **2**.
- `(2,0)` bottom of the line: only up `(1,0)` is live → **1**.
- `(0,1)`: left `(0,0)` and down-left `(1,0)` → **2**.
- `(1,1)`: up-left `(0,0)`, left `(1,0)`, down-left `(2,0)` → **3**.
- `(2,1)`: up-left `(1,0)` and left `(2,0)` → **2**.
- `(3,0)` and `(3,1)`: each touches only `(2,0)` → **1**.
- Column 2 and row 4 are two squares away from every live cell → **0**.

Program output (`print_neighbor_counts`, rows 0–4, columns 0–2):

```
row 0   120
row 1   230
row 2   120
row 3   110
row 4   000
```

**Agreed: yes.** Every cell matches, and the program did not panic at row 0 or column 0,
so the edge check works.

## Full program output for reference

```
12000000000000000000
23000000000000000000
12000000000000000000
11000000011100000000
00000000021200000000
00000000032300000000
00000000021200000000
00000000011100000000
00000000000000000000
00000000000000000000
```
