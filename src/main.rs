use std::io;

fn average(arr: &[i32]) -> f32 {
    let sum: i32 = arr.iter().sum();
    let size = arr.len();
    sum as f32 / size as f32
}

fn main() {
    println!("Hello, world!");
    //0 1 2 3 4 5
    let arr =  [4, 3, 2, 1, 100, 50, 200];
    println!("My average is {}", average(&arr));
    
}
