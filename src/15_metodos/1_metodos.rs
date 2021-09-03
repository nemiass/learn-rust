
// Con esto le indicamos al Rust que podremos examinar los objetos de tipo User,
// osea que este podrá se mostrado cuand usemos el "{:?}" en el println!()
#[derive(Debug)]
struct User {
    username: String,
    password: String,
}

// De esta forma se implementan metodos para las estructuras
impl User {
    // - un metodo es una funcion que le pertenece a una estructura
    // Todos los metodos reciben el parametro "self" que hace referencia al objeto en sí
    fn saluda(& self) {
        println!("Hola soy el usuario: {}", self.username);
    }

    // como ya sabemos le ponemos self como una referencia mutable, para poder modificar
    // los atributos
    fn chanin_password(&mut self, new_password: String) {
        // colocamos ";" para que nuestro valor no se retornes
        self.password = new_password;
    }
}

fn main() {
    // Implementacion de metodos para las structuras
    let mut user1 = User {
        username: String::from("Cristina"),
        password: String::from("1234"),
    };
    user1.saluda();
    // modificando el password de user1 mediante su metodo
    user1.chanin_password("Nuevo password".to_string());
    println!("El usuario es : {:?}", user1);
}