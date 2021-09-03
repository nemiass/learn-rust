fn main() {
    // La diferencia entre tupas y arreglos, es que las tuplas pueden almacenar
    // diferenctes tipos de datos.

    // Definiendo una tupla
    let tupla = (10, false, 2.5);
    println!("Mostrando la tupla {:?}", tupla);

    // Otra forma de definir tuplas
    let tupla_2: (i32, bool, f64, i64) = (10, false, 5.5, 1000);
    println!("Mostrando la tupla2 {:?}", tupla_2);

    // Accediento a lso datos de la tupla, para ello se hace de la siguiente forma
    let primer_elemento = tupla_2.0;
    let ultimo_elemento = tupla_2.3;
    println!("Primer elemento y ultimo elmento: {}, {}", primer_elemento, ultimo_elemento);

    // MODIFICANDO elementos de una tupla
    let mut my_tupla = (10, false, 2.5, 33);
    my_tupla.1 = true;
    println!("Mostrando mi tupla modificada {:?}", my_tupla);
}