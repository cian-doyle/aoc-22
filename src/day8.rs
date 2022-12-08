use std::{fs};

const PUZZLE_INPUT: &str = "data/forest.txt";

type Tree = usize;
type Forest = Vec<Vec<Tree>>;

#[derive(Default, Debug)] // Defaults = false for boolean, 0 for usize
struct GridInfo<T: Copy> { 
    top: T,
    bottom: T,
    left: T,
    right: T
}

impl<T: Copy> GridInfo<T> {
    pub fn to_vec(&self) -> Vec<T> {
        vec![self.top, self.bottom, self.right, self.left]
    }
}

fn parse_forest(data: &str) -> Forest { // Builds 2d matrix from input
    data
        .split('\n')
        .map(|row| {
            row
                .chars()
                .map(|c|
                    c.to_digit(10).unwrap() as Tree
                ).collect::<Vec<Tree>>()
        }).collect::<Forest>()
}

fn visible_tree_count(forest: &Forest) -> usize {
    let row_size = forest[1].len();
    let column_size = forest.len();
    // Assuming there are no missing tree along the edges, minimum count is sum of edge lengths (-4 for shared corners)
    let mut visible_count = (row_size * 2) + (column_size * 2) - 4;

    // Visibility for interior trees
    for row in 1..row_size-1 {
        for column in 1..column_size-1 {
            let tree_height = forest[row][column];
            let mut visibility_grid = GridInfo::<bool>::default(); // default to false
            // let mut visible = [true, true, true, true];
            for r in 0..row {
                if forest[r][column] >= tree_height {
                    visibility_grid.top = false;
                }
            }
            for r in row + 1 .. column_size {
                if forest[r][column] >= tree_height {
                    visibility_grid.bottom = false;
                }
            }
            for c in 0..column {
                if forest[row][c] >= tree_height {
                    visibility_grid.left = false;
                }
            }
            for c in column + 1 .. row_size {
                if forest[row][c] >= tree_height {
                    visibility_grid.right = false;
                }
            }
            if visibility_grid.top || visibility_grid.bottom || visibility_grid.left || visibility_grid.right { // If any sides are exposed
                visible_count += 1;
            }
        }
    }
    visible_count
}

fn scenic_scores(forest: &Forest) -> usize {
    let row_size = forest[1].len();
    let column_size = forest.len();
    let mut forest_scenic_grid_data = Vec::<GridInfo<usize>>::new();
    for row in 1..row_size-1 { // Loop through inner grid
        for column in 1..column_size-1 {
            let tree_height = forest[row][column];
            let mut scenic_score_grid = GridInfo::<usize>::default(); // defaulted to 0 values
            for r in (0..row).rev() { // reversing iterator necessary to check line of sight in correct order e.g checking left = [3][3] -> [2][3] -> [1][3]
                if forest[r][column] < tree_height {
                    scenic_score_grid.top += 1;
                }
                else {
                    scenic_score_grid.top += 1;
                    break;
                }
            }
            for r in row + 1 .. column_size {
                if forest[r][column] < tree_height {
                    scenic_score_grid.bottom += 1
                }
                else {
                    scenic_score_grid.bottom += 1;
                    break;
                }
            }
            for c in (0..column).rev() {
                if forest[row][c] < tree_height {
                    scenic_score_grid.left += 1
                }
                else {
                    scenic_score_grid.left += 1;
                    break;
                }
            }
            for c in column + 1 .. row_size {
                if forest[row][c] < tree_height {
                    scenic_score_grid.right += 1
                }
                else {
                    scenic_score_grid.right += 1;
                    break;
                }
            }
            forest_scenic_grid_data.push(scenic_score_grid);
        }
    }

    forest_scenic_grid_data // Remove score directions and fold remaining directional scores into one val per tree, and get maximum value
        .iter()
        .map(|grid|
            grid.to_vec()
                .into_iter()
                .filter(|num| *num != 0)
                .product::<usize>() // fold(1, |total, num| total * num)
        )
        .max()
        .unwrap()
}

pub fn solve() -> (usize, usize) {
    let input = fs::read_to_string(PUZZLE_INPUT).expect("Can't read file");
    let forest = parse_forest(&input);
    (visible_tree_count(&forest), scenic_scores(&forest))
}

