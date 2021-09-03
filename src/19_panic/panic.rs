fn main() {
    // PANIC
    // -panic es una funcion macro el cual nos permite finalizar el programa en caso exista
    // algun tipo de error.
    println!("Hola soy un mensaje de la linea 5");
    println!("Hola soy un mensaje de la linea 6");
    // ejemplo de simulando un error, el programa finaliza en esta linea y no se muestran
    // el resto de lineas del codigo, todo esto hecho con pa función "panic!", como sabemos
    // en "rust" no existen las excepciones, asi que cuando queramos finalizar nuestro programa
    // por algun error debemos hacer uso de "panic!"
    panic!("El programa finaliza de forma inesperada!!!");

    println!("Hola soy un mensaje de la linea 7");
    println!("Hola soy un mensaje de la linea 8");
}