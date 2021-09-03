struct Rectangulo {
    ancho: u32,
    alto: u32
}

fn area(rectangulo: Rectangulo) -> u32 {
    rectangulo.ancho * rectangulo.alto
}

fn area2(rectangulo: &Rectangulo) -> u32 {
    rectangulo.ancho * rectangulo.alto
}

fn main() {
    // OWNERSHIP
    // posee 3 reglas:
    // - cada valor en rust tiene su propio Ownership
    // - solo puede existir un ownership a la vez
    // - si un ownership sale del alcance, el valor se descartará

    let rectangulo = Rectangulo{ ancho: 10, alto: 20};
    let resultado = area(rectangulo);
    println!("El area es del rectangulo es: {}", resultado);

    // En este ejemplo tenemos la variable "rectangulo", al pasarlo como
    // argument de la funciona area, esta deja de existir para el scope main()
    // y solo existe para el scope "area()", cuando area finaliza, todo lo que hay en
    // su scope se destruye, por lo tanto retangulo deja de existir, el cual ya no puede
    // ser usado.

    // Los argumentos son pasados mediante prestamo, eso es el comportamiento por default

    // Otra forma es pasar por referencia, osea cuando pasemos prestemos un dato a algo, 
    // este tiene que deveolver el "ownership" de el dato prestado, este ejemplo se ve con
    // la funcion "area2(rectangulo: &Rectangulo)" que espera una referencia a rectangulo.
    let rectangulo2 = Rectangulo{ancho:5, alto:5};
    let res = area2(&rectangulo2);
    println!("El areal de rectangulo 2 es: {}", res);
    println!("y el rectangulo es ancho: {}, alto: {}", rectangulo2.ancho, rectangulo2.alto);
}
