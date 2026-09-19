pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

pub fn greeting(name: &str) -> String {
    format!("Hello {name}!")
}

pub fn add_two(num: i32) -> i32 {
    num + 2
}

// Esta crate es parte del capitulo 11 que trata sobre pruebas en Rust.

// Como escribir pruebas.
#[cfg(test)] 
mod tests { 

    // mod tests es un modulo especial que permite ejecutar 'cargo test' para ejecutar en la librería una serie de pruebas que verifiquen el comportamiento de alguna entidad implementada en la librería, sin tener que hacer uso de un main que pruebe de manera manual el código.
    use super::*;

    #[test] // Las funciones que empiecen con #[test] se ejecutan con cargo test.
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn panic() { // Descomentar el panic para hacer fallar el test.
        // panic!("Hacer fallar el test"); 
        assert!(true);
    }

    // Probando resultados con el macro assert!()
    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle {
            width: 8,
            height: 7
        };

        let smaller = Rectangle {
            width: 7,
            height: 6,
        };

        assert!(larger.can_hold(&smaller)); 
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };

        let smaller = Rectangle {
            width: 7,
            height: 5,
        };

        assert!(!smaller.can_hold(&larger)); // También funciona para falsos.
    }

    // Probando igualdades con assert_eq!() y assert_ne!()
    #[test]
    fn two_plus_two_is_four() {
        let four = 2 + 2;

        assert_eq!(four, 4);
        assert_ne!(four, 5);
    }

    // Añadiendo mensajes de falla personalizados
    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"), 
            "El saludo no contiene el nombre, el valor recibido fue '{result}'"
            );
    }

    // Probando panics con should_panic
    #[test]
    #[should_panic (expected = "menor o igual a 100")]
    fn guess_greater_than_100_panics() {
        Guess::new(200);
    }

    #[test]
    #[should_panic (expected = "mayor o igual a 1")]
    fn guess_less_than_1_panics() {
        Guess::new(0);
    }

    // Usando Result<T, E> en tests
    #[test]
    fn works() -> Result<(), String>{
        if 2 + 2 == 4 {
            Ok(())
        } else {
            Err(String::from("Dos más dos no es cuatro")) // Preocupante
        }
    }

    // Controlando como corren los tests
    // Usar cargo test permite una variedad de opciones para la ejecución de los tests.
    // Los argumentos de consola se pueden escribir después de un doble guión '--'
    // Los test se corren de forma paralela, usando una cantidad X de hilos de ejecución. Si se quiere usar una cantidad específica de hilos se puede hacer:
    // 'cargo test -- --test-threads=Y' donde Y es la cantidad de hilos a usarse.
    // Además se puede visualizar el output tanto de los tests que fallan como de los que pasan usando:
    // 'cargo test -- --show-output' y se podrán ver los output de los tests que pasan también

    // Correr un subconjunto de tests por nombre
    #[test]
    fn add_two_to_two_makes_four() {
        assert_eq!(4, add_two(2))
    }

    #[test]
    fn add_two_to_three_makes_five() {
        assert_eq!(5, add_two(3))
    }

    #[test]
    fn one_hundred_plus_two_makes_one_hundred_and_two() { // Su corro 'cargo run add' este test se va a ver filtrado junto con todos los que no tengan 'add' en el nombre.
        assert_eq!(102, add_two(100))
    }

    
}

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("El valor debe ser mayor o igual a 1");
        } else if value > 100 {
            panic!("El valor debe ser menor o igual a 100");
        }
        Guess { value }
    }

    pub fn value(&self) -> i32 {
        self.value
    }
}


pub struct Rectangle {
     width: u32,
     height: u32,
}

impl Rectangle {
    pub fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
