fn dec_bin(mut num: isize) -> String {
    let mut bin: Vec<isize> = Vec::new();
    while num > 0 {
        let c = num / 2;
        let r = num % 2;
        num = c;
        bin.insert(0, r);
    }
    let bin: Vec<String> = bin.iter().map(|val| val.to_string()).collect();
    bin.join("")
}

fn main() {
    let num = 10;
    let res = dec_bin(num);
    println!("{}", res);
}
