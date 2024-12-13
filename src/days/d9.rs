use crate::utils;
use std::usize;

fn to_fragmented(disk_map: String) -> Vec<Option<usize>> {
    let mut result = vec![];
    let mut file_id = 0;
    let mut is_file = true;

    for size in disk_map.chars().map(|c| {
        c.to_digit(10)
            .expect(format!("could not parse '{}' as digit", c).as_str())
    }) {
        for _ in 0..size {
            result.push(if is_file { Some(file_id) } else { None });
        }
        if is_file {
            file_id += 1;
        }
        is_file = !is_file;
    }

    //println!("fragmented -> {:?}", result);
    result
}

fn defrag_p1(fragmented: Vec<Option<usize>>) -> Vec<Option<usize>> {
    let (mut dest_gap_idx, mut source_frag_idx) = (0_usize, fragmented.len() - 1);

    let mut result = vec![];
    while dest_gap_idx <= source_frag_idx {
        match (fragmented[dest_gap_idx], fragmented[source_frag_idx]) {
            (Some(file_id), _) => {
                result.push(Some(file_id));
                dest_gap_idx += 1;
            }
            (None, Some(file_id)) => {
                result.push(Some(file_id));
                dest_gap_idx += 1;
                source_frag_idx -= 1;
            }
            (None, None) => {
                source_frag_idx -= 1;
            }
        }
    }

    //println!("defragmented -> {:?}", result);
    result
}

// find the first gap of size file_len or greater
fn first_gap_before(
    defragmented: &mut Vec<Option<usize>>,
    file_src_idx: usize,
    file_len: usize,
) -> Option<usize> {
    let mut gap_len = 0;
    let mut gap_idx = 0;
    for (i, file_id) in defragmented[0..file_src_idx].iter().enumerate() {
        match file_id {
            None => {
                gap_len += 1;
                if gap_len == file_len {
                    return Some(gap_idx);
                }
            }
            Some(_) => {
                gap_len = 0;
                gap_idx = i + 1;
            }
        }
    }
    None
}

fn maybe_move_file(
    defragmented: &mut Vec<Option<usize>>,
    file_id: usize,
    file_len: usize,
    file_src_idx: usize,
) {
    if let Some(dest_gap_idx) = first_gap_before(defragmented, file_src_idx, file_len) {
        for i in 0..file_len {
            defragmented[dest_gap_idx + i] = Some(file_id);
            defragmented[file_src_idx + i] = None;
        }
    }
}

fn defrag_p2(fragmented: Vec<Option<usize>>) -> Vec<Option<usize>> {
    let mut defragmented = fragmented.clone();
    let mut source_frag_idx = fragmented.len() - 1;

    let (mut current_file_id, mut current_file_len): (Option<usize>, usize) = (None, 0);

    while source_frag_idx > 0 {
        match (fragmented[source_frag_idx], current_file_id) {
            // traversing through a gap; continue
            (None, None) => {
                source_frag_idx -= 1;
            }
            // begin file traversal
            (Some(file_id), None) => {
                current_file_id = Some(file_id);
                current_file_len = 1;
                source_frag_idx -= 1;
            }
            // continue traversing the same file
            (Some(traversal_file_id), Some(curr_file_id)) if traversal_file_id == curr_file_id => {
                current_file_len += 1;
                source_frag_idx -= 1;
            }
            // current file traversal ends because we're either in a gap or traversing a new file
            (_, Some(curr_file_id)) => {
                //println!("pre move (idx={}): {:?}", source_frag_idx, defragmented);
                maybe_move_file(
                    &mut defragmented,
                    curr_file_id,
                    current_file_len,
                    source_frag_idx + 1,
                );
                //println!("post move (idx={}): {:?}", source_frag_idx, defragmented);
                (current_file_id, current_file_len) = (None, 0);
            }
        }
    }

    defragmented
}

fn checksum(defragged: Vec<Option<usize>>) -> usize {
    defragged
        .iter()
        .enumerate()
        .map(|(i, file_id)| i * file_id.unwrap_or(0))
        .sum()
}

pub fn d9p1(file_path: &str) -> usize {
    let disk_map = utils::read_all(file_path);
    let fragmented = to_fragmented(disk_map);
    let defragged = defrag_p1(fragmented);
    checksum(defragged)
}

pub fn d9p2(file_path: &str) -> usize {
    let disk_map = utils::read_all(file_path);
    let fragmented = to_fragmented(disk_map);
    let defragged = defrag_p2(fragmented);
    checksum(defragged)
}

pub fn d9() {
    //let file_path = "inputs/d9sample1.txt";
    //let file_path = "inputs/d9sample2.txt";
    let file_path = "inputs/d9.txt";
    let mut result = d9p1(file_path);
    println!("Result Day 9 Part 1: {}", result);
    result = d9p2(file_path);
    println!("Result Day 9 Part 2: {}", result);
}

mod tests {
    use super::*;

    #[test]
    fn to_fragmented_test() {
        let mut disk_map = "1".to_string();
        assert_eq!(to_fragmented(disk_map), vec![Some(0)]);
        disk_map = "12".to_string();
        assert_eq!(to_fragmented(disk_map), vec![Some(0), None, None]);
        disk_map = "123".to_string();
        assert_eq!(
            to_fragmented(disk_map),
            vec![Some(0), None, None, Some(1), Some(1), Some(1)]
        );
    }

    #[test]
    fn defrag_p1_test() {
        let mut fragmented = vec![Some(0), None, None, Some(1), None, Some(2), Some(2)];
        assert_eq!(
            defrag_p1(fragmented),
            vec![Some(0), Some(2), Some(2), Some(1)]
        );
        fragmented = vec![
            Some(0),
            None,
            None,
            Some(1),
            Some(1),
            Some(1),
            None,
            None,
            None,
            None,
            Some(2),
            Some(2),
            Some(2),
            Some(2),
            Some(2),
        ]; // 12345
        assert_eq!(
            defrag_p1(fragmented),
            vec![
                Some(0),
                Some(2),
                Some(2),
                Some(1),
                Some(1),
                Some(1),
                Some(2),
                Some(2),
                Some(2)
            ]
        );
    }

    #[test]
    fn defrag_p2_test() {
        let mut fragmented = vec![Some(0), None, None, Some(1), None, Some(2), Some(2)];
        assert_eq!(
            defrag_p2(fragmented),
            vec![Some(0), Some(2), Some(2), Some(1), None, None, None]
        );
    }
}
