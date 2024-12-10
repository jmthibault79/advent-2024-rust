use super::string_iter;

fn to_char_vec(s: String) -> Vec<char> {
    s.chars().collect()
}

pub fn as_matrix(path: &str) -> Vec<Vec<char>> {
    string_iter(path).map(to_char_vec).collect()
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
