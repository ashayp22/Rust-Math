mod fibonacci_word;
mod h_tree;
mod sierpinski;
mod mandelbrot;

/*
Using Rust's module system to split up the fractal code into individual files.
*/

pub use fibonacci_word::FibonacciWord;
pub use h_tree::HTree;
pub use sierpinski::SierpinskiCarpet;
pub use mandelbrot::Mandelbrot;
