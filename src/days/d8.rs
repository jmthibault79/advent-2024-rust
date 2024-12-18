use crate::utils::{self, matrix};
use multimap::MultiMap;

const EMPTY_SPACE: char = '.';
fn parse_antennas(mat: &Vec<Vec<char>>) -> MultiMap<char, (usize, usize)> {
    let mut antennas = MultiMap::new();
    for (row, row_vec) in mat.iter().enumerate() {
        for (col, col_vec) in row_vec.iter().enumerate() {
            if *col_vec != EMPTY_SPACE {
                antennas.insert(*col_vec, (row, col));
            }
        }
    }
    antennas
}

pub fn calc_antinodes(
    (a_row, a_col): (isize, isize),
    (b_row, b_col): (isize, isize),
    row_count: usize,
    col_count: usize,
    p1: bool,
) -> Vec<(usize, usize)> {
    let row_diff = a_row - b_row;
    let col_diff = a_col - b_col;
    let mut antinodes = vec![];

    if p1 {
        let (c_row, c_col) = (a_row + row_diff, a_col + col_diff);
        let (d_row, d_col) = (b_row - row_diff, b_col - col_diff);

        if matrix::in_bounds(row_count, col_count, c_row, c_col) {
            antinodes.push((c_row as usize, c_col as usize));
        }
        if matrix::in_bounds(row_count, col_count, d_row, d_col) {
            antinodes.push((d_row as usize, d_col as usize));
        }
    } else {
        let (mut up_counter, mut row, mut col) = (0, a_row, a_col);
        while matrix::in_bounds(row_count, col_count, row, col) {
            antinodes.push((row as usize, col as usize));
            up_counter += 1;
            row = a_row + up_counter * row_diff;
            col = a_col + up_counter * col_diff;
        }

        let mut down_counter = -1;
        row = a_row + down_counter * row_diff;
        col = a_col + down_counter * col_diff;
        while matrix::in_bounds(row_count, col_count, row, col) {
            antinodes.push((row as usize, col as usize));
            down_counter -= 1;
            row = a_row + down_counter * row_diff;
            col = a_col + down_counter * col_diff;
        }
    }
    antinodes
}

pub fn calc_all_antinodes(
    antennas: &Vec<(isize, isize)>,
    row_count: usize,
    col_count: usize,
    p1: bool,
) -> Vec<(usize, usize)> {
    utils::all_pairs(antennas)
        .iter()
        .flat_map(|(a, b)| calc_antinodes(*a, *b, row_count, col_count, p1))
        .collect()
}

pub fn d8p1(file_path: &str) -> usize {
    let mat = matrix::as_char_matrix(file_path);
    let antennas = parse_antennas(&mat);
    let antinodes: Vec<(usize, usize)> = antennas
        .iter_all()
        .map(|(_, v)| v.iter().map(|(a, b)| (*a as isize, *b as isize)).collect())
        .flat_map(|v| calc_all_antinodes(&v, mat.len(), mat[0].len(), true))
        .collect();
    matrix::unique_coordinates(&antinodes).len()
}

pub fn d8p2(file_path: &str) -> usize {
    let mat = matrix::as_char_matrix(file_path);
    let antennas = parse_antennas(&mat);
    let antinodes: Vec<(usize, usize)> = antennas
        .iter_all()
        .map(|(_, v)| v.iter().map(|(a, b)| (*a as isize, *b as isize)).collect())
        .flat_map(|v| calc_all_antinodes(&v, mat.len(), mat[0].len(), false))
        .collect();
    matrix::unique_coordinates(&antinodes).len()
}

pub fn d8() {
    //let file_path = "inputs/d8sample.txt";
    let file_path = "inputs/d8.txt";
    let mut result = d8p1(file_path);
    println!("Result Day 8 Part 1: {}", result);
    result = d8p2(file_path);
    println!("Result Day 8 Part 2: {}", result);
}

mod tests {
    use super::*;

    #[test]
    fn parse_test() {
        let mut mat = vec![vec!['.']];
        assert_eq!(parse_antennas(&mat), MultiMap::new());

        mat = vec![vec!['a', 'a'], vec!['b', '.']];
        let expected = {
            let mut map = MultiMap::new();
            map.insert('a', (0, 0));
            map.insert('a', (0, 1));
            map.insert('b', (1, 0));
            map
        };
        assert_eq!(parse_antennas(&mat), expected);
    }

    #[test]
    fn antinodes_test() {
        assert_eq!(calc_antinodes((0, 0), (1, 1), 2, 2, true), vec![]);
        assert_eq!(calc_antinodes((0, 0), (1, 1), 7, 7, true), vec![(2, 2)]);
        assert_eq!(
            calc_antinodes((2, 2), (1, 1), 7, 7, true).sort(),
            vec![(0, 0), (3, 3)].sort()
        );
    }

    #[test]
    fn antinodes_p2_test() {
        assert_eq!(
            calc_antinodes((0, 0), (1, 1), 2, 2, false).sort(),
            vec![(0, 0), (1, 1)].sort()
        );
        assert_eq!(
            calc_antinodes((0, 0), (1, 1), 3, 3, false).sort(),
            vec![(0, 0), (1, 1), (2, 2)].sort()
        );
        assert_eq!(
            calc_antinodes((2, 2), (1, 1), 4, 4, false).sort(),
            vec![(0, 0), (1, 1), (2, 2), (3, 3)].sort()
        );
        assert_eq!(
            calc_antinodes((2, 2), (1, 1), 6, 7, false).sort(),
            vec![(0, 0), (1, 1), (2, 2), (3, 3), (4, 4), (5, 5)].sort()
        );
    }

    #[test]
    fn antinodes_all_test() {
        assert_eq!(
            calc_all_antinodes(&vec![(0, 0), (1, 1)], 2, 2, true),
            vec![]
        );
        assert_eq!(
            calc_all_antinodes(&vec![(0, 0), (1, 1)], 3, 3, true),
            vec![(2, 2)]
        );
        assert_eq!(
            calc_all_antinodes(&vec![(2, 2), (1, 1)], 4, 4, true).sort(),
            vec![(0, 0), (3, 3)].sort()
        );

        assert_eq!(
            calc_all_antinodes(&vec![(10, 10), (15, 15), (17, 11)], 22, 22, true).sort(),
            vec![(5, 5), (20, 20), (19, 7), (13, 19), (3, 9)].sort()
        );
    }
}
