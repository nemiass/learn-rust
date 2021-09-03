fn main() {
    let mensaje = "Hola, soy una variable en el bloque main";
    // BLOQUES
    // dfinición: Es una colección de sentencias el cual están agrupadas dentro de
    // un juego de llaves.

    // Las variables tienen un ciclo de vida qiue está relacionado con los bloques, osea estos se
    // crean, se utilizan, se modifican y se destruyen dentro de un bloque.
    // Cuando un bloque es finalizado todas las variables dentro de el se destruyen.

    // LO INTERESANTE de rust, es que podemos crear la "n" cantidad de bloques que
    // necesitemos, no limitandonos a utilizar ciclos, funciones o condiciones.
    // Ejempplo de creacion de un bloque: dentro de ella podemos colocar la n 
    // cantida de sentencias que queramos utilizar
    {
        println!("Hola desde el segundo bloque");
        println!("Usando a mensaje desde el bloque hijo, {}", mensaje);

        // Definiendo una variable en el bloque anidado, esta variable solo puede ser usado
        // en el ambito de este bloque y en los bloques anidados que haya acá dentro.
        let mensaje_2 = "Hola soy una variable en un bloque anidado";
        println!("Mostrando a mensaje_2 desnde el bloque hijo. {}", mensaje_2);
        
        // Nota: el SHADOWING respeta el alcance de las variables, osea si creamos
        // una variable con el mismo nombre que el bloque padre en el bloque hijo, este
        // es una variable totalmente distinta y no hace "shadowing"
    }

    println!("Mensaje {}", mensaje)

    // SCOPE:
    // Al nosotros definir una variable dentro de un bloque, estamos limitando su alcance, osea
    // donde podemos utilizar la variable, en este caso la variable "mensaje" se crea 
    // en el bloque principal por ende esta variable puede ser utilizada en el bloque en donde
    // fue creada y en los bloques hijos.
}
