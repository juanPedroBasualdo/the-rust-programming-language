use std::collections::HashMap;

fn main() {
    // Vectores

    let _v: Vec<i32> = Vec::new();
    let _v = vec![1, 2, 3]; // Tipo inferido por el valor que contiene.

    // Insertar valores a los vectores
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7); // Tipo inferido por el valor que agrega.

    // Leer valores de vectores
    let mut v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2]; // Por indice. Los indices van del 0-n. Devuelve referencia al propio valor.
    println!("El valor en 3er lugar es: {third}");

    let third: Option<&i32> = v.get(2); // Por get. Devuelve Option que contiene referencia al valor si Some o None.
    match third {
        Some(third) => println!("El valor en el 3er lugar es: {third}"),
        None => println!("No hay valor en ese índice.")
    } // Como todo match se debe garantizar usar todas las variantes;

    {
        let first = &v[0];
        println!("El valor en primera posición es {first}")
    }
    
    v.push(6); // Al hacer push, se hace un borrow mutable, ya que si el vector tiene que cambiar de capacidad se debe copiar los valores a un nuevo vector cuya capacidad es mayor, por eso si first no se dropea antes del push, el borrow checker da pánico al compilar ya que se tiene un borrow mutable y uno inmutable, por lo que el valor que presta a first puede estar apuntando a memoria invalida.

    // Usando enums para guardar distintos tipos.
    enum SpreadsheetCell {
        Int(i32),
        Float(f32),
        Text(String),
    }

    let _row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.2),
    ]; // Como las variantes de un enum pueden llevar distintos tipos, se pueden guardar distintos tipos en un vector usando variantes de un mismo enum.

    // Soltando vectores y sus elementos
    {
        let _v = vec![1, 2, 3, 4, 5];

        // Hacer cosas con v
    } // <- v sale de scope y por lo tanto se dropea junto con sus elementos.

    // Strings

    let _s = String::from("Hola, mundo");
    let _s = "Hello world".to_string(); // El to_string() hace que un string literal pase a un String collection.

    // Actualizando un string
    let mut s = "Hola".to_string();
    s.push_str(" mundo");
    println!("{s}");

    let mut s1 = "Hola".to_string();
    let s2 = " mundo";
    s1.push_str(s2);
    println!("{s1}");
    println!("s2 es: '{s2}'"); // push_str() no toma ownership del contenido de s2.
    let mut s = "lo".to_string();
    s.push('l');
    println!("{s}");

    let s1 = "Hola ".to_string();
    let s2 = "mundo".to_string();
    let s = s1 + &s2;
    println!("{s}");
    println!("{s2}"); // add() no toma posesión de s2 pero si de s1

    let s1 = "Tic".to_string();
    let s2 = "Tac".to_string();
    let s3 = "Toe".to_string();
    let _s = s1 + "-" + &s2 + "-" + &s3; //Esto hace Tic-Tac-Toe

    let s1 = "Tic".to_string();
    let s = format!("{s1}-{s2}-{s3}"); // Esto tambien hace Tic-Tac-Toe pero no se queda con s1.
    println!("s1: {s1}, s2: {s2}, s3: {s3}, s: {s}");

    // Indexar a un String
    let hello = "hola".to_string();
    let answer = hello.chars().nth(0); // Rust guarda los strings como UTF-8, como UTF-8 guarda en distintos tamaños sus caracteres, indexar un string es un poco más complicado que simplemente hacer &hello[0]. Se puede usar .chars() para dividir el string en chars o .bytes() para obtener el valor en bytes del caracter. Luego .nth(n) devuelve el valor en el n-avo lugar del string como un Option<byte> o Option<char> según si se usó .chars() o .bytes().

    match answer {
        Some(answer) => println!("{answer}"),
        _ => (),
    }; // Esto al final va a imprimir 'h'

    // Indexando slices
    let hello = "Hola";
    let s = &hello[0..1]; // Los slices si se pueden indexar, pero debe ser en un intervalo.
    println!("{s}"); // 'H'

    // Metodo para iterar sobre Strings
    let hello = "hola".to_string();

    for c in hello.chars() {
        println!("{c}");
    } // .chars() devuelve un tipo de dato tipo Chars<'_> que es iterable y puede ser impreso por stdout sobrepasando la necesidad de usar matching.

    for c in "hola".bytes() {
        println!("{c}");
    } // .bytes() devuelve un tipo de dato tipo Bytes<'_> que tiene las mismas características que Chars<'_>.

    // HashMap

    let _hm: HashMap<String, i32> = HashMap::new();
    let mut scores = HashMap::new(); // Tipo implicito a partir de valores agregados.
    scores.insert("blue".to_string(), 10);
    scores.insert("yellow".to_string(), 50);

    // Acceder a valores de un HashMap
    let team_name = "blue".to_string();
    let score = scores.get(&team_name).copied().unwrap_or(0); // Get devuelve Option<&v> donde Some(&v) es la referencia del valor de la key del HashMap o None si la key no existe. .copied() hace que devuelva una copia del valor como Option<v>, siguiendo la misma idea que .get(). Por ultimo unwrap devuelve el valor en Option<v> a menos que sea None donde el _or(0) hace que devuelva 0.
    println!("Puntaje del equipo azul: {score}");

    for (key, value) in &scores {
        println!("{key}: {value}")
    } // Patern matching en loop con for.

    // Agregando valores a un HashMap
    let mut scores = HashMap::new();
    scores.insert("blue".to_string(), 10);
    scores.insert("blue".to_string(), 20); // Esto reemplaza el valor de la key "blue".
    println!("{:?}", scores);

    scores.entry("yellow".to_string()).or_insert(50);
    scores.entry("blue".to_string()).or_insert(50); // Aquí solo se inserta el valor si la key no existe.
    println!("{:?}", scores);

    let text = "Hola mundo hermoso mundo";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);

}