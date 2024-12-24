// idea: store 3 planes with identical coordinates and a Vec
// plane 1 is the input plots
// plane 2 is which region the plot is in
// NOPE, NOT NEEDED -> plane 3 is count of how much perimeter the plot has
// one Vec 0-indexed for the region for cumulative perimeter

// total = sum of [total-perimeter for the region] of every plot in the plane

use crate::utils::matrix;
use crate::utils::plane::{self, Direction};

fn perimeter_at(plots: &Vec<Vec<char>>, row_idx: usize, col_idx: usize) -> usize {
    let (row_count, col_count) = (plots.len(), plots[0].len());
    [
        Direction::Up,
        Direction::Right,
        Direction::Down,
        Direction::Left,
    ]
    .iter()
    .map(|dir| {
        let after_move = plane::move_one(row_idx, col_idx, row_count, col_count, *dir);
        // perimeters are with different plot types or the outer wall
        if after_move.out_of_bounds
            || plots[after_move.row][after_move.col] != plots[row_idx][col_idx]
        {
            1
        } else {
            0
        }
    })
    .sum()
}

fn merge_regions(
    a: usize,
    b: usize,
    region: &mut Vec<Vec<usize>>,
    total_perimeter: &mut Vec<usize>,
    row_idx: usize,
    col_idx: usize,
) -> usize {
    let (merged_region, region_to_remove) = (a.min(b), a.max(b));

    total_perimeter[merged_region] += total_perimeter[region_to_remove];
    total_perimeter[region_to_remove] = 0;

    let total_cols = region[0].len();
    for r in 0..=row_idx {
        // only iterate up to col_idx for the last row
        let col_count = if r == row_idx { col_idx } else { total_cols };
        for c in 0..col_count {
            if region[r][c] == region_to_remove {
                region[r][c] = merged_region;
            }
        }
    }
    merged_region
}

fn continues_region(
    plots: &Vec<Vec<char>>,
    region: &mut Vec<Vec<usize>>,
    total_perimeter: &mut Vec<usize>,
    char: &char,
    row_idx: usize,
    col_idx: usize,
) -> Option<usize> {
    let (row_count, col_count) = (plots.len(), plots[0].len());

    let up = plane::move_one(row_idx, col_idx, row_count, col_count, Direction::Up);
    let continues_up_region = !up.out_of_bounds && plots[up.row][up.col] == *char;
    let left = plane::move_one(row_idx, col_idx, row_count, col_count, Direction::Left);
    let continues_left_region = !left.out_of_bounds && plots[left.row][left.col] == *char;

    match (continues_up_region, continues_left_region) {
        (true, true) => {
            let up_region = region[up.row][up.col];
            let left_region = region[left.row][left.col];
            if up_region != left_region {
                let merged_region = merge_regions(
                    up_region,
                    left_region,
                    region,
                    total_perimeter,
                    row_idx,
                    col_idx,
                );
                Some(merged_region)
            } else {
                Some(up_region)
            }
        }
        (true, false) => Some(region[up.row][up.col]),
        (false, true) => Some(region[left.row][left.col]),
        (false, false) => None,
    }
}

fn parse(plots: &Vec<Vec<char>>) -> (Vec<Vec<usize>>, Vec<usize>) {
    let (row_count, col_count) = (plots.len(), plots[0].len());

    let mut region = Vec::with_capacity(row_count);
    let mut total_perimeter: Vec<usize> = vec![];
    let mut next_region = 0_usize;

    for (row_idx, row) in plots.iter().enumerate() {
        region.push(Vec::with_capacity(col_count));

        for (col_idx, char) in row.iter().enumerate() {
            let plot_perimeter = perimeter_at(&plots, row_idx, col_idx);
            let plot_region = continues_region(
                &plots,
                &mut region,
                &mut total_perimeter,
                char,
                row_idx,
                col_idx,
            )
            .unwrap_or_else(|| {
                let this_region = next_region;
                next_region += 1;
                this_region
            });

            region[row_idx].push(plot_region);

            match total_perimeter.get_mut(plot_region) {
                Some(perimeter) => *perimeter += plot_perimeter,
                None => total_perimeter.push(plot_perimeter),
            }
        }
    }

    (region, total_perimeter)
}

fn p1(plots: &Vec<Vec<char>>) -> usize {
    let (row_count, col_count) = (plots.len(), plots[0].len());

    let (region, total_perimeter) = parse(&plots);

    // print!("region:");
    // for row in region.clone() {
    //     println!("{:?}", row);
    //  }

    // println!("total_perimeter: {:?}", total_perimeter);

    (0..row_count)
        .map(|row_idx| {
            (0..col_count)
                .map(|col_idx| {
                    let region = region[row_idx][col_idx];
                    total_perimeter[region]
                })
                .sum::<usize>()
        })
        .sum()
}

pub fn d12p1(file_path: &str) -> usize {
    let plots = matrix::as_char_matrix(file_path);
    p1(&plots)
}

pub fn d12p2(_file_path: &str) -> usize {
    0
}

