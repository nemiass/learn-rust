fn main() {
    // Ejemplo de implementacion de "Some" junto "match", con la finalidad
    // que podamso sacarle el maximo provecho.
    let mensaje = Some("Hola mundo");

    // ene ste caso estamos validando los dierentes valores que puede o no tomar
    // "Some", en este caso sobre os casos, "Hola mundo" y "Adios", este es una 
    // for ade sacarle provecho a Some + Match
    match mensaje {
        Some("Hola mundo") => println!("El mensaje es Hola mundo"),
        Some("Adios") => println!("El mensaje es Adios"),
        Some(_) => println!("Es otro mensaje"),
        None => println!("No existe valor alguno"),
    }
}
