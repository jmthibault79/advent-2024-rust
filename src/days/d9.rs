use std::{fs, usize};

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

fn defrag(fragmented: Vec<Option<usize>>) -> Vec<usize> {
    let (mut dest_gap_idx, mut source_frag_idx) = (0_usize, fragmented.len() - 1);

    let mut result = vec![];
    while dest_gap_idx <= source_frag_idx {
        match (fragmented[dest_gap_idx], fragmented[source_frag_idx]) {
            (Some(file_id), _) => {
                result.push(file_id);
                dest_gap_idx += 1;
            }
            (None, Some(file_id)) => {
                result.push(file_id);
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

fn checksum(defragged: Vec<usize>) -> usize {
    defragged
        .iter()
        .enumerate()
        .map(|(i, file_id)| i * file_id)
        .sum()
}

pub fn d9p1(file_path: &str) -> usize {
    let disk_map: String =
        fs::read_to_string(file_path).expect(format!("Could not read file {}", file_path).as_str());
    let fragmented = to_fragmented(disk_map);
    let defragged = defrag(fragmented);
    checksum(defragged)
}

pub fn d9p2(file_path: &str) -> usize {
    0
}

pub fn d9() {
    //let file_path = "inputs/d9sample1.txt";
    //let file_path = "inputs/d9sample2.txt";
    let file_path = "inputs/d9.txt";
    let mut result = d9p1(file_path);
    println!("Result Day X Part 1: {}", result);
    result = d9p2(file_path);
    println!("Result Day X Part 2: {}", result);
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
    fn defrag_test() {
        let mut fragmented = vec![Some(0), None, None, Some(1), None, Some(2), Some(2)];
        assert_eq!(defrag(fragmented), vec![0, 2, 2, 1]);
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
        assert_eq!(defrag(fragmented), vec![0, 2, 2, 1, 1, 1, 2, 2, 2]);
    }
}