pub fn d12() {
    // let file_path = "inputs/d12sample1.txt";
    // let file_path = "inputs/d12sample2.txt";
    let file_path = "inputs/d12.txt";
    let mut result = d12p1(file_path);
    println!("Result Day 12 Part 1: {}", result);
    result = d12p2(file_path);
    println!("Result Day 12 Part 2: {}", result);
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_perim() {
        let mut plots = vec![vec!['A']];
        assert_eq!(perimeter_at(&plots, 0, 0), 4);

        plots = vec![vec!['A', 'A'], vec!['A', 'A']];
        assert_eq!(perimeter_at(&plots, 0, 0), 2);
        assert_eq!(perimeter_at(&plots, 0, 1), 2);
        assert_eq!(perimeter_at(&plots, 1, 0), 2);
        assert_eq!(perimeter_at(&plots, 1, 1), 2);

        plots = vec![vec!['A', 'A'], vec!['A', 'B']];
        assert_eq!(perimeter_at(&plots, 0, 0), 2);
        assert_eq!(perimeter_at(&plots, 0, 1), 3);
        assert_eq!(perimeter_at(&plots, 1, 0), 3);
        assert_eq!(perimeter_at(&plots, 1, 1), 4);

        plots = vec![vec!['A', 'B'], vec!['B', 'B']];
        assert_eq!(perimeter_at(&plots, 0, 0), 4);
        assert_eq!(perimeter_at(&plots, 0, 1), 3);
        assert_eq!(perimeter_at(&plots, 1, 0), 3);
        assert_eq!(perimeter_at(&plots, 1, 1), 2);
    }

    #[test]
    fn test_continues() {
        let mut plots = vec![vec!['A']];
        let mut region = vec![vec![]];
        assert!(continues_region(&plots, &mut region, &mut vec![], &'A', 0, 0).is_none());

        plots = vec![vec!['A', 'A']];
        region = vec![vec![0]];
        assert_eq!(
            continues_region(&plots, &mut region, &mut vec![], &'A', 0, 1),
            Some(0)
        );
    }

    #[test]
    fn test_continues_with_merging() {
        let mut plots = vec![vec!['A', 'B'], vec!['B', 'B']];
        let mut region = vec![vec![0, 1], vec![2]];
        let mut total_perimeter = vec![4, 3, 3];

        assert_eq!(
            continues_region(&plots, &mut region, &mut total_perimeter, &'B', 1, 1),
            Some(1)
        );
        assert_eq!(region, vec![vec![0, 1], vec![1]]);
        assert_eq!(total_perimeter, vec![4, 6, 0]);
    }

    #[test]
    fn test_p1() {
        let mut plots = vec![
            // AAAA
            // BBCD
            // BBCC
            // EEEC
            vec!['A', 'A', 'A', 'A'],
            vec!['B', 'B', 'C', 'D'],
            vec!['B', 'B', 'C', 'C'],
            vec!['E', 'E', 'E', 'C'],
        ];
        assert_eq!(p1(&plots), 140);

        plots = vec![
            // OOOOO
            // OXOXO
            // OOOOO
            // OXOXO
            // OOOOO
            vec!['O', 'O', 'O', 'O', 'O'],
            vec!['O', 'X', 'O', 'X', 'O'],
            vec!['O', 'O', 'O', 'O', 'O'],
            vec!['O', 'X', 'O', 'X', 'O'],
            vec!['O', 'O', 'O', 'O', 'O'],
        ];
        assert_eq!(p1(&plots), 772);

        plots = vec![
            // RRRRIICCFF
            // RRRRIICCCF
            // VVRRRCCFFF
            // VVRCCCJFFF
            // VVVVCJJCFE
            // VVIVCCJJEE
            // VVIIICJJEE
            // MIIIIIJJEE
            // MIIISIJEEE
            // MMMISSJEEE
            vec!['R', 'R', 'R', 'R', 'I', 'I', 'C', 'C', 'F', 'F'],
            vec!['R', 'R', 'R', 'R', 'I', 'I', 'C', 'C', 'C', 'F'],
            vec!['V', 'V', 'R', 'R', 'R', 'C', 'C', 'F', 'F', 'F'],
            vec!['V', 'V', 'R', 'C', 'C', 'C', 'J', 'F', 'F', 'F'],
            vec!['V', 'V', 'V', 'V', 'C', 'J', 'J', 'C', 'F', 'E'],
            vec!['V', 'V', 'I', 'V', 'C', 'C', 'J', 'J', 'E', 'E'],
            vec!['V', 'V', 'I', 'I', 'I', 'C', 'J', 'J', 'E', 'E'],
            vec!['M', 'I', 'I', 'I', 'I', 'I', 'J', 'J', 'E', 'E'],
            vec!['M', 'I', 'I', 'I', 'S', 'I', 'J', 'E', 'E', 'E'],
            vec!['M', 'M', 'M', 'I', 'S', 'S', 'J', 'E', 'E', 'E'],
        ];
        assert_eq!(p1(&plots), 1930);
    }
}
