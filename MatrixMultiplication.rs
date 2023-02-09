/**
* Write a method to read two integer matrices from StdIn and print
* their product matrix.
*/
use std::io;

pub struct Matrix {
    name: String,
    rows: i32,
    cols: i32,
    matrix: Vec<Vec<f32>>,
}

impl Matrix {
    pub fn set_matrix(&mut self) -> &mut Matrix {
        println!("Matrix {} rows: ", self.name);
        let mut input_line: String = String::new();
        io::stdin()
            .read_line(&mut input_line)
            .expect("Failed to read line");
        let rows: i32 = input_line.trim().parse().expect("Input not an integer");
        println!("Matrix {} cols: ", self.name);
        let mut input_line: String = String::new();
        io::stdin()
            .read_line(&mut input_line)
            .expect("Failed to read line");
        let cols: i32 = input_line.trim().parse().expect("Input not an integer");
        println!(
            "Space separated Matrix {} entries (e.g. '1 2 3'):  ",
            self.name
        );
        let mut input_line: String = String::new();
        io::stdin()
            .read_line(&mut input_line)
            .expect("Failed to read line");
        let mat_raw: Vec<f32> = input_line
            .trim()
            .split(' ')
            .collect::<Vec<&str>>()
            .iter()
            .map(|n| n.parse().expect("Input not an integer"))
            .collect();
        let mut mat: Vec<Vec<f32>> = Vec::new();
        for i in 0..rows {
            let mut row_temp: Vec<f32> = Vec::new();
            for j in 0..cols {
                row_temp.push(mat_raw[(i * cols + j) as usize]);
            }
            mat.push(row_temp);
        }
        self.rows = rows;
        self.cols = cols;
        self.matrix = mat;
        return self;
    }
}

fn matrix_multiplication(mat_a: &Matrix, mat_b: &Matrix) -> Vec<Vec<f32>> {
    /**
     * Function multiplies two matrices given col_A == rows_B.
     *
     * Args:
     *     mat_a (matrix): Matrix A.
     *     mat_b (matrix): Matrix B.
     *
     * Raises:
     *     None.
     *
     * Returns:
     *     prod_mat (matrix): Product of Matrix A and Matrix B.
     */
    let mut prod_mat: Vec<Vec<f32>> = Vec::new();
    for i in 0..mat_a.rows {
        let mut row_temp: Vec<f32> = Vec::new();
        for j in 0..mat_b.cols {
            let mut prod_mat_ij: f32 = 0.0;
            for k in 0..mat_a.cols {
                prod_mat_ij +=
                    mat_a.matrix[i as usize][k as usize] * mat_b.matrix[k as usize][j as usize];
            }
            row_temp.push(prod_mat_ij as f32);
        }
        prod_mat.push(row_temp);
    }
    return prod_mat;
}

fn main() {
    let mut mat_a = Matrix {
        name: String::from("A"),
        rows: 0,
        cols: 0,
        matrix: Vec::new(),
    };
    mat_a.set_matrix();
    println!("Matrix {}: ", mat_a.name);
    for i in 0..mat_a.rows {
        for j in 0..mat_a.cols {
            print!("{} ", mat_a.matrix[i as usize][j as usize]);
        }
        println!("{}", "");
    }
    let mut mat_b = Matrix {
        name: String::from("B"),
        rows: 0,
        cols: 0,
        matrix: Vec::new(),
    };
    mat_b.set_matrix();
    println!("Matrix {}: ", mat_b.name);
    for i in 0..mat_b.rows {
        for j in 0..mat_b.cols {
            print!("{} ", mat_b.matrix[i as usize][j as usize]);
        }
        println!("{}", "");
    }
    if mat_a.cols == mat_b.rows {
        println!("The product Matrix is: ");
        let prod_mat: Vec<Vec<f32>> = matrix_multiplication(&mat_a, &mat_b);
        for i in 0..mat_a.rows {
            for j in 0..mat_b.cols {
                print!("{} ", prod_mat[i as usize][j as usize]);
            }
            println!("{}", "");
        }
    } else {
        println!("{}", "Matrices can't be multiplied!");
    }
}
