// para leer lo que el usuario ingrese por teclado hacemos uso de esta librería
use std::io;

fn main() {
    println!("Ingresa el nombre de usuario:");
    // declarando una variable string vacío, String::new(), estamos ejecutando su metodo
    // "estático" "new()", este retorna un string vacío
    let mut username: String = String::new();

    // Prestamos username por "referencia" para ello colocamos "&", este prestamo es con
    // permisos de mutabilidad "&mut", osea este no solo tiene el permiso de leer username
    // sino que tambien tiene permiso de modificarlo
    // - de esta forma le indicamos a rust que estamos prestando una variable y que la queremos
    // de vuelta.
    // - "read_line()", retorna un result, esta estructura puede poseer 2 valores, uno de exito 
    // o uno de error, "esto lo veremos mas adelante"
    io::stdin().read_line(&mut username);

    // Usamos shadowing, creamos una nueva variable para quitar salto de linea final, usamos
    // "trim()" que quita los espacios el inicio y final de una cadena
    let username = username.trim();
    println!("Tu nombre es: {}", username);

    // Tambien recibiremos el dato de la edad
    let mut edad = String::new();

    io::stdin().read_line(&mut edad);

    // limpiamos el string edad
    let edad = edad.trim();
    // El metodo parse tambien retorna dos estados, exito o error, hacemos uso del metodo
    // "unwrap()" para obtener el caso de exito.
    // - Para tenet el dato ya parseado, en este caso el compilador ya intuye
    // que el entero en que se parseará será de 64bits porque la variable en que se 
    // guardará el parse será un entero de 64bits
    let edad: i64 = edad.parse().unwrap();
    println!("Tu edad es: {}", edad);
}
