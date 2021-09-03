fn main() {
    // El "Match" no está limitado unicamente a evaluar sobre numeros enteros,
    // de hecho se puede evaluar sobre estructuras mucho mas complejas.

    // REALIZANDO FIZZ BUZZ CON MATCH

    for numero in 1..31 {
        // ya no estamos evaluando sobre un numero, sino sobre una tupla de 2 elementos
        match (numero % 3, numero % 5) {
            (0, 0) => println!("Fizz Buzz"),
            (0, _) => println!("Fizz"),
            (_, 0) => println!("Buzz"),
            _ => println!("{}", numero)
        }
    }
}