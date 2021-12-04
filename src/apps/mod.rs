mod fibonacci_word;
mod h_tree;
mod mandelbrot;
mod sierpinski;

/*
Using Rust's module system to split up the fractal code into individual files.
*/

pub use fibonacci_word::FibonacciWord;
pub use h_tree::HTree;
pub use mandelbrot::Mandelbrot;
pub use sierpinski::SierpinskiCarpet;
