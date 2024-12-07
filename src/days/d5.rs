use crate::utils;
use multimap::MultiMap;

fn parse_rule(s: String) -> (u32, u32) {
    let mut parts = s.split('|');
    let a = parts
        .next()
        .unwrap()
        .parse()
        .expect(format!("could not parse integer a from {}", s).as_str());
    let b = parts
        .next()
        .unwrap()
        .parse()
        .expect(format!("could not parse integer b from {}", s).as_str());
    if parts.next().is_some() {
        panic!("expected rule in format a|b, saw: {}", s);
    }
    (a, b)
}

fn parse_page_data(s: String) -> Vec<u32> {
    s.split(',')
        .map(|x| {
            x.parse()
                .expect(format!("could not parse page_data from {}", s).as_str())
        })
        .collect()
}

fn parse_pages(i: impl Iterator<Item = String>) -> (MultiMap<u32, u32>, Vec<Vec<u32>>) {
    let mut rules = MultiMap::new();
    let mut page_data = Vec::new();

    // parse file in 2 phases: rules, then data
    let mut parsing_rules = true;

    for line in i {
        match (parsing_rules, line.len()) {
            // first, parse rules
            (true, n) if n > 0 => {
                let (a, b) = parse_rule(line);
                rules.insert(a, b);
            }
            // then switch to parsing page data
            (true, _) => {
                parsing_rules = false;
            }
            // parse page data
            (false, n) if n > 0 => {
                page_data.push(parse_page_data(line));
            }
            // end of file
            _ => return (rules, page_data),
        }
    }

    (rules, page_data)
}

fn middle_value(data: Vec<u32>) -> u32 {
    if data.len() % 2 == 0 {
        panic!(
            "expected odd number of pages, got {} for {:?}",
            data.len(),
            data
        );
    } else {
        data[data.len() / 2]
    }
}

// a rule means: KEY must always come before any of the VALUEs in the data
// it's fine if the KEY or any VALUE is not in the rules
fn check_pages(data: Vec<u32>, rules: &MultiMap<u32, u32>) -> u32 {
    let mut before_me: Vec<u32> = Vec::new();
    for page in &data {
        for must_come_before in &before_me {
            if let Some(to_check) = rules.get_vec(&page) {
                if to_check.contains(&must_come_before) {
                    return 0;
                }
            }
        }
        before_me.push(*page);
    }
    middle_value(data)
}

fn d5p1(path: &str) -> u32 {
    let (rules, page_data) = parse_pages(utils::string_iter(path));
    page_data
        .into_iter()
        .map(|data| check_pages(data, &rules))
        .sum()
}

pub fn d5() {
    //let path = "inputs/d5sample.txt";
    let path = "inputs/d5.txt";
    let mut result = d5p1(path);
    println!("Result Day 5 Part 1: {}", result);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_rule_simple() {
        let (a, b) = parse_rule("1|2000".to_string());
        assert_eq!(a, 1);
        assert_eq!(b, 2000);
    }

    #[test]
    fn parse_page_data_simple() {
        assert_eq!(
            parse_page_data("1,2,3,4,50000".to_string()),
            vec![1, 2, 3, 4, 50000]
        );
    }

    #[test]
    fn parse_pages_simple() {
        let source_data = vec![
            "1|2".to_string(),
            "3|4".to_string(),
            "".to_string(),
            "1,2,3,4,50000".to_string(),
            "9,10,11".to_string(),
        ];
        let (rules, page_data) = parse_pages(source_data.into_iter());
        assert_eq!(*rules.get_vec(&1).unwrap(), vec![2u32]);
        assert_eq!(*rules.get_vec(&3).unwrap(), vec![4u32]);
        assert_eq!(page_data, vec![vec![1, 2, 3, 4, 50000], vec![9, 10, 11]]);
    }

    #[test]
    fn check_pages_simple() {
        let mut rules = MultiMap::new();
        rules.insert(1, 2);

        assert_eq!(check_pages(vec![1, 2], &rules), 1);
        assert_eq!(check_pages(vec![2, 1], &rules), 0);
    }

    #[test]
    fn check_pages_more() {
        let mut rules = MultiMap::new();
        rules.insert(1, 2);
        rules.insert(1, 3);
        rules.insert(2, 3);

        assert_eq!(check_pages(vec![1, 2, 3], &rules), 2);
        assert_eq!(check_pages(vec![1, 2, 4], &rules), 2);
        assert_eq!(check_pages(vec![1, 3, 4], &rules), 3);
        assert_eq!(check_pages(vec![2, 3, 4], &rules), 3);
        assert_eq!(check_pages(vec![1], &rules), 1);
        assert_eq!(check_pages(vec![2], &rules), 2);
        assert_eq!(check_pages(vec![3], &rules), 3);
        assert_eq!(check_pages(vec![4], &rules), 4);
        assert_eq!(check_pages(vec![1, 2, 3, 4, 5], &rules), 3);
        assert_eq!(check_pages(vec![4, 1, 2, 3, 5], &rules), 2);

        assert_eq!(check_pages(vec![3, 2, 1], &rules), 0);
        assert_eq!(check_pages(vec![3, 2, 4], &rules), 0);
        assert_eq!(check_pages(vec![3, 1, 4], &rules), 0);
        assert_eq!(check_pages(vec![2, 1, 4], &rules), 0);
    }

    #[test]
    fn middle_value_simple() {
        assert_eq!(middle_value(vec![1]), 1);
        assert_eq!(middle_value(vec![1, 2, 3]), 2);
    }
}
