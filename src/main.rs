fn max_and_min(vec: &[i32]) -> i32 {
    let mut max = vec[0];
    let mut min = vec[0];

    for &num in vec.iter() {
        if num > max {
            max = num;
        }
        if num < min {
            min = num;
        }
    }

    max - min
}

fn main() {
    let vec = vec![1, 2, 3, 4, 5];
    println!("{}", max_and_min(&vec));
}
