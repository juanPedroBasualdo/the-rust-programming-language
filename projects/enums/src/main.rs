enum IpAddrKind { // Los enums permiten especificar las variantes de una entidad, ya sea de datos, opciones, resultados, errores, etc.
    V4, 
    V6,
}

struct IpAddr { // Acá se implementa un estilo de simulación de direcciones IP que usan un campo 'kind' que acepta el enum IpAddrKind.
    kind: IpAddrKind,
    address: String,
}

enum InternetAddr { // Pero aún mejor, se puede saltear la necesidad de crear structs ya que las variantes PUEDEN GUARDAR DATOS. :):):)
    V4(String),
    V6(String),
}

enum InternetProtocolAddress { // PERO AÚN MEJOR, se puede especificar en cada variante distintos tipos de datos!!!
    V4(u8, u8, u8, u8),
    V6(String),
}

// Entre las cosas que pueden contener las variantes de un enum están, units, variables, tuplas, otros enums, campos y/u otros structs.
enum Message {
    Quit, // Unit -> ()
    Move { x: u32, y: u32 }, // Campos
    Write(String), // Datos complejos
    ChangeColor(i32, i32, i32), // Tupla
}

// Además se pueden implementar comportamientos para los enums
impl Message {
    fn call(&self) -> &str { // Para instancias, pero también puede ser asociativo
        "¿Hola?"
    }
}

// 'match'

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)] // Los enums tambien pueden derivar e implementar Traits.
enum UsState {
    Alabama,
    Alaska,
    // Etc etc.
}

fn main() {
    println!("Hello, world!");

    let _home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("192.168.0.1"),
    };

    let _loopback = IpAddr {
        kind: IpAddrKind::V6, // Acá se hace uso del enum, como se ve, se especifica la variante a través del uso de ::, es decir: 'Enum::Variante'
        address: String::from("::1"),
    };

    let _home_addr = InternetAddr::V4(String::from("10.0.0.1")); // Bue-ni-si-mo poder hacer esto.
    let _loopback_addr = InternetAddr::V6(String::from("::"));

    let _home_ip_addr = InternetProtocolAddress::V4(181, 45, 0, 1); // WOW
    let _loopback_ip_addr = InternetProtocolAddress::V6(String::from(":::1")); // WOOOW

    let _q = Message::Quit;
    let _m = Message::Move { x: 10, y: 10 };
    let w = Message::Write(String::from("Hola!"));
    let _cc = Message::ChangeColor(0, 0, 0);

    println!("{}", w.call());

    // Option<T>

    let some_number = Some(3); // Option permite al usuario determinar si una variable contiene un valor o no. Si lo tiene es Some(valor), sino es None que representa que el valor de esa variable no existe o es desconocido.
    let some_char = Some('e'); // CUIDADO, esto no es equivalente a 'e', ese valor es un char, este es un Option<char>. Para poder usar el valor que contiene el Some hay que usar pattern matching.

    let _absent: Option<i32> = None; // Acá hay que especificar el tipo de dato que representa este None, porque sino no se sabría qué operaciones son válidas con este valor.

    // El constructo de control de flujo 'match'
    let penny = Coin::Penny;
    let alaskan_quarter = Coin::Quarter(UsState::Alaska);

    println!("La moneda de {} centavo se llama Penny", value_in_cents(penny));
    println!("La moneda de {} centavos se llama Quarter", value_in_cents(alaskan_quarter));


    // Matcheo con Option<T>

    let number = match some_number {
        Some(num) => num,
        None => 0,
    };

    println!("El resultado es {:?} (Debe ser 8, no 0)", number + 5 );

    // Los matcheos son exhaustivos

    let _e = match some_char {
        Some(c) => c,
        None => ' ', // Si este valor no se considera, el compilador tira un error pues los matcheos tienen que considerar todos los casos
    };

    // Patrones Catch-All y '_' como enunciado por defecto.

    let dice_roll = 9; // Para este ejemplo se hardcodea un valor de tirada de dados.
    match dice_roll {
        3 => add_fancy_hat(), // También se puede matchear por resultado
        7 => remove_fancy_hat(),
        _ => reroll(), // El '_' es el valor por defecto de matcheo 'atrapa todo', esto evita tener que ser exhaustivos con cada uno de los resultados posibles. Se puede designar un nombre a este valor pero lo normal es usar '_' ya que no almacena el valor del patrón matcheado.
    }

    // Control de flujo conciso con if let
    let max = Some(3u8);
    if let Some(max_config) = max {
        println!("El máximo está configurado para ser {}",  max_config);
    } // If let permite solo implementar un patron de matcheo, todas las demás variantes se asignan a '_ => ()', es decir que no hacen nada.

    let mut _coins = 0;
    let coin = Coin::Penny;
    if let Coin::Quarter(state) = coin {
        println!("Un cuarto de dolar del estado de {:?}", state);
    } else { // También se puede usar else para que no se asigne al catch-all, sino que se realice otra operación.
        _coins += 1;
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => { // Para matchear a bloques de código se puede usar llaves, se tiene que retornar el valor pedido por la función en algún momento.
            println!("Lucky penny!");
            1
        },
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => { // El valor especificado dentro del parentesis puede usarse en cualquier momento dentro del matching.
            println!("Una moneda del estado de {:?}!", state);
            25
        },
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn reroll() {}