use std::io;
use ndarray::{Array2, Axis};

fn main() {
    let mut input = read_line().unwrap();
    let size = input.len();
    let mut matrix: Array2<bool> = Array2::<bool>::default((size, size));
    let mut visited: Vec<bool> = vec![false; size];

    populate_matrix(&mut matrix, &input, 0);
    for i in 1..size {
        input = read_line().unwrap();
        populate_matrix(&mut matrix, &input, i);
    }

    let component_count = (0..size)
        .fold(0, |acc, root| acc + dfs(&mut matrix, &mut visited, root));

    println!("{}", component_count);
}

fn read_line() -> Result<String, io::Error> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).and_then(|_| Ok(input.trim_right().to_string()))
}

fn populate_matrix(matrix: &mut Array2<bool>, line: &str, from: usize) {
    for (to, character) in line.chars().enumerate() {
        match character {
            '1' => { matrix[[from, to]] = true }
            _ => { matrix[[from, to]] = false }
        };
    }
}

fn dfs(matrix: &Array2<bool>, visited: &mut Vec<bool>, root: usize) -> u32 {
    if visited[root] {
        return 0;
    }
    visited[root] = true;

    let col_count = matrix.len_of(Axis(1));

    for neighbor in 0..col_count {
        println!("n{}", neighbor);

        if matrix[[root, neighbor]] {
            dfs(matrix, visited, neighbor);
        }
    }
    return 1;
}

