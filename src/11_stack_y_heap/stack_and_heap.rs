fn main() {
    // STACK y el HEAP es la forma en que Rust gestiona la memoria
    
    // STACK:
    // aqui se encuentran alojadas todas aquiellas variables las cuales ya conozcamos
    // su longitud, es decir su tamaño, el stack es mas rapido porque ya conocemos el tamaño
    // de la variable. Osea lo vemos como una pila donde se almacenan los valores con forme
    // estos se van agregando, osea es un LIFO, esto hace muy rapido al stack en cuanto
    // a la escritura y lectura de valores.

    // HEAP
    // Cuando se desconoce el tamaño de las variables, estas se almacenan en el HEAP,
    // aquí se ecuentran todas aquellas variables en las cuales su tamaño pueda variar
    // en tiempo de ejecución, un ejemplo de esto son los vestores y strings.
    // - el HEAP es un poco ms lento que el STACK debido a su naturaleza.
}