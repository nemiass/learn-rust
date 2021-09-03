fn main() {
    // Loop
    // -apartiur de loop seremos capaz de generar un ciclo infinito, ejemplo

    let mut cont = 0;
    loop {
        cont += 1;
        println!("Hola nos encontramos dentro de un ciclo infinito, {}", cont);

        // con break podemos terminar el cilo de forma abrupta.
        if cont > 10 {
            break;
        }
    }
}