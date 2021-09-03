fn main() {
    // BLOQUES PARTE 2
    // - Los bloques tienen la capacidad de retornar valores

    // De esta forma decimos que resultado será igual a lo que el bloque nos retorne, 
    // para ello debemos colocar ";" al final del bloque.
    let resultado = {
        println!("Hola nos encontramos en un bloque anidado!!");

        let variable: i32 = 200;
        println!("Variable, {}", variable);

        // -Al hacer esto, hacemos que se retorne el valor de la variable "variable",
        // no se retorna la variable solo su valor.
        variable
        // y de esta forma es como podemos retornar el valor apartir de un bloque, no es
        // necesario colocar ";" ni "return" vasta con que se ejecute la ultima sentencia del
        // bloque.
    };

    println!("El valor de resultado es: {}", resultado);

}