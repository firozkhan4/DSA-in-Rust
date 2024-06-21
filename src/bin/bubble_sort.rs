/*
    Steps of Bubble Sort:

    - Compare the first two adjacent elements of the list.
    - Swap them if they are in the wrong order (i.e., the first element is greater than the second).
    - Move to the next pair of elements and repeat the comparison and swap if needed.
    - Continue this process until the end of the list is reached.
    - Repeat the entire process for the remaining elements, reducing the range of comparison by one each time (since the largest element will be in its correct place after each pass).

*/

fn bubble_sort(vac: &mut [i32]) {
    let n = vac.len();

    for i in 0..n {
        let mut flag = true;
        for j in 0..n - i - 1 {
            if vac[j] > vac[j + 1] {
                vac.swap(j, j + 1);
                flag = false;
            }
            if flag {
                break;
            }
        }
    }
}

fn main() {
    let mut vec = [];
    bubble_sort(&mut vec);
    println!("{:?}", vec);
}
