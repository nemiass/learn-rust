fn main() {
    let mut cont = 1;

    while cont <= 10 {
        println!("Contador: {}", cont);
        cont += 1;
    }

    // El cilco "while" lo vamos a utlizar siempre y cuando no conozcamos cuantas
    // iteraciones se van a arealizar.
    // Ejemplo: programa que cuenta la cantidad de digitos de un nuemro, 548 -> 3
    let mut numero = 123;
    let mut cont = 0;

    while numero > 0 {
        numero = numero / 10;
        cont += 1;
    }
    println!("La cantidad de digitos son {}", cont);
}