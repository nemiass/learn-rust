struct Rectangulo {
    ancho: u32,
    alto: u32
}

fn area(rectangulo: &Rectangulo) -> u32 {
    rectangulo.ancho * rectangulo.alto
}

fn main() {
    // OWNERSHIP
    // posee 3 reglas:
    // - cada valor en rust tiene su propio Ownership
    // - solo puede existir un ownership a la vez
    // - si un ownership sale del alcance, el valor se descartará

    // En este ejemplo se aplicará la regla #2 -> solo puede existir
    // un ownership a la vez.

    let rectangulo = Rectangulo{ ancho: 10, alto: 20};
    let resultado = area(&rectangulo);

    // en este caso vemos que, el ownership de rectangulo pasa a "nuevo_rectangulo"
    let nuevo_rectangulo = rectangulo; // movimiento de ownership
    // eso quiere decir que daria error ejecutar la sguiente linea de codigo, ya
    // que rectangulo deja de existir ya que su ownership se ha movido a "nuevo_rectangulo"
   // ->  println!("El rectangulo, ancho: {}, alto:{}", rectangulo.ancho, rectangulo.alto);

   // lo correcto es hacer el prinln! anterior cin "nuevo:rectangulo", ya que el ownership
   // de "rectangulo" se movió a "nuevo_rectangulo"
   println!("El rectangulo, ancho: {}, alto:{}", nuevo_rectangulo.ancho, nuevo_rectangulo.alto);
    // con esto se compruenba la regla 2, que solo puede haber un ownership a la vez

    // Nota: Esto sucede unicamente para los objetos que se almacenan el el HEAP, pero no 
    // para los que estan en el STACK
    println!("El area es del rectangulo es: {}", resultado);

    // EJEMPLO DE STACK
    let x = 10;
    let y = x;
    println!("{}", x);
    println!("{}", y);
    // vemos que al ejecutar esto no da problemas de ownership, ya que estas variables
    // se almacenan el en STACK, estos no presentan un movimiento de ownership

    // EN RESUMEN:
    // * todas las variables que se almacena  el en HEAP, van a mover su ownership
    // siempre que se haga una asignación, caso contrario para las variables que se 
    // almacenen en el stack ya que estos ya poseen un tamaño definido, no es necesario
    // conocer a quien le pertenecen.
}
