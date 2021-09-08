
// cuando pasamos de esta forma, este array se pasa como copia, osea
// se comporta como un tipo primitivo, ya que ya sabemos que los primitvos
// no pasan su "ownership", sino que pasan una copia
fn double_array(arr: [i32; 5]) {
    println!("dentro del double_array {:?}", arr);
}

// este es otra forma de pasar, en este caso pasamos el array, como referencia,
// podemos ver que es un poco mas facil de escribir, no necesitamos especificar
// el tamaño
fn double_array2(arr: &[i32]) {
    println!("dentro del double_array2 {:?}", arr);
}

fn print_arr_bi(arr_bi: &[i32]) {
    println!("{:?}", arr_bi);
}

fn main() {
    let arr = [5, 10, 15, 20, 25];
    double_array(arr);
    double_array2(&arr);
    println!("mostrando el array en el main{:?}", arr);

    // Array bidimensionales
    //let arr_bi = [[0i32; 5]; 5];
    let arr_bi = [[0, 0], [0, 0]];
    print_arr_bi(&arr_bi);
    println!("\nMostrando el array bidimensional en el main: {:?}", arr_bi);
}
