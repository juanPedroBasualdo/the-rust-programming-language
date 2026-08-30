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

        
}
