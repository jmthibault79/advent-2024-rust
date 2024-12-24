// idea: store 3 planes with identical coordinates and a Vec
// plane 1 is the input plots
// plane 2 is which region the plot is in
// NOPE, NOT NEEDED -> plane 3 is count of how much perimeter the plot has
// one Vec 0-indexed for the region for cumulative perimeter

// total area x perimeter = sum of [total-perimeter for the region] of every plot in the plane

// part 2: now need to collect edge-contibutions per plot in a Vec
// total area x sides = sum of [region sides] of every plot in the plane

use crate::utils::matrix;
use crate::utils::plane::{self, Direction, MovingObject};

fn same_region(plots: &Vec<Vec<char>>, plot_char: char, other_plot_maybe: &MovingObject) -> bool {
    !other_plot_maybe.out_of_bounds
        && plots[other_plot_maybe.row][other_plot_maybe.col] == plot_char
}

fn has_edge(plots: &Vec<Vec<char>>, row_idx: usize, col_idx: usize, dir: &Direction) -> bool {
    let (row_count, col_count) = (plots.len(), plots[0].len());
    let after_move = plane::move_one(row_idx, col_idx, row_count, col_count, *dir);
    !same_region(plots, plots[row_idx][col_idx], &after_move)
}

fn perimeter_at(plots: &Vec<Vec<char>>, row_idx: usize, col_idx: usize) -> usize {
    [
        Direction::Up,
        Direction::Right,
        Direction::Down,
        Direction::Left,
    ]
    .iter()
    .map(|dir| {
        if has_edge(plots, row_idx, col_idx, dir) {
            1
        } else {
            0
        }
    })
    .sum()
}

// what does this plot contribute to the region's edge count?
// algorithm:
// - a new L/R edge exists if the same-region 'up' plot doesn't have that edge
// - a new U/D edge exists if the same-region 'left' plot doesn't have that edge
fn edge_contribution(plots: &Vec<Vec<char>>, row_idx: usize, col_idx: usize) -> usize {
    let up = plane::move_one(row_idx, col_idx, plots.len(), plots[0].len(), Direction::Up);
    let left = plane::move_one(
        row_idx,
        col_idx,
        plots.len(),
        plots[0].len(),
        Direction::Left,
    );

    let plot_char = plots[row_idx][col_idx];
    let up_in_region = !up.out_of_bounds && same_region(plots, plot_char, &up);
    let left_in_region = !left.out_of_bounds && same_region(plots, plot_char, &left);

    let up_has_left_edge = up_in_region && has_edge(plots, up.row, up.col, &Direction::Left);
    let up_has_right_edge = up_in_region && has_edge(plots, up.row, up.col, &Direction::Right);

    let left_has_up_edge = left_in_region && has_edge(plots, left.row, left.col, &Direction::Up);
    let left_has_down_edge =
        left_in_region && has_edge(plots, left.row, left.col, &Direction::Down);

    // println!(
    //     "up_left, up_right, left_up, left_down: ({},{}) -> {} {} {} {}",
    //     row_idx, col_idx, up_has_left_edge, up_has_right_edge, left_has_up_edge, left_has_down_edge
    // );

    let mut edge_count: usize = 0;
    if !up_has_left_edge && has_edge(plots, row_idx, col_idx, &Direction::Left) {
        edge_count += 1;
    }
    if !up_has_right_edge && has_edge(plots, row_idx, col_idx, &Direction::Right) {
        edge_count += 1;
    }
    if !left_has_up_edge && has_edge(plots, row_idx, col_idx, &Direction::Up) {
        edge_count += 1;
    }
    if !left_has_down_edge && has_edge(plots, row_idx, col_idx, &Direction::Down) {
        edge_count += 1;
    }

    edge_count
}

fn merge_regions(
    a: usize,
    b: usize,
    region_of_plot: &mut Vec<Vec<usize>>,
    perimeter_of_region: &mut Vec<usize>,
    edges_of_region: &mut Vec<usize>,
    row_idx: usize,
    col_idx: usize,
) -> usize {
    let (merged_region, region_to_remove) = (a.min(b), a.max(b));

    perimeter_of_region[merged_region] += perimeter_of_region[region_to_remove];
    perimeter_of_region[region_to_remove] = 0;

    edges_of_region[merged_region] += edges_of_region[region_to_remove];
    edges_of_region[region_to_remove] = 0;

    let total_cols = region_of_plot[0].len();
    for r in 0..=row_idx {
        // only iterate up to col_idx for the last row
        let col_count = if r == row_idx { col_idx } else { total_cols };
        for c in 0..col_count {
            if region_of_plot[r][c] == region_to_remove {
                region_of_plot[r][c] = merged_region;
            }
        }
    }
    merged_region
}

