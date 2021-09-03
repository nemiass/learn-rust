fn mani() {
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
    // "Err()" se usará cuando existe un error y este almacenará "Err(E)"
}