use crate::utils;

fn vec_scan(v: &Vec<char>, to_scan: &str) -> i32 {
    if v.len() < to_scan.len() {
        return 0;
    }

    let to_scan_rev: String = to_scan.chars().rev().collect();

    let mut count: i32 = 0;

    for scan_index in 0..=(v.len() - to_scan.len()) {
        let scan_slice = &v[scan_index..scan_index + to_scan.len()];
        let scan_str: String = scan_slice.iter().collect();
        if scan_str == to_scan || scan_str == to_scan_rev {
            count += 1;
        }
    }

    count
}

fn diagonal_matrix_scan(mat: &Vec<Vec<char>>, to_scan: &str) -> i32 {
    if to_scan.len() < 2 {
        panic!("this algorithm doesn't work for len 0 or 1");
    }

    if mat.len() < to_scan.len() || mat[0].len() < to_scan.len() {
        return 0;
    }

    let to_scan_rev: String = to_scan.chars().rev().collect();

    let mut count: i32 = 0;

    let rows_to_check = mat.len() - to_scan.len() + 1;
    let cols_to_check = mat[0].len() - to_scan.len() + 1;
    let start_col_for_rev_check = to_scan.len() - 1;

    let mut scan_vec: Vec<char> = Vec::new();
    for row in 0..rows_to_check {
        for col in 0..cols_to_check {
            for slice_builder in 0..to_scan.len() {
                scan_vec.push(mat[row + slice_builder][col + slice_builder]);
            }
            let scan_str: String = scan_vec.iter().collect();
            if scan_str == to_scan || scan_str == to_scan_rev {
                count += 1;
            }
            scan_vec.clear();
        }
        for col in start_col_for_rev_check..mat[0].len() {
            for slice_builder in 0..to_scan.len() {
                scan_vec.push(mat[row + slice_builder][col - slice_builder]);
            }
            let scan_str: String = scan_vec.iter().collect();
            if scan_str == to_scan || scan_str == to_scan_rev {
                count += 1;
            }
            scan_vec.clear();
        }
    }

    count
}

fn d4p1(path: &str) -> i32 {
    let scan_pattern = "XMAS";

    let mut mat = utils::as_matrix(path);
    let mut count: i32 = 0;

    count += mat
        .iter()
        .map(|row| vec_scan(row, scan_pattern))
        .sum::<i32>();

    // ok to replace original because it does not change the diagonal search
    mat = utils::flip_matrix(&mat);

    count += mat
        .iter()
        .map(|row| vec_scan(row, scan_pattern))
        .sum::<i32>();

    count + diagonal_matrix_scan(&mat, scan_pattern)
}

fn d4p2(path: &str) -> i32 {
    0
}

pub fn d4() {
    //let path = "inputs/d4sample1.txt";
    //let path = "inputs/d4sample2.txt";
    let path = "inputs/d4.txt";
    let mut result = d4p1(path);
    println!("Result Day 4 Part 1: {}", result);
    result = d4p2(path);
    println!("Result Day 4 Part 2: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scan_empty_vec() {
        let result = vec_scan(&vec![], "a");
        assert_eq!(result, 0);
    }

    #[test]
    fn scan_simplest() {
        let result = vec_scan(&vec!['a'], "a");
        assert_eq!(result, 1);
    }

    #[test]
    fn scan_start() {
        let result = vec_scan(&vec!['a', 'b', 'c', 'd'], "a");
        assert_eq!(result, 1);
    }

    #[test]
    fn scan_end() {
        let result = vec_scan(&vec!['a', 'b', 'c', 'd'], "d");
        assert_eq!(result, 1);
    }

    #[test]
    fn scan_2a() {
        let result = vec_scan(&vec!['a', 'a'], "a");
        assert_eq!(result, 2);
    }

    #[test]
    fn scan_2ab() {
        let result = vec_scan(&vec!['a', 'b', 'c', 'a', 'b'], "ab");
        assert_eq!(result, 2);
    }

    #[test]
    fn scan_vec_too_small() {
        let result = vec_scan(&vec!['a', 'b', 'c'], "abcd");
        assert_eq!(result, 0);
    }

    #[test]
    fn scan_empty_mat() {
        let result = diagonal_matrix_scan(&vec![vec![]], "ab");
        assert_eq!(result, 0);
    }

    #[test]
    fn scan_mat_too_small_vert() {
        let matrix = vec![vec!['a', 'b', 'c']];
        let result = diagonal_matrix_scan(&matrix, "ab");
        assert_eq!(result, 0);
    }

    #[test]
    fn scan_mat_too_small_horiz() {
        let matrix = vec![vec!['a'], vec!['b'], vec!['c']];
        let result = diagonal_matrix_scan(&matrix, "ab");
        assert_eq!(result, 0);
    }

    #[test]
    fn scan_diag_2x2() {
        let matrix = vec![vec!['a', 'b'], vec!['c', 'd']];
        assert_eq!(diagonal_matrix_scan(&matrix, "ad"), 1);
        assert_eq!(diagonal_matrix_scan(&matrix, "da"), 1);
        assert_eq!(diagonal_matrix_scan(&matrix, "bc"), 1);
        assert_eq!(diagonal_matrix_scan(&matrix, "cb"), 1);
    }
}