fn continues_region(
    plots: &Vec<Vec<char>>,
    region_of_plot: &mut Vec<Vec<usize>>,
    perimeter_of_region: &mut Vec<usize>,
    edges_of_region: &mut Vec<usize>,
    plot_char: &char,
    row_idx: usize,
    col_idx: usize,
) -> Option<usize> {
    let (row_count, col_count) = (plots.len(), plots[0].len());

    let up = plane::move_one(row_idx, col_idx, row_count, col_count, Direction::Up);
    let left = plane::move_one(row_idx, col_idx, row_count, col_count, Direction::Left);

    match (
        same_region(plots, *plot_char, &up),
        same_region(plots, *plot_char, &left),
    ) {
        (true, true) => {
            let up_region = region_of_plot[up.row][up.col];
            let left_region = region_of_plot[left.row][left.col];
            if up_region == left_region {
                Some(up_region)
            } else {
                let merged_region = merge_regions(
                    up_region,
                    left_region,
                    region_of_plot,
                    perimeter_of_region,
                    edges_of_region,
                    row_idx,
                    col_idx,
                );
                Some(merged_region)
            }
        }
        (true, false) => Some(region_of_plot[up.row][up.col]),
        (false, true) => Some(region_of_plot[left.row][left.col]),
        (false, false) => None,
    }
}

fn parse(plots: &Vec<Vec<char>>) -> (Vec<Vec<usize>>, Vec<usize>, Vec<usize>) {
    let (row_count, col_count) = (plots.len(), plots[0].len());

    let mut region_of_plot = Vec::with_capacity(row_count);
    let mut perimeter_of_region: Vec<usize> = vec![];
    let mut edges_of_region: Vec<usize> = vec![];
    let mut next_region = 0_usize;

    for (row_idx, row) in plots.iter().enumerate() {
        region_of_plot.push(Vec::with_capacity(col_count));

        for (col_idx, plot_char) in row.iter().enumerate() {
            let plot_perimeter = perimeter_at(&plots, row_idx, col_idx);
            let plot_edges = edge_contribution(&plots, row_idx, col_idx);

            let plot_region = continues_region(
                &plots,
                &mut region_of_plot,
                &mut perimeter_of_region,
                &mut edges_of_region,
                plot_char,
                row_idx,
                col_idx,
            )
            .unwrap_or_else(|| {
                let this_region = next_region;

                // init counters for this region
                assert!(perimeter_of_region.len() == this_region);
                assert!(edges_of_region.len() == this_region);
                perimeter_of_region.push(0);
                edges_of_region.push(0);

                next_region += 1;
                this_region
            });

            region_of_plot[row_idx].push(plot_region);

            match perimeter_of_region.get_mut(plot_region) {
                Some(perimeter) => *perimeter += plot_perimeter,
                None => panic!("perimeter_of_region has no entry for {}", plot_region),
            }
            match edges_of_region.get_mut(plot_region) {
                Some(edges) => *edges += plot_edges,
                None => panic!("edges_of_region has no entry for {}", plot_region),
            }
        }
    }

    (region_of_plot, perimeter_of_region, edges_of_region)
}

fn p1(plots: &Vec<Vec<char>>) -> usize {
    let (row_count, col_count) = (plots.len(), plots[0].len());

    let (region_of_plot, perimeter_of_region, _) = parse(&plots);

    (0..row_count)
        .map(|row_idx| {
            (0..col_count)
                .map(|col_idx| {
                    let region = region_of_plot[row_idx][col_idx];
                    perimeter_of_region[region]
                })
                .sum::<usize>()
        })
        .sum()
}

fn p2(plots: &Vec<Vec<char>>) -> usize {
    let (row_count, col_count) = (plots.len(), plots[0].len());

    let (region_of_plot, _, edges_of_region) = parse(&plots);

    (0..row_count)
        .map(|row_idx| {
            (0..col_count)
                .map(|col_idx| {
                    let region = region_of_plot[row_idx][col_idx];
                    let result = edges_of_region[region];
                    // println!("region {} -> {} edges", region, result);
                    result
                })
                .sum::<usize>()
        })
        .sum()
}

pub fn d12p1(file_path: &str) -> usize {
    let plots = matrix::as_char_matrix(file_path);
    p1(&plots)
}

pub fn d12p2(file_path: &str) -> usize {
    let plots = matrix::as_char_matrix(file_path);
    p2(&plots)
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
        let mut region_of_plot = vec![vec![]];
        assert!(continues_region(
            &plots,
            &mut region_of_plot,
            &mut vec![],
            &mut vec![],
            &'A',
            0,
            0
        )
        .is_none());

        plots = vec![vec!['A', 'A']];
        region_of_plot = vec![vec![0]];
        assert_eq!(
            continues_region(
                &plots,
                &mut region_of_plot,
                &mut vec![],
                &mut vec![],
                &'A',
                0,
                1
            ),
            Some(0)
        );
    }

    #[test]
    fn test_continues_with_merging() {
        let plots = vec![vec!['A', 'B'], vec!['B', 'B']];
        let mut region_of_plot = vec![vec![0, 1], vec![2]];
        let mut perimeter_of_region = vec![4, 3, 3];
        let mut edges_of_region = vec![4, 3, 3];

        assert_eq!(
            continues_region(
                &plots,
                &mut region_of_plot,
                &mut perimeter_of_region,
                &mut edges_of_region,
                &'B',
                1,
                1
            ),
            Some(1)
        );
        assert_eq!(region_of_plot, vec![vec![0, 1], vec![1]]);
        assert_eq!(perimeter_of_region, vec![4, 6, 0]);
        assert_eq!(edges_of_region, vec![4, 6, 0]);
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

    #[test]
    fn test_p2() {
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
        assert_eq!(p2(&plots), 80);

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
        assert_eq!(p2(&plots), 436);

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
        assert_eq!(p2(&plots), 1206);
    }
}
