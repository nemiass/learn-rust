fn main() {
    // Ejemplo de cilo de vida en PRESTAMOS
    let mensaje = String::from("Hola soy una variable de prestamo");
    let mut mensaje2 = String::from("Hola soy una varibla de prestamo 2");
    {
        // al hacer el prestamo la variable "mensaje" se mueve a la variable "prestamo",
        // osea ahora esa variable solo existe dentro del scope de este bloque
        let prestamo = mensaje;
        // Para evitar que suceda lo anterior, lo que se puede hacer es hacer un
        // prestamo por referencia.
        let perstamo2 = &mensaje2;

        // En este caso la variable "mensaje2" se está prestando a la variable "prestamo2",
        //por todo ello la variable "mensaje2" no puede ser usada para modificarse, osea
        // queda congelada hasta que se libere, osea hasta que se use "prestamo2" que
        // es el que hizo el prestamo, ose si hacemos:
        // var mensaje2 = String::from("xd") -> daria un error
        println!("Mostrando prestamo2: {}", perstamo2); // freezing de mensaje2
        // esto ya no daría error porque al usar la variable "prestamo2" que hizo el prestamo
        // mensaje2 se descongela, por el cual podemos hacer uso de ella, analizar mas
        // a fondo esto en rust, **osea no puedo usar mi variable "mensaje2" cuando despues de el
        // se va a usar la variable que hizo un prestamo de "mensaje2", amenos que el que hizo
        // el prestamo nunca se use.
        mensaje2 = String::from("xd");
        println!("{}", mensaje2);
    }
    // ejecutar esto dará error, ya que la varibla se movió al escope de el bloque anidado
    //println!("Haciendo uso de la variable mensaje: {}", mensaje);

    // ahora si podemos usar la variable "mensaje2" ya que este se prestó al bloque
    // anidado mediante referencia, osea cuando el bloque anidado finaliza este devuelve
    // la variable "mensaje2".
    println!("Mostrando la variable mensaje2: {}", mensaje2);
}

// NOTA:
// * El ownership nos indica las reglas de las referencias, como que slo puede hacer una 
// referencia para un objeto, y, el ciclo de vida que una vez el bloque finaliza todas
// las variables que se hayan declarado en este serán destruidas.
// * Por todo ello es importante saber cuando vamos a mover una variable y cuando vamos a
// hacer un prestamo por referencia.