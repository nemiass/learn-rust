// Creando la structura "User", ademas le definimos que la propiedad "edad" va ser
// opcional, osea "edad" puede o no almacenar un valor.
#[derive(Debug)]
struct User {
    username: String,
    password: String,
    correo: String,
    edad: Option<usize>,
}

fn main() {
    // Implementando el "enum Option" en las estructuras
    let usuario1 = User {
        username: String::from("nemias"),
        password: String::from("1234"),
        correo: String::from("nehemiastf@gmail.com"),
        // como edad es un tipo Option, y si le vamos a meter dato a edad, lo hacemos
        // con Some()
        edad: Some(21),
    };
    println!("El usuario es: {:?}", usuario1);
    // para acceder a la variable edad podemos usar "match" u otros metodos que ya vimos
    let edad = usuario1.edad.unwrap();
    println!("Mostrando la edad del usuario: {}", edad);

    // Ejenmplo cuando un usuario no tiene un valor en edad.
    let usuario2 = User {
        username: String::from("cristina"),
        password: String::from("1111"),
        correo: String::from("cris@gmail.com"),
        // En este ejemplo no meteremos nigun dato a edad, osea ene ste caso el user
        // no tiene edad
        edad: None,
    };
    // ene ste caso usaremos "match" para evaluar la edad del usuario
    match usuario2.edad {
        Some(edad) => println!("Su edad es: {}", edad),
        // si no tiene edad, en este caso no hacemos nada
        None => {}
    }
}
// NOTA: Recordar que el "enum Option" nos permite validar si existe o no
// ulgun valor, si este valor existe ese se almacenará en la tupla "Some()",
// en caso constrario haremos uso de "None"
