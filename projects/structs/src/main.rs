fn main() {
    println!("Hello, world!");

    // Definición e instancia de structs
    let mut user1 = User {
        active: true,
        username: String::from("user123"),
        email: String::from("mail123@mail.com"),
        sign_in_account: 1,
    };

    user1.email = String::from("hola@mail.com");

    // Crear instancias de structs a partir de otros
    let _user2 = User{
        email: String::from("otromail@mail.com"),
        ..user1 // Acá user2 se consume al user1 por lo que no es valido de usar.
    };

    // Tuple struct
    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0); // Si bien ambas tuplas son identicas, como el tipo de struct es diferente, las funciones que sirven con una, no sirven con la otra y viceversa.

    // Unit-like struct
    let equal = AlwaysEqual; // Este es un struct vacío (Unit-like) no tienen valores pero pueden implementar métodos y traits (Ver "Generic types, Traits and Lifetimes").
    
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_account: u64,
}

// Construir e usar Init shorthand
fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        email,
        username, // Como se llama igual al campo no se necesita especificar el valor que modifica
        sign_in_account: 1
    }
}

struct Point(i32, i32, i32);
struct Color(i32, i32, i32);
struct AlwaysEqual;