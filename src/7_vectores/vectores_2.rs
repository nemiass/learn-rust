fn main() {
    // Otra forma de definir vectores usando una ESTRUCTURA
    
    // Definiendo un vector usando la estructura "Vec"
    let mut vector = Vec::new(); // esto genera un vector vacío

    // Al insertar el primer elemento a un vector vacío, el compilador intuye
    // el tipo de dato que va contener el vector, que es apartir del primer elemento
    // insertado
    vector.push(4);
    vector.push(5);
    vector.push(6);
    println!("Mostrando el vector, {:?}", vector);

    // Tambien podemos indicar el tipo de dato que va contener un vector desde 
    // el principio
    let mut vector_2: Vec<i64> = Vec::new();
    vector_2.push(2);
    vector_2.push(4);
    vector_2.push(6);
    println!("Mostrando el vector 2, {:?}", vector_2);

    // NOTA: cuando usar la macro "vec!" o la esctructura "Vec" para definir nuestro vector?,
    // - ps cuando conocemos lo que almacenaremos en nuestro vector se usaría la macro, y cuando
    // no sabemos los datos que alamacenará nustro vector usaríamos la estructura "Vec
    // en teoría.
}