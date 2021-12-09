fn sum_extremos(arr: &[isize]) {
    let mut i = 0;
    let mut j = arr.len() - 1;

    while i <= j {
        let suma = if i != j {arr[i] + arr[j]} else {arr[i]};
        i += 1;
        j -= 1;
        println!("{}", suma);
    }
}

fn main() {
    let arr = [2, 4, 5, 1, 2];
    sum_extremos(&arr);
}
