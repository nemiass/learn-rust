use std::{fs, fs::File, io::Write};

fn write_f() {
    let c = "Holaaaaaaaaaaaaaaa";
    let mut mi_archivo = File::create("hola2.txt").unwrap();
    mi_archivo.write_all(c.as_bytes()).unwrap();
}

fn main() {
    let content = fs::read_to_string("hola.txt").unwrap();
    println!("{}", content);
    write_f();
}
