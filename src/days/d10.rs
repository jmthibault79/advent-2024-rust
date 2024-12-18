use crate::utils;
use crate::utils::matrix;
use crate::utils::plane;

// pub fn paths_forward(
//     plane: &Vec<Vec<u32>>,
//     target: u32,
//     start_row: usize,
//     start_col: usize,
// ) -> Vec<Vec<(usize, usize)>> {
//     let curr_val = plane[start_row][start_col];
//     println!("{} @ ({},{})", curr_val, start_row, start_col);

//     [
//         plane::Direction::Down,
//         plane::Direction::Up,
//         plane::Direction::Left,
//         plane::Direction::Right,
//     ]
//     .iter()
//     .flat_map(|d| {
//         let mo = plane::move_one(start_row, start_col, plane.len(), plane[0].len(), *d);
//         if !mo.out_of_bounds && plane[mo.row][mo.col] == curr_val + 1 {
//             if plane[mo.row][mo.col] == target {
//                 let last_step = vec![vec![(start_row, start_col), (mo.row, mo.col)]];
//                 println!("last_step: {:?}", last_step);
//                 last_step
//             } else {
//                 paths_forward(plane, target, mo.row, mo.col)
//                     .iter()
//                     .map(|f_path| {
//                         let mut path = vec![(start_row, start_col)];
//                         path.extend(f_path);
//                         path
//                     })
//                     .collect()
//             }
//         } else {
//             vec![]
//         }
//     })
//     .collect()
// }

fn one_step(
    plane: &Vec<Vec<u32>>,
    target: u32,
    starts: Vec<(usize, usize)>,
) -> Vec<(usize, usize)> {
    let raw_result = starts
        .iter()
        .flat_map(|(row, col)| {
            [
                plane::Direction::Down,
                plane::Direction::Up,
                plane::Direction::Left,
                plane::Direction::Right,
            ]
            .iter()
            .flat_map(|d| {
                let mo = plane::move_one(*row, *col, plane.len(), plane[0].len(), *d);
                if !mo.out_of_bounds && plane[mo.row][mo.col] == target {
                    vec![(mo.row, mo.col)]
                } else {
                    vec![]
                }
            })
        })
        .collect();
    utils::distinct(raw_result)
}

fn reachable_destinations(
    plane: &Vec<Vec<u32>>,
    destination: u32,
    start_row: usize,
    start_col: usize,
) -> Vec<(usize, usize)> {
    println!();
    println!("starting at n=0: {}, {}", start_row, start_col);
    let mut n = 0;
    let mut all_n = vec![(start_row, start_col)];
    while n < destination {
        n += 1;
        all_n = one_step(plane, n, all_n);
        println!("all n={}: {:?}", n, all_n);
    }
    all_n
}

fn p1(plane: &Vec<Vec<u32>>, destination: u32) -> usize {
    matrix::find_all(plane, 0)
        .iter()
        .map(|(row, col)| reachable_destinations(plane, destination, *row, *col).len())
        .sum()
}

pub fn d10p1(file_path: &str) -> usize {
    let mat = matrix::as_digit_matrix(file_path, 10);
    p1(&mat, 9)
}

pub fn d10p2(file_path: &str) -> usize {
    0
}

pub fn d10() {
//    let file_path = "inputs/d10sample1.txt";
//    let file_path = "inputs/d10sample2.txt";
    let file_path = "inputs/d10.txt";
    let mut result = d10p1(file_path);
    println!("Result Day 10 Part 1: {}", result);
    result = d10p2(file_path);
    println!("Result Day 10 Part 2: {}", result);
}

mod tests {
    use super::*;

    fn as_int_matrix(plane: Vec<String>) -> Vec<Vec<u32>> {
        plane.iter().map(|s| matrix::to_digit_vec(s, 10)).collect()
    }

    #[test]
    fn test_p1() {
        let mut mat = as_int_matrix(vec![
            "0123".to_string(),
            "1234".to_string(),
            "8765".to_string(),
            "9876".to_string(),
        ]);
        assert_eq!(p1(&mat, 9), 1);

        mat = as_int_matrix(vec![
            "...0...".to_string(),
            "...1...".to_string(),
            "...2...".to_string(),
            "6543456".to_string(),
            "7.....7".to_string(),
            "8.....8".to_string(),
            "9.....9".to_string(),
        ]);
        assert_eq!(p1(&mat, 9), 2);

        mat = as_int_matrix(vec![
            "10..9..".to_string(),
            "2...8..".to_string(),
            "3...7..".to_string(),
            "4567654".to_string(),
            "...8..3".to_string(),
            "...9..2".to_string(),
            ".....01".to_string(),
        ]);
        assert_eq!(p1(&mat, 9), 3);

        mat = as_int_matrix(vec![
            "89010123".to_string(),
            "78121874".to_string(),
            "87430965".to_string(),
            "96549874".to_string(),
            "45678903".to_string(),
            "32019012".to_string(),
            "01329801".to_string(),
            "10456732".to_string(),
        ]);
        assert_eq!(p1(&mat, 9), 36);
    }
}
