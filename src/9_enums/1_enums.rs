fn main() {
    // El "Enum" junto con el match, son unos de los features mas poderosos que posee
    // el lenguaje de programacion Rust

    // ENUM
    // Es un tipo el cual almacena diferentes variantes, almacena diferentes opciones
    // Creando nuestro propio "enum" -> por convecion se usa CamelCase

    enum Response {
        Success,
        // indicamos que Error va ser una tupla de dos elementos
        Error(u32, String), // 403, 404, 500
    }
    // para aceder a la opción de un "enum" se usa los "::"
    let respuesta = Response::Error(600, String::from("No es posible conpletar la operación"));

    // Combinando "enum" con "match"
    match respuesta {
        Response::Success => println!("La petición se realizó exitosamente"),
        Response::Error(403, _) => println!("Forbidden"),
        Response::Error(404, _) => println!("Not Found"),
        Response::Error(500, _) => println!("Internal server Error"),
        // este sería del default por si el error posee otro código diferente a los anteriores
        Response::Error(_, mensaje) => println!("{}", mensaje)
    }
}
