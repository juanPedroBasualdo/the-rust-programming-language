fn main() {
    let mut i = 0;
    loop {
        i += 1;
        println!("Otra vez!");
        if i == 2 {
            break;
        }
    }

    // Retornar valores de loops
    let mut counter = 0;
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // Acá es como hacer un "return counter * 2" donde se retorna dentro de este loop, esto es completamente loco y solo lo vi en Rust. Es como si el break fuera un return de loop.
        }
    };
    println!("El resultado es: {result}");

    // Loop labels
    let mut count = 0;
    'counting_up : loop {
        println!("Contador: {count}");
        let mut remaining = 10;

        loop {
            println!("Queda: {remaining}");
            if remaining == 9 {
                break; // Este break rompe con counting_down, break a secas funciona solo para el loop más interno.
            }
            if count == 2 {
                break 'counting_up; // Ahora se rompe el loop de counting_up ya que tiene su label.
            }

            remaining -= 1;
        }

        count += 1;
    }

    // Loops condicionales con while
    let mut number = 3;

    while number != 0 {
        println!("{:?}!", number);

        number -= 1;
    }

    println!("LIFTOFF!!!");

    // Iterando sobre una colección con for (y en general)
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    // Opción while
    while index < 5 {
        println!("El valor es: {:?}", a[index]);

        index += 1;
    }

    // Mismo algoritmo con for
    for element in a {
        println!("El valor es: {:?}", element);
    }

    // Algoritmo de cuenta atras versión for
    for number in (1..4).rev() {
        println!("{:?}!", number);
    }
    println!("LIFTOFF!!!");
}
