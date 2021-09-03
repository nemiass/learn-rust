fn main() {
    // Ciclo de vida de las variables
    // Un bloque limita el alcance de una variables, osea la variable "mensaje"
    // puede ser utilizada unicamente dentro del bloque donde fue definida, tambien
    // en bloques hijos.
    let mensaje = "soy una variable del bloque main";
    println!("Bloque 1: {}", mensaje);
    {
        // cuando colocamos una variable con el mismo nombre en un bloque hijo, este
        // ya no hará uso de la variable "mensaje" del bloque superior, porque esta
        // variable está siendo definida dentro de este bloques
        let mensaje = "soy una variable del bloque 2";
        println!("Bloque 2: {}", mensaje);
        {
            // En este caso este bloque 3, toma la variable del bloque superior, y
            // si no lo encuentra, sube al siguiente bloque, en este caso toma la variable
            // del bloque 2
            println!("Bloque 3: {}", mensaje);

            // esta variable del bloque 3, no puede ser usado fuera de este bloque, osea
            // bloique 2 y bloque 1 no pueden hacer uso de ellas, ejemplo de "resultado"
            // - cuando finaliza el bloque 3, todo dentro de el se destruye
            let resultado = 5 + 5;
        }
    }
}

// El cliclo de vida de una variable depende del alcance, el alcance depende del bloque en que
// se encuentre una variable.