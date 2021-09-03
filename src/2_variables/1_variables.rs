fn main() {
    // esto es un comentario de una linea
    /*
        este es un comentario multilinea
    */

    // esto es una form de definir variables, el compilador detecta el tipo de dato
    // que va poseer la variable
    let numero_uno = 10;
    let numero_dos = 10;
    let res = numero_uno + numero_dos;
    println!("El valor de la variabele es ({} + {}): {}",numero_uno, numero_dos, res);

    // esto es la forma mas formal en el cual nosotros le indicamos el tipo de dato que 
    // va poseer
    let a: i64 = 10;
    let b: i64 = 5;
    println!("Mostrando las variables del modo formal ({} + {}) = {}", a, b, a + b);

    // Las variables en rust POR DEFECTO SON INMUTABLES, osea hacer:
    // a = 20, esto dará un error
    // * para hacer que una variable pueda cambiar de valor en tiempo de ejecucion
    // DEBEMOS DECLARAR QUE SEAN MUTABLES
    let mut m: i64 = 6;
    m = 7;
    println!("Mostarando la variable mutable cambiada: {}", m);
    // vemos que al ejecutar nos mostrará un warning, pero ignorar por el momento
}
