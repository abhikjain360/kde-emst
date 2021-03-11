mod emst;
mod kde;
use kde::*;

fn main() {
    let mean = [0f32; 3];
    let cov = [[2f32, -1.0, 0.0], [-1.0, 2.0, -1.0], [0.0, -1.0, 2.0]];
    let x = [0f32; 3];
    let data = vec![[0f32; 3]];
    let z = kde(1.0, &x, &data, gaussian_kernel(mean, cov));
    println!("main_x = {}", z);
}
