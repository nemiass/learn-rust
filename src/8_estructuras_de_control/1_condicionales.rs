use std::io;

fn main() {
    let mut color = String::new();

    println!("Ingrese un color para el semáforo:");
    // dentro de read_line() indicamos que estamos prestando nuestra variable por
    // referencia y el prestamo incluye mutabilidad, osea que res_line() puede modificarlo
    // de un String vacío a lo que el usuario ingrese por teclado.
    io::stdin().read_line(&mut color);

    // eliminadno el salto de linea de la entrada de teclado al precionar enter, y 
    // colocando todo a minusculas
    let color = color.trim().to_lowercase();

    if color == "verde" {
        println!("Puede pasar");
    } else if color == "amarillo" {
        println!("Alto parcial");
    } else if color == "rojo" {
        println!("Detengase el semáforo está en rojo");
    } else {
        println!("Color no valido");
    }
}
