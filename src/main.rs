
fn average(arr: &[i32]) -> i32 {
    let mut sum: i32 = 0;
    for number in arr {
        sum = sum + number;
    }
    let size: i32 = arr.len() as i32;
    sum / size
}

fn main() {
    println!("Hello, world!");
    //0 1 2 3 4 5
    let arr: [i32; 5] =  [4, 3, 2, 1, 100];
    println!("My average is {}", average(&arr));
    
}
