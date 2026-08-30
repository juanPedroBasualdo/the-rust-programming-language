fn main() {
    let number = 3;

    // Expresiones 'if' y 'else'
    if number < 5 {
        println!("La condición es verdadera!"); 
    } else {
        println!("No se cumplió la condición.");
    }

    if number != 0 {
        println!("El numero no es cero.");
    }

    // Expresion 'else if'
    let number = 6;

    if number % 4 == 0 {
        println!("El numero es divisible por 4.");
    } else if number % 3 == 0 {
        println!("El numero es divisible por 3.");
    } else if number % 2 == 0 {
        println!("El numero es divisible por 2.");
    } else {
        println!("El numero no es divisible por 2, 3 o 4.");
    }

    // Usar if en la asignación de variables
    let condition = true;

    let number = if condition { 5 } else { 6 }; // Acá los valores tienen que tener un solo tipo, esto debe ser para evitar que algunas funciones solo funcionen si la variable es de un tipo u otro. Esto generaría efectos secundarios en la concurrencia que podrían hacer que el código se rompa en ciertas condiciones de carrera.

    println!("El valor del numero es {number}");
}
