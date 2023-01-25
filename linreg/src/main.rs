use rand::Rng;
//use linreg::{linear_regression};
use smartcore::linear::linear_regression::*;


fn number_generator() -> (Vec<f64>, Vec<f64>){
    // thread_rng to create different numbers each time
    let mut rng = rand::thread_rng();

    // define x and y to be vectors of f64
    let mut x: Vec<f64> = Vec::new();
    let mut y: Vec<f64> = Vec::new();

    // fill in x and y
    for _ in 0..100 {
        x.push(rng.gen_range(0.0, 100.0));
        y.push(rng.gen_range(0.0, 100.0));
    }
    return (x, y);
}

// function to calculate the average of a vector
fn average(vector: Vec<f64>) -> f64 {
    let mut sum = 0.0;
    for i in 0..vector.len() {
        sum += vector[i];
    }
    return sum / vector.len() as f64;
}

// main function to run linear regression on our random vectors
fn main() {
    let (x, y) = number_generator();

    let lr = LinearRegression::fit(&x, &y,
        LinearRegressionParameters::default().
        with_solver(LinearRegressionSolverName::QR)).unwrap();

    let y_hat = lr.predict(&x).unwrap();
}