fn main() {
    // TIPOS DE DATOS

    // i8, i16, i32, i64, i128 -> Con signo, positivos y negativos + -
    // u8, u16, u32, u64, u128 -> Sin signo, solo positivos +

    // el compilador nos mostrará un warning, si no estamos haciendo uso de alguna variable,
    // para solucionarlo colocamremos un "_" al inicio de nustas variables, o también
    // podemos imprimirlo con la macro println! para que no nos marque el warning
    let _n1: i8 = -10;
    let _n2: u8 = 10;

    // CHAR -> UTF-8
    // los caracteres se reresentan usando comillas simples
    // rust utiliza el codificado "UTF-8", podemos colocar emollis
    let caracter = 'a';
    println!("Caracter: {}", caracter);

    // FLOTANTES -> f32, f64
    let flotante: f64 = 12.5;
    println!("Un valor flotante: {}", flotante);

    // BOOLEANOS -> true, false
    let boleano = true;
    println!("Un valor boolelano: {}", boleano);

    // str y String
    // Son tipos de datos que serán explicados depués...
}
