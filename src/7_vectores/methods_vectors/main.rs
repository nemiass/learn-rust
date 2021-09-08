// fn vecc(mut v: &Vec<i32>) -> &Vec<i32> {
//     &v.push(1000);
//     v
// }


fn main() {
    let mut a = vec![1, 2, 3];
    let b = vec![4, 5, 6];

    a.extend(&b);
    println!("{:?}", a);
    println!("{:?}", b);
}
