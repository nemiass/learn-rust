fn main() {
    // Podemos ver que estamos usando el potencial de los bloques para asignar un valor
    // a la variable mensaje, osea tenemos 2 bloques que dependiendo si cumplen o no
    // la condición retoran un String y se guardan en la variable "calificaciones".
    let calificaciones: i8 = 10;

    let mensaje = if calificaciones == 10 {
        // El metodo "from" nos permite crear un string apartir de una cadena de caracteres
        String::from("Felicitaciones pasaste el curso!!")
    } else {
        String::from("Para el otro año será")
    };

    println!("{}", mensaje);

    // NOTA: recordar que los bloques tienen la capacidad de retornar valores, el valor que
    // se va retornar será la ultima sentencia ejecutada en el bloque, esa sentencia no debe 
    //poseer un "return" ni un ";".
}