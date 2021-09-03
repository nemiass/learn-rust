
// Definiendo nustra estructura tipo tupla
// de esta froma creamos una estructura de tipo tupla donde no definimos atributos
// sino que definimos tipos de datos.
#[derive(Debug)]
struct Color(u32, u32, u32); // RGB

fn main() {
    // ESTRUCTURA TIPO TUPLAS
    // Este tipo de estructura en lugar de poseer atributos poseerá valores

    let black = Color(0, 0, 0);
    let white = Color(255, 255, 255);

    println!("El color es: {:?}", black);
    println!("El color es: {:?}", white);

    // Al tratarde de una Estructura de tipo tupla, podemos tratar a nuestros objetos
    // como una tupla
    let mut color_personalizable = Color(187, 62, 104);
    color_personalizable.1 = color_personalizable.1 + 10;
    println!("Mostrando mi color personalizado: {:?}", color_personalizable);
}
