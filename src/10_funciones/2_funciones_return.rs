fn suma(a: i64, b: i64) -> i64 {
    a + b
}

fn factorial(numero: u32) -> u32 {
    if numero == 1 {
        // en este caso debemos poner return porque es el caso base de la recursión
        // osea la función debe terminar retornando 1, ademas colocamos return poque
        // este no es la ultima linea de ejecucion del bloque de la función.
        return numero;
    }
    // en este caso se devuelve factorial sin necesidad de colocar ";" ni la palabra "return"
    factorial(numero - 1) * numero
}

fn factorial2(numero: i64) -> i64 {
    // De esta forma podemos manejar los bloques, y no necesitamos colocar el "return",
    // el resultado de este bloque se va retorna en la función
    if numero == 1 {
        numero
    } else {
        factorial2(numero - 1) * numero
    }
}

fn main() {
    // Como ya es explicado anteriormente un bloque tiene la capacida de retornar un
    // valor, el valor a retornar es el resultado de la ejecución de la última linea
    // de codigo dentro del bloque, como vemos con el ejemplo "suma()"
    let res = suma(10, 200);
    println!("Elresultado de la suma es: {}", res);

    // Pero que psas si una función tiene que retornar un valor antes de tiempo, osea
    // antes de llegar a la ultima linea de ejecución, para ese caso se debe usar
    // la palabra reservada return, como se ve en el ejemplo de "factorial()"
    let facto = factorial(5);
    println!("El factorial es: {}", facto);

    // Refactorizando la funcion "factorial" sería de la siguiente forma
    let facto2 = factorial2(5);
    println!("El factorial es: {}", facto2);

    // NOTA: podemos usar return cuando queremos retornar un dato en una función antes
    // de tiempo, osea antes de llegar al retorno automatico en la ultima linea del bloque.

}
