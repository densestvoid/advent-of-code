use log::{info, debug};

pub fn solve_part_1(input: &str) {
    let trees = parse_grid(input);
    let visible = visible_trees(&trees);
    info!("visible trees: {}", visible)
}

pub fn solve_part_2(input: &str) {
    let trees = parse_grid(input);
    let max_scenic_score = max_scenic_score(&trees);
    info!("most scenic: {}", max_scenic_score)
}

fn parse_grid(input: &str) -> Vec<Vec<u32>> {
    let mut grid = Vec::new();

    for line in input.lines() {
        let mut row = Vec::new();

        for c in line.chars() {
            match c.to_digit(10) {
                Some(d) => row.push(d),
                None => panic!("unable to parse input"),
            }
        }

        grid.push(row);
    }

    grid
}

fn visible_trees(grid: &Vec<Vec<u32>>) -> u32 {
    let mut count = 0;

    for (i, row) in grid.iter().enumerate() {
        for (j, height) in row.iter().enumerate() {
            if tree_is_visible(height, i, j, grid) {
                count += 1;
            }
        }
    }

    count
}

fn tree_is_visible(height: &u32, x: usize, y: usize, grid: &Vec<Vec<u32>>) -> bool {
    // borders
    let grid_size = grid.len()-1;
    if x%grid_size == 0 || y%grid_size == 0 {
        return true
    }

    // north
    let mut visible_from_north = true;
    for i in (0..=x-1).rev() {
        if grid[i][y] >= *height {
            visible_from_north = false;
            break;
        }
    }
    if visible_from_north {
        return true
    }

    // south
    let mut visible_from_south = true;
    for i in x+1..=grid_size {
        if grid[i][y] >= *height {
            visible_from_south = false;
            break;
        }
    }
    if visible_from_south {
        return true
    }

    // east
    let mut visible_from_east = true;
    for i in (0..=y-1).rev() {
        if grid[x][i] >= *height {
            visible_from_east = false;
            break;
        }
    }
    if visible_from_east {
        return true
    }

    // west
    let mut visible_from_west = true;
    for i in y+1..=grid_size {
        if grid[x][i] >= *height {
            visible_from_west = false;
            break;
        }
    }
    if visible_from_west {
        return true
    }

    false
}

fn max_scenic_score(grid: &Vec<Vec<u32>>) -> u32 {
    let mut max = 0;

    let grid_size = grid.len()-1;

    for (i, row) in grid.iter().enumerate() {
        if i == 0 || i == grid_size {
            continue;
        }

        for (j, height) in row.iter().enumerate() {
            if j == 0 || j == grid_size {
                continue;
            }

            let score = scenic_score(height, i, j, grid);
            if score > max {
                max = score;
            }
        }
    }

    max
}

fn scenic_score(height: &u32, x: usize, y: usize, grid: &Vec<Vec<u32>>) -> u32 {
    let mut score = 1;

    let grid_size = grid.len()-1;

    // north
    let mut visible_from_north = 0;
    for i in (0..=x-1).rev() {
        visible_from_north += 1;
        if grid[i][y] >= *height {
            break;
        }
    }
    debug!("North[{}][{}]: {}", x, y, visible_from_north);
    score *= visible_from_north;

    // south
    let mut visible_from_south = 0;
    for i in x+1..=grid_size {
        visible_from_south += 1;
        if grid[i][y] >= *height {
            break;
        }
    }
    debug!("South[{}][{}]: {}", x, y, visible_from_south);
    score *= visible_from_south;

    // east
    let mut visible_from_east = 0;
    for i in (0..=y-1).rev() {
        visible_from_east += 1;
        if grid[x][i] >= *height {
            break;
        }
    }
    debug!("East[{}][{}]: {}", x, y, visible_from_east);
    score *= visible_from_east;

    // west
    let mut visible_from_west = 0;
    for i in y+1..=grid_size {
        visible_from_west += 1;
        if grid[x][i] >= *height {
            break;
        }
    }
    debug!("West[{}][{}]: {}", x, y, visible_from_west);
    score *= visible_from_west;

    debug!("Score[{}][{}]: {}", x, y, score);
    score
}