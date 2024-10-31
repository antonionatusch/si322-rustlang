#![allow(unused_variables, dead_code)]

fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut tranposed_matrix: [[i32; 3]; 3] = matrix;

    (0..3).for_each(|row| {
        (0..3).for_each(|column| {
            tranposed_matrix[row][column] = matrix[column][row];
        });
    });

    tranposed_matrix
}

fn pretty_print(matrix: &[[i32; 3]; 3]) {
    (0..3).for_each(|i| {
        (0..3).for_each(|j| {
            print!(" {}", matrix[i][j]);
        });
        println!();
    });
}

fn main() {
    let matrix = [[101, 102, 103], [201, 202, 203], [301, 302, 303]];
    println!("Matriz: ");
    pretty_print(&matrix);
    let transposed = transpose(matrix);
    println!("Matriz transpuesta: ");
    pretty_print(&transposed);
}
