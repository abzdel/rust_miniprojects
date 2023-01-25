// generate a random vector of costs to be used by a gradient descent algorithm

use rand::Rng;

fn main() {
    // rng is a random number generator
    let mut rng = rand::thread_rng();

    // generate a vector of 100 random costs
    let mut costs: Vec<f64> = Vec::new();
    for _ in 0..100 {
        costs.push(rng.gen_range(0.0, 100.0));
    }
    println!("{:?}", costs);
}

// implement gradient descent to find the minimum cost from our generated vector

// Path: week1/src/main.rs
// implement gradient descent to find the minimum cost from our generated vector


fn main() {
    // rng is a random number generator
    let mut rng = rand::thread_rng();

    // generate a vector of 100 random costs
    let mut costs: Vec<f64> = Vec::new();
    for _ in 0..100 {
        costs.push(rng.gen_range(0.0, 100.0));
    }
    println!("{:?}", costs);

    // initialize the learning rate and the number of iterations
    let learning_rate = 0.01;
    let iterations = 1000;

    // initialize the parameters
    let mut m = 0.0;
    let mut b = 0.0;

    // perform gradient descent
    for _ in 0..iterations {
        let (dm, db) = gradient(&costs, m, b);
        m -= learning_rate * dm;
        b -= learning_rate * db;
    }

    // print the final parameters
    println!("m: {}, b: {}", m, b);
}
