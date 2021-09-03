// en este caso le decimos a nuestra función que va retornar un enum "Option", y
// le pasamos el tipo de dato que va poseer, osea que va almacenar la tupla "Some()".
// * En resumen, estamos diciendo que la funcion puede retornar algun valor, y si es así
// ese valor será de tipo "String".
fn obtener_valor(bandera: bool) -> Option<String> {
    // retornando la tupla "Some()" con un String o "None" dependiendo
    // de la bandera
    if bandera {
        Some(String::from("Soy un mensaje para la tupla some"))
    } else {
        None
    }
}

fn obtener_valor_2(bandera: bool) -> Option<String> {
    if bandera {
        Some(String::from("Soy un mensaje para la tupla some de la fn 2"))
    } else {
        None
    }
}

fn obtener_valor_3(bandera: bool) -> Option<String> {
    if bandera {
        Some(String::from("Soy un mensaje para la tupla some de la fn 2"))
    } else {
        None
    }
}

fn main() {
    // Muchos lenguajes de programacion utilizan el "null", "nil" o el "undefined"
    // para representar la ausencio de algun valor y excepciones para manejar los
    // errores, en "rust" no existe nada de ello, esto para preveer erroes como el
    // "null pointer exception".
    // * En lugar de ello, "rust" implementa 2 tipos espceciales de "enums"
    // "Option" y "Result", estos nos permiten trabajar la ausencia de valores y
    // con los errores.
    // Option -> Nos permite conocer si existe o no un valor.
    // Result -> Nos permite trabajar con Errores -> estos errores los podemos
    // complementar con la función macro "panic!" para finalizar el programa en caso
    // exista un error.

    // Usando OPTION:
    // * el enum "Option" nos permite trabajr con cualquier tipo de dato, ya que utiliza la
    // abstracción "<T>", el enum posee 2 opciones, "Some(T)" -> el cual es una tupla que almacena
    // cualquier tipo de dato, aqui se almacena el valor que deseemos, la segunda opción es
    // "None" -> nos sirve para representar la ausencia de algún valor:
    /*
        enum Option<T> {
            Some(T), -> El valor
            None -> La ausencia de alg´ún valor
        }
    */
    // EJEMPLO:
    // en este caso resultado será un objeto de tipo "Option"
    let resultado = obtener_valor(true);
    // Una buena forma de trabajar con este tipo de objetos, es atravez de match, en
    // este caso evaluaremos si nuestro resultado devuelve Some o None
    match resultado {
        Some(valor) => println!("El valor es: {}", valor),
        None => println!("No esxite valor alguno"),
    }

    // Otra forma de obtener el valor de un objeto de tipo "Option" es atravez
    // de metodos, el metodo mas utilizado es el "unwrap".
    // unwrap -> intenta obtener lo que la tupla "Some()" almacena.
    let res = obtener_valor_2(true);
    // obteniendo el valor de la variable res usando "unwrap", si "Some()" no posee
    // ningún valor y ejecutamos el "unwrap()", en ese caso se ejecuta un "panic!"
    let val = res.unwrap();
    println!("El valor es: {}", val);

    // si no queremos que se ejecute un panic cuando el valor Some() de una variable
    // no existe, debemos usar "unwrap_or()", el el cual le pasaremos un valor por defecto
    // que tomará la variable en caso de que el Some() no almacene ningún valor, osea None
    let res_3 = obtener_valor_3(false);
    // el val por defecto que le pasemos debe ser del mismo tipo que almacena el Some()
    let val_3 = res_3.unwrap_or("La tupla no almacena valor".to_string());
    println!("El val 3 es: {}", val_3);

    // OTRO METODO: "sxpect()"
    // expect() -> este método tambien intentará obtener el valor de la tupla Some(), si es
    // un None este lanzará "panic!", pero nosotros podemos pasar como argumento una cadena
    // de caracteres la cuald escriba con mas en detalle cual fue el error
    let res_4 = obtener_valor_3(false);
    let val_4 = res_4.expect("Se esperaba un String");
    println!("El val 4 es: {}", val_4);
}
