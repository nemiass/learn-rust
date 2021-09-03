
// Definiendo una structura en Rust
struct User {
    username: String,
    password: String
}

fn create_user(username: String, password: String) -> User {
    User{ username, password }
}

fn main() {
    // onstanciando nen teoríanuestra estructura
    let user = User {
        username: String::from("Usuario"),
        password: String::from("1234")
    };

    println!("Mostrando a usuario:{}, password:{}", user.username, user.password);

    // Esto es otra forma de declarar una variable de tipo , las variabls a ingresar
    // deben tener nombre igual a las filas a ingresar de la estructura "Usuario"
    let username = String::from("nemias");
    let password = String::from("1111");
    let user_nemias = User{ username, password };

    println!("Mostrando usuario, name:{}, password:{}", user_nemias.username, user_nemias.password);

    // creando al usuario mediante la funcion
    let user2 = create_user("Cristina".to_string(), "123".to_string());
    println!("Mostrando el nuevo user, name:{}, password:{}" ,user2.username, user2.password);

    // Para que nuestros usuarios que creemos sean mutables, debemos declararlos como mutables
}