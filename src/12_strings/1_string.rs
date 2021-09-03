fn main() {
    // Un string es una cadena de caracteres, y en Rust existen dos formas
    // de representar dichas cadenas.

    // Podemos definirlas haciendo uso del tipo de dato "str" o la estructura "String".
    // str -> es una cadena de carcateres inmutable.
    //  el str se almacena en el STACK
    // String -> es una cadena mutable, podemos agregar o quitar caracteres.
    //  el String se almacena en el HEAP

    // Ejemplos
    let variable_str = "Hola, soy un tipo str";

    // usamos "new()" para crear una cadena vacía, y "from()" para generar un string a partir de
    // una cadena.
    let mut variable_string = String::from("Hola soy un String");

    println!("El str es: {}", variable_str);
    println!("El String es : {}", variable_string);

    // Podemos aprovechar las ventajas de nuestra variable String
    // haciendo push() para agregar caracter por caracter.
    variable_string.push(' ');
    variable_string.push('X');
    variable_string.push('D');
    variable_string.push(' ');

    // tambien podemos agregar un str mas grande
    variable_string.push_str(",Agregando un str.");

    println!("Mostrando a la variable_string: {}", variable_string);

    // Tambien podemos convertir un cadena de tipo "str" a "String", usando el metodo
    // "to_string()" de los str, ejemplo:
    let nuevo_string = "Hola, soy la nueva cadena".to_string();
    println!("Mostrando al nueco string: {}", nuevo_string);

    // tambien podemos comparar strings
    let igual = nuevo_string == "Hola, soy la nueva cadena.".to_string();
    println!("Los strings son iguales??: {}", igual);
}