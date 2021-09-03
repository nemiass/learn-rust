fn main() {
    // Los vectores y arreglos son lo mismo, osea ambos solo pueden almacenar 
    // valores de un mismo tipo, pero la diferencia entre un vecotor y un arreglo
    // es que los vectores is pueden modificar su longitud, osea puede crecer o
    // decrecer, estos tambien tienen el metodo "len()".

    // Definiendo nuestro vector, hacemos uso de la macro vec!, el cual nos permite crear
    // un vector
    let mut vector = vec![1, 2, 3];
    println!("El valor del vector es {:?}", vector);
    // Agregando nuevos elemtos al vector
    vector.push(4);
    vector.push(5);
    println!("El valor del vector modificado es {:?}", vector);
    // Otra forma de agregar datos al vector es con su metodo "insert()", este recive
    // 2 valores, el primero es la posición en el cual queremos agregar un elementom y el
    // segundo es el elemento como tal, los elementos corren a la derecha del indice insertado
    vector.insert(0, -1);
    println!("Mostrando el vector despues del insert (0, -1), {:?}", vector);

    // ELIMINANDO
    // eliminando un elemento de un vector, "remove()", este recibe el indice del elemento
    // a remover.
    vector.remove(0);
    println!("Mostrando el vector despues de remover remove(0), {:?}", vector);

    // Obteniendo el elemento de un vector
    let primer_elemento = vector[0];
    println!("Mostrando el primer elemento del vector, {}", primer_elemento);

    // MODIFICANDO
    // modificando un elemento del vector
    vector[0] = 1000;
    println!("Mostrando el vector modificado, {:?}", vector);
    
    // PODEMOS usar los vectores como una pila, ya se tiene el metodo "pop()",
    // este retorna un option, hacemos "unwrap()" para tener el ultimo elemento
    let ultimo_elemento = vector.pop().unwrap();
    println!("Mostrando el elemento extraido con pop(), {}", ultimo_elemento);
}