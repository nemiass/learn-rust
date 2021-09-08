#[derive(Debug)]
enum ErrorDivision {
    DivisionporCero,
    DivisionNegativos,
}

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
    // * Al igual que "Option", "Result" también posee sus metodos, este posee
    // 3 metodos que itentarán obtener lo que "Ok()" almacene, estos metodos son:
    // unwrap, unwrap_or, expect, estos son metodos que ya vimos anteriormente

    let resultado = division(10, 2);
    // usando un wrap para sacar el valor de Ok(), si no existe un valor lanzará un panic,
    // para poder usar el metodo unwrap, debemos poner el #[derive(Debug)] en nuestro enum
    // ErrorDivision que es el que maneja el error que definimos
    let valor = resultado.unwrap();
    println!("Mostrando valor: {}", valor);

    let resultado2 = division(10, 0);
    // usando unwrap_or(), el cual nos permite definir un valor por defecto cuando "Ok()"
    // no contenga un valor establecido, el valor por defecto debe ser del mismo tipo
    // que el valor que va contener el "Ok()"
    let valor2 = resultado2.unwrap_or(0);
    println!("Mostrando valor2: {}", valor2);

    let resultado3 = division(-100, 2);
    // usando expect(), el cual nos permite definir un mensaje para el panic!, de esta
    // forma podemos dar mas información sobre el error
    let valor3 = resultado3.expect("No es posible dividir negativos");
    println!("Monstrando valor3: {}", valor3);
}
