fn main() {
    // MATCH
    // el match es equivalente a switch en otros lenguajes, osea podemos
    // evaluar un valor en diferentes casos.

    let numero: i32 = 55;

    match numero {
        // valor => sentencia / tarea
        1 => println!("El numero es uno."),
        2 => println!("El numero es dos."),
        3 => println!("El numero es tres."),
        4 | 5 | 6 => println!("El numero se encuentra entre 4 y 6"),
        // si el numero esta entre 7 a 100
        7..=100 => {
            // usamos un bloque cunado vamos a ejecutar multiples sentencias
            println!("El numero es mayor o igual a 7");
            println!("El numero se evalua mediante un rango")
        }
        _ => println!("{}", numero), // esto equivale a default, no lleva ","
    }

    // Lo interesante del "Match" es que podemos retornar valores de el
    let num: i64 = 60;

    let  mensaje = match num {
        1 => "En numero es uno",
        2 => "El numero es dos.",
        3 => "El numero es tres.",
        4 | 5 | 6 => "El numero se encuentra entre 4 y 6",
        // si el numero esta entre 7 a 100
        7..=100 => {
            let mensaje = "El numero se evalua mediante un rango de 7 al 100";
            mensaje
        },
        _ => "numero"
    };
    println!("El resultado es: {}", mensaje);
}
