fn main() {
    // definiendo un arreglo
    let numeros = [1, 2, 3, 4, 5];
    // Al colocar el ":?" en el corchete, le indicamos al compilador que nos muestre
    // que valores almacena el las variables
    println!("Mostrando el arreglo: {:?}", numeros);

    // Tambien podemos definir el tipo de datos que almacena un arreglo, colocamos el 
    // tipo de datoq ue almacenará y le indicamos su capacidad maxima.
    let numeros_2: [i64; 5] = [2 ,4, 6, 8, 10];
    println!("Mostrando el arreglo: {:?}", numeros_2);

    // Otra forma de definir arreglos es por valor por default le pasamos el valor
    // por defecto y el tamalo máximo.
    let valores = [1; 10];
    println!("Mostrando el arrelo por defecto {:?}", valores);

    // mostrando datos de los arreglos
    let primer_elemento = valores[0];
    let ultimo_elemento = valores[valores.len() - 1];
    println!("Mostrando el primer elemento y el ultimo {}, {}, ", primer_elemento, ultimo_elemento);

    // Modificando elementos de array
    let mut num = [10; 5];
    num[3] = 1000;
    println!("Mostrando el elemento modificado {:?}", num);
}
