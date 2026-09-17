use std::{cmp::PartialOrd};

// Generics en funciones
fn largest<T: PartialOrd>(list: &[T]) -> &T { // El tipo generico T permite que una función pueda aplicarse para muchos tipos de datos diferentes
    let mut largest = &list[0];

    for item in list {
        if item > largest { // Acá se requiere el Trait PartialOrd (Partial Order) para poder comparar los valores dentro de la lista, si el tipo de dato no implementa PartialOrd, no se puede comparar. Para más info ver "Traits: Defining Shared Behaviour (Cap-10)"
            largest = item;
        }
    }

    largest
}

// Generics en structs
#[derive(Debug)]
struct Point<T> { // Acá Point solo puede ser tipo T, entonces sus campos x e y deben ser si o si del mismo tipo T
    x: T,
    y: T,
}

#[derive(Debug)]
struct Points<T, U> { // Agregando un nuevo Generic permite que x e y sean de distinto tipo.
    x: T,
    y: U
}

// Generics en definiciones de Enums
enum _Resultado<T> { // Por ejemplo podríamos definir un Result propio o un Options propio que cargue con generics dentro de este.
    Ok(T),
    Err
}

enum _Opciones<T, A, N> { // O tener cuantas opciones se quieran, la convención es usar T, U, V... etc, pero pueden tener el nombre que se desee.
    Todas(T),
    Algunas(A),
    Nada(N)
}

// Generics en métodos
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}


impl Point<f64> { // Pueden declararse métodos específicos para un tipo especifico de dato.
    fn distance_from_origin(&self) -> f64 { 
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl<T, U> Points<T, U> {
    fn mixup<X, Y>(self, other: Points<X,Y>) -> Points<T, Y> { // También se pueden declarar otros Generics dentro de los métodos.
        Points { x: self.x, y: other.y }
    }
}

fn main() {
    // Generics en funciones
    let number_list = vec![34, 50, 25, 100, 65];

    let result = largest(&number_list);
    println!("El numero más grande es: {result}.");

    let char_list = vec!['y', 'm', 'a', 'q']; // Los chars se ordenan según que tan grande es su valor en Bytes. Normalmente de manera alfabética -> numérica -> demás cosas.

    let result = largest(&char_list);
    println!("El char más grande es: {}", result);

    // Generics en structs
    let integer = Point { x: 5, y: 10 }; 
    let float = Point { x: 10.5, y: 11.4 };

    let mixed = Points { x: 5.5, y: 'a' };

    // Generics en metodos
    let x = integer.x();
    println!("p.x = {x}");

    let origin = float.distance_from_origin();
    println!("Distancia del punto: {float:?} hasta el origen es: {origin}.");

    let super_mixed = mixed.mixup(Points { x: 'b', y: 'b'});
    println!("Super mix es: {super_mixed:?}")
}
