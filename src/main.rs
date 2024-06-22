//This code is for automating Gaussian Elimination for solving linear equations.

use matrix_display::{cell, matrix, Format, MatrixDisplay};

fn print_matrix(matrix: &Vec<Vec<f64>>, n: usize) {
    let format = Format::new(7, 3);

    let board = matrix.iter()
        .flat_map(|row| row.iter())
        .enumerate()
        .map(|(i, &val)| cell::Cell::new(val, 7, 240))
        .collect::<Vec<_>>();
    let mut data = matrix::Matrix::new(n+1, board);
    let display = MatrixDisplay::new(&format, &mut data);
    display.print(&mut std::io::stdout(), &matrix_display::style::BordersStyle::Light);
}

fn swap_rows(matrix: &mut Vec<Vec<f64>>, i: usize, j: usize){
    let temp = matrix[i].clone();
    matrix[i] = matrix[j].clone();
    matrix[j] = temp;
}

///Check if the augmented matrix is upper triangular
fn check_if_mat_is_ut(matrix: &Vec<Vec<f64>>, n: usize) -> bool{
    for i in 0..n{
        for j in 0..i{
            if matrix[i][j] != 0.0{
                return false;
            }
        }
    }
    return true;
}


fn main() {
    println!("Hello! This is a code to solve Gaussian Elimination in a Square system (i.e. the number of equations equal the number of variables to be solved for).\n This code will handle the problems encountered during the formation of the Upper Triangle Matrix and will notify of a singular case.");
    println!("Enter the number of unknown variables: ");
    let n: usize;
    
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    n = input.trim().parse().unwrap();



    let mut coefficient_matrix: Vec<Vec<f64>> = Default::default();

    //Matrix input
    for i in 0..n{
        let mut temp: Vec<f64> = Default::default();
        let mut val: f64;
        for j in 0..n{
            input.clear();
            println!("Enter the matrix element at Mat[{}][{}]", i,j);            
            std::io::stdin().read_line(&mut input).unwrap();
            val = input.trim().parse().unwrap();
            temp.push(val);
        }
        input.clear();
        println!("Enter RHS of the equation {}",i);
        std::io::stdin().read_line(&mut input).unwrap();
        temp.push(input.trim().parse().unwrap());

        coefficient_matrix.push(temp);       
        
    }


    //Example Matrix to avoid inputting everytime:
    // coefficient_matrix = vec![vec![2.0, 1.0, -1.0, 8.0], vec![-3.0, -1.0, 2.0, -11.0], vec![-2.0, 1.0, 2.0, -3.0]];
    // let n=3;


    println!("The matrix you entered is: ");
    print_matrix(&coefficient_matrix, n);


    //Gaussian Elimination:

    //Ideal case
    let mut i = 0;
    while i<n {
        let pivot = coefficient_matrix[i][i];
        if pivot == 0.0{
            //TODO: Row exchange brings up NaNs, fix it!
            for j in i+1..n{
                if coefficient_matrix[j][i] != 0.0{
                    swap_rows(&mut coefficient_matrix, i, j);
                    break;
                }
            }
            println!("Pivot was zero, so row exchange was done. New pivot: {}", coefficient_matrix[i][i]);
            print_matrix(&coefficient_matrix, n);
            //The row is exchanged, so try again
            i-=1;

        }
        for j in i+1..n{
            let ratio = coefficient_matrix[j][i]/pivot;
            for k in 0..n+1{
                coefficient_matrix[j][k] = coefficient_matrix[j][k] - ratio*coefficient_matrix[i][k];
            }
        }
        i+=1;
        print_matrix(&coefficient_matrix, n);
        if check_if_mat_is_ut(&coefficient_matrix, n){
            println!("The matrix is upper triangular now.");
            break;
        }
    }

    

    //Back substitution:
    let mut solutions: Vec<f64> = vec![0.;n];

    for i in (0..n).rev(){
        let rhs_only: f64 = (i..n).map(|j| coefficient_matrix[i][j]*solutions[j]).collect::<Vec<f64>>().iter().sum();
        solutions[i] = (coefficient_matrix[i][n] - rhs_only)/coefficient_matrix[i][i];           
    }

    println!("The solutions are: ");
    let start_var: char = 'a';
    for i in 0..n{
        println!("{} = {}", (start_var as u8 + i as u8) as char, solutions[i]);
    }

    
    


}
