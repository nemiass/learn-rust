fn es_primo(n: isize) -> bool {
    for i in 2..n {
        if i % 2 == 0 {
            return false
        }
    }
    true
}

fn main() {
    let number = 11;
    println!("{}", es_primo(number));
}
