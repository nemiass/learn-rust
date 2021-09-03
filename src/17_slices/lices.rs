fn main() {
    // SLICE
    // * so similares a un arreglo, la diferencia ente ellos es que los slices
    // no se les conoce el tamaño en tiempo de ejecucion, es decir los slices
    // son almacenados en el HEAP
    // * Los slices nos permiten prestar una seccion de un arreglo
    // por ejemplo de un string:
    let mensaje = String::from("Hola mundo slices");
    println!("el mensaje es: {}", mensaje);

    // Generando slices
    // hacemos un prestamo de nuestro string
    let hola = &mensaje[0..4]; // [start..end]
    // de esta forma le indicamos a rust que queremos tomar desde el indice 4 hasta tomar todo
    let resto_mensaje = &mensaje[4..];
    // tambien podemos tener la procion completa de nuestro mensaje
    let mensaje_completo = &mensaje[..];
    println!("El slice es: {}", hola);
    println!("El slice resto es: {}", resto_mensaje);
    println!("El slice completo es: {}", mensaje_completo);

    // NOTA: un slice es un prestamo de una seccion de un arrelo, recordar
    // que un String es un areglo de caracteres
}
