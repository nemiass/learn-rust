

fn show(arr: &[i32]) {
    for v in arr.iter() {
        println!("val: {}", v);
    }
}

fn show_two(arr: &[&[i32]]) {
    for a in arr.iter() {
        for v in a.iter() {
            print!("{} ", v);
        }
        println!();
    }
}

fn main() {
    let arr = [10, 15, 20, 25, 30];
    show(&arr);

    let arr_bi = [[10, 20], [80, 90]];
    show_two(&arr_bi);
}