fn main() {
    let strs = vec!["catana", "catman", "caraoke"];

    let mut target: Vec<char> = strs[0].chars().collect();
    let point = 0;
    println!("{}", target[0]);
    println!("{:?}", strs);
}
