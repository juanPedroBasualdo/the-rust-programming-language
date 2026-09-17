use std::{fs::{self, File}, io::{self, ErrorKind, Read}};

fn main() {

    // Errores recuperables con Result<T, E>
    let file_result = File::open("hello.txt");

    let _file = match file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => { // Esto permite que el error, que es recuperable simplemente creando el archivo, no termine repentinamente con la ejecución cuando se puede solucionar.
                match File::create("hello.txt") {
                    Ok(fc) => fc,
                    Err(e) => panic!(
                        "Error creando el archivo: {e}"
                    ),
                }
            }
            other => {
                panic!("Error abriendo el archivo: {other}"); 
            }
        }
    };

    // Alternativas al matching para errores recuperables.
    let _file = File::open("hola.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hola.txt").unwrap_or_else(|error| {
                panic!("Problema creando el archivo: {error}");
            })
        } else {
            panic!("Problema abriendo el archivo: {error}");
        }
    }); // Puff, que locura. Otra forma de solucionar errores recuperables es usando unwrap_or_else(), esto es funcional puro y duro, usa una clausula para determinar que hacer si da error.

    // Atajos para hacer panic! al errar: unwrap y expect.
    let _file = File::open("konichiwa.txt").unwrap(); // Al igual que unwap_or_else(), esot permite no usar matching, pero en este caso es simplemente o se hace o se hace panic!.

    let _file = File::open("bonjour.txt").expect("bonjour.txt tiene que existir");  // Esto es para los más quisquillosos, si OBLIGATORIAMENTE tiene que existir la precondición se usa expect, que al hacer panic! te da el mensaje de lo que se esperaba para que no haga panic.

    // Incluyo los txt para poder ejecutar la siguiente sección.

    
}

// Propagando errores
fn _read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

// El ? operador
fn _read_username_from_file_short() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?; // El operador ? no es el ternario lamentablemente :( pero es un equivalente al matching de Result donde se devuelve el valor dentro de Ok.
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

fn _read_username_from_file_shorter() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?; // Se pueden concatenar los ?

    Ok(username)
}

fn _read_username_from_file_shortest() -> Result<String, io::Error> {
    fs::read_to_string("hello") // Lo más chistoso de todo es que como es una operación super común el std de Rust tiene una operación que hace esto. Acá prácticamente se está haciendo un wrapping de dicha función.
}

// Donde puedo usar ? ?
fn last_char_of_first_line(text: &str) -> Option<char> {
    text.lines().next()?.chars().last() // En funciones donde se devuelva Result<T, E> o Option<T>. En esta segunda opción, si es None se devuelve None, sino se devuelve lo que tiene adentro Some<T>.
}