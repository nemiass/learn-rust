fn main() {
    // Inmutable no es lo mismo que Constante
    // Hay dos formas de declarar variables constantes en rust
    /**
     * Una forma de declarar una constante es anteponiendo la palabra "static",
     * pero por ahora usaremos "const", mas adelante se verá lo de "sratic"
     */
    // - Por convencion se coloca el nombre de la constante en mayuscula, además es
    // obligatorio definir el tipo de dato
    const PI: i64 = 13;
    let num_1: i64 = 10;
    let num_2: i64 = 30;
    let res: i64 = num_1 + num_2 + PI;
    println!("La suma de los numeros ({} + {} + {}) es: {}", num_1, num_2, PI, res);
}