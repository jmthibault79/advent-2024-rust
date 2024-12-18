use std::fmt::Display;

fn to_char_vec(s: String) -> Vec<char> {
    s.chars().collect()
}

pub fn as_char_matrix(path: &str) -> Vec<Vec<char>> {
    super::string_iter(path).map(to_char_vec).collect()
}

pub fn as_int_matrix(path: &str) -> Vec<Vec<u32>> {
    super::string_iter(path).map(super::to_int_vec).collect()
}

pub fn flip_matrix<T: Copy>(mat: &Vec<Vec<T>>) -> Vec<Vec<T>> {
    let height = mat.len();
    let width = mat[0].len();

    let mut flipped: Vec<Vec<T>> = vec![vec![mat[0][0]; height]; width];

    for h_idx in 0..height {
        for w_idx in 0..width {
            flipped[w_idx][h_idx] = mat[h_idx][w_idx];
        }
    }
    flipped
}

// return NxM new matrices, with the same values as the original except a single value is replaced with a new value
pub fn replace_one_cell<T: Copy>(mat: &Vec<Vec<T>>, new_val: T) -> Vec<Vec<Vec<T>>> {
    let mut result = Vec::new();
    for (row_idx, row) in mat.iter().enumerate() {
        for (col_idx, _) in row.iter().enumerate() {
            let mut new_mat = mat.clone();
            new_mat[row_idx][col_idx] = new_val;
            result.push(new_mat);
        }
    }
    result
}

pub fn pretty_print<T: Display>(mat: &Vec<Vec<T>>) {
    for row in mat {
        for cell in row {
            print!("{}", cell);
        }
        println!();
    }
}

pub fn unique_coordinates(mat: &Vec<(usize, usize)>) -> Vec<(usize, usize)> {
    let mut unique = vec![];
    for coord in mat {
        if !unique.contains(coord) {
            unique.push(*coord);
        }
    }
    unique
}

pub fn in_bounds(row_count: usize, col_count: usize, row: isize, col: isize) -> bool {
    row >= 0 && row < row_count as isize && col >= 0 && col < col_count as isize
}

pub fn in_bounds_m<T>(mat: &Vec<Vec<T>>, row: isize, col: isize) -> bool {
    in_bounds(mat.len(), mat[0].len(), row, col)
}

pub fn find_all<T: PartialEq>(mat: &Vec<Vec<T>>, target: T) -> Vec<(usize, usize)> {
    let mut result = Vec::new();
    for (row_idx, row) in mat.iter().enumerate() {
        for (col_idx, cell) in row.iter().enumerate() {
            if *cell == target {
                result.push((row_idx, col_idx));
            }
        }
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_all_test() {
        assert_eq!(find_all(&vec![vec![0]], 0), vec![(0, 0)]);
        assert_eq!(
            find_all(&vec![vec![0, 1], vec![1, 0]], 1),
            vec![(0, 1), (1, 0)]
        );
    }
}
