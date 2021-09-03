fn main() {
    // For
    // en Rust, el ciclo for nos permitirá iterar sobrea una colección de datos,
    // ya sean un arreglo, tupla o vector, etc., este funcionará como un foreach

    let numeros: [i64; 5] = [2, 4, 6, 8, 10];

    // al arerglo debemos ejecutarle el metodo "iter()", el cual devuelve una estructura
    // iterable
    for numero in numeros.iter() {
        println!("El valor de numero: {}", numero);
    }

    // también podemos generar un rango
    for i in 1..10 {
        println!("{}", i);
    }
}