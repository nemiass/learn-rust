

// este es la funcion normal, sim implementar el "enum Result" para manejar
// el error, osea si dividimos por "0", el programa nos dará un panic
fn division(num1: i32, num2: i32) -> i32 {
    num1 / num2
}

fn division2(num1: i32, num2: i32) -> Result<i32, String> {
    if num2 == 0 {
        return Err(String::from("No es posible dividir por 0"))
    }
    Ok(num1 / num2)
}

fn main() {
    // RESULT
    // * ahora toca el turno de hablar con result, enum el cual se menciocnó
    // anteriormente el cual nos permitirá trabajar con errores.
    // Definiendo su estructura:
    // * enum result<T, E>, el enum result puede almacenar dos valores "T y E", estos valores
    // son mutuamente excluyentes, osea no pueden existir los dos al mismo tiempo, ambos
    // valores pueden ser de cualquier tipo sea entero, flotante, boleano, caracter, string,
    // vector, array, tupla, etc.
    // * T -> hace referencia al valor el cual queremos almacenar.
    // * E -> hace referencia al error mismo.
    // en "enum Result" posee dos opciones "OK()" y "Err", "Ok()" se usará cuando 
    // no exista ningún error y podamos establecer un valor, "Ok(T)" almacenará "T",
    // "Err()" se usará cuando existe un error y este almacenará "Err(E)":
    /*
        enum Result<T, E> {
            Ok(T),
            Err(E)
        }
     */
    // ejemplo: En este caso tenemos una division de dos numeros, pero que pasa si
    // dividimos un numero entre "0", pues el programa hará un panic, apra controlar
    // ese error implmentaremos el "enum Result".
    // * la variable "resultado" es un objeto de tipo result, asi que podemos usar
    // el match.
    let resultado = division2(10, 0);

    match resultado {
        Ok(valor) =>  println!("El resultado es: {}", valor),
        Err(error) => println!("El error es: {}", error)
    }
    // ene ste ejemplo estamos controlando un error, podemos ver que al 
    // dividir por "0" nuestro programa no finaliza con el panic, sino que
    // nos muestra un mensaje del erorr que nosotros definimos con el enum Option,
    // osea estamos controlando el error atravez el "enum Result"
}
