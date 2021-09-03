
// Por conveción las funciones debem escribirse al estilo "snake_case"

fn saludar_usuarios() {
    // este e suna función sin parametros
    println!("Hola, desde la función saludar_usuario");
}

fn suma(a: i64, b: i64) -> i64 {
    // para retornar algo en un bloque, basta con colocar "a + b", no es necesario
    // ";" ni la palabra reservada "return"
    a + b
}


fn main() {
    saludar_usuarios();

    let res = suma(5, 5);
    println!("El resultado es: {}", res);
}