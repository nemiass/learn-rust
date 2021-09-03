fn main() {
    // SHADOWING:
    // Concepto el cual dicta que podemos utilizar el nombre de una variable la "n"
    // cantidad de veces que necesitemos
    let valor: i64 = 15;
    println!("El valor de la variable es {}", valor);

    // Si queremos cambiar el el valor de nuestra variable "valor", lo que se haría es 
    // declararlo como mutable, pero ¿si queremos cambiar el valor de la variable sin 
    // hacerlo mitable??, en este caso cimplemente lo redeclaramos
    let valor = 12;
    println!("El valor de la variable es {}", valor);

    // EXPLICACIÓN:
    // podriamos pensar que estamos cambiando el dato de la variable "valor" definida primero,
    // pero no es así, lo que está ocurriendo es que estamos creando una nueva variable
    // reutilizando el nombre, a esto se le llama "shadowing", y lo que le pasa a la variable 
    // declarada primeramente es que se destruye. 
    // - Lo interesante es que podemos definir una nueva variable 
    //   con un tipo completamente nuevo, aunque estamos creando nuevas variables, estas siguen
    //   siendo inmutables.
    let valor: bool = true;
    println!("El valor de la variable es: {}", valor);
} 