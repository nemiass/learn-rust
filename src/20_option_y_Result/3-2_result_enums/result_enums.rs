
// creando nuestro emun ErrorDivision para manejar los tipos de errores
// que pueden ocurrir en nustra division(), osea aqui definimos los tipos de
// errores que puedan ocurrir en nuestra division()
enum ErrorDivision {
    DivisionporCero,
    DivisionNegativos,
}

// definimos nuesta funcion division() que devuelve un i32 si todo está correcto
// y en caso contrario retornamos un enum "ErrorDivision" que es un enum que nosotros
// definimos para manejar los tipos de errores que puedan ocurrir en nuestra division(),
// todo ello lo hacemos con un "Result".
fn division(num1: i32, num2: i32) -> Result<i32, ErrorDivision> {
    if num2 == 0 {
        return Err(ErrorDivision::DivisionporCero);
    }
    if num1 < 0 || num2 < 0 {
        return Err(ErrorDivision::DivisionNegativos);
    }
    Ok(num1 / num2)
}

fn main() {
    // RESULT
    // * En este ejempo vamos a definir nuesto Error usando un Enum que nosotros
    // hemos creado para controlar los errores que puedan ocurrir en nuestra division()

    let resultado = division(-10, 2);

    // podemos ver que ahora tenemos un poco mas de flexibilidad al momento de definir
    // los posibles errores que hayan ocurrido, ademas de ello podemos usar la macro
    // panic para decidir finalizar un porograma en determinado error
    match resultado {
        Ok(valor) => println!("El resultado es: {}", valor),
        Err(ErrorDivision::DivisionporCero) => {
            // aqi el programa finaliza, caso contrario pasa el el segundo Err
            // que muestra el error en pantalla y el programa continua.
            panic!("No se puede dividir por 0, adios ...")
        },
        Err(ErrorDivision::DivisionNegativos) => {
            println!("El error es por intentar dividir numeros negativos")
        }
    }
}
