fn insertion_sort(vac: &mut [i32]) {
    let n = vac.len();

    for i in 1..n {
        let mut j = i;
        while j > 0 && vac[j - 1] > vac[j] {
            vac.swap(j - 1, j);
            j -= 1;
        }
    }
}

fn main() {
    let mut vec = [];
    insertion_sort(&mut vec);
    println!("{:?}", vec);
}
