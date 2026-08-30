use std::io;

fn main() {
    // Variables y mutabilidad
    let mut x = 5;
    println!("El valor de x es: {}", x);
    x = 6;
    println!("El valor de x es: {}", x);

    // Shadowing y contexto
    let y = 5;
    let y = y + 1;
    {
        let y = y * 2;
        println!("El valor de y dentro del contexto es: {}", y);
    }
    println!("El valor de y fuera del contexto es: {}", y);

    // Tipo entero
    let guess: u32 = "42".parse().expect("No es un numero!");
    println!("El valor parseado es {guess}");

    // Tipo punto flotante
    let _f_64 = 2.0;
    let _f_32 : f32 = 3.0;

    // Operaciones numéricas
    let _sum = 5 + 10; // Adicion
    let _difference = 95.5 - 4.3; // Resta
    let _product = 4 * 30; // Multiplicación
    let _quotient = 56.7 / 32.2; // División (de flotantes)
    let _truncated = -5 / 3; // División truncada (es i32 por lo que resulta en -1 truncado)
    let _remainder = 43 % 5; // Resto / módulo

    // Tipo booleano
    let _t = true;
    let _f = false;

    // Tipo caracter
    let _c = 'z'; // Un byte en formato ASCII u8 ('z' = 122 en u8)
    let _z = 'Z';

    // Tipo tupla
    let tup : (i32, f64, u8) = (500, 6.4, 1); // La tupla permite que los valores que guarda sean diferentes
    let (_x, y, _z) = tup; // Pattern matching para acceder a los valores individuales

    println!("El valor de y es: {y}");

    let tup_2 : (i32, f64, u8) = (500, 6.4, 1); // Tambien se puede acceder por índices
    let _five_hundred = tup_2.0; // Idem a tup_2[0]... etc.
    let _six_point_four = tup_2.1;
    let _one = tup_2.2;

    // Tipo array (arreglo)
    let a = [1, 2, 3, 4, 5]; // En el array tienen que ser todos iguales, se guarda exclusivamente en Stack
    let _months = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"]; // Tiene un tamaño fijo
    let _b : [i32; 5] = [1, 2, 3, 4, 5]; // Tipo y tamaño específico
    let _c = [0; 5]; // Array con todos los valores iguales 5 veces

    let _first = a[0];
    let _second = a[1];

    // Acceso fuera de rango
    println!("Por favor escribir un indice de array");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Falló al leer la linea");

    let index: usize = index
        .trim()
        .parse()
        .expect("No es un número");

    let element = a[index]; // Si el valor del índice es >5 va a dar un panic y el programa crashea

    println!(
        "El valor del elemento en el índice {index} es: {element}"
    ); 

    // Funciones
}
