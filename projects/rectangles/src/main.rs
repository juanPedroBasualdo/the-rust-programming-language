#[derive(Debug)] // El trait derivado Debug permite imprimir info debug respecto al struct.
struct Rectangle {
    width: u32,
    height: u32,
}

// Sintaxis de método
impl Rectangle {
    fn area(&self) -> u32 { // 'self: &Self' (alias &self) sirve para que la operación solo pueda ser llamada con una implementación del struct, si no se especifica que sea una referencia, el método se consume a la instancia luego de invocar al método.
        self.width * self.height
    }

    fn width(&self) -> bool { // Se pueden definir métodos que tengan el mismo nombre que los campos de un struct, la distincción está en el uso de parentesis.
        self.width > 0
    } // Normalmente usado para getters.

    // Metodos con más argumentos

    fn can_hold(&self, other: &Rectangle) -> bool { // Si se quiere usar más de un argumento, se continúa agregando después del &self.
        self.width > other.width && self.height > other.height
    }
}

// Multiples bloques 'impl' y funciones asociadas al struct
impl Rectangle { // Se pueden tener muchos bloques 'impl' para una sola entidad. Si bien no parece una funcionalidad útil, yo le encuentro utilidad para separar constructores, funciones asociadas y comportamientos de entidad en distintos bloques. Además para la implementación de Traits (Ver capítulo 10 sección "Traits: Defining Shared Behavior")
    fn square(size: u32) -> Self { // Las funciones asociadas son las funciones que son del struct en si y no de una instancia de este, se pueden identificar por la falta de &self como argumento, en este caso es un 'Constructor' de un tipo específico de rectangulo (o bueno, cuadrilátero), que es el cuadrado, se simplifica la creación haciendo que la función solo tome el argumento del tamaño de los lados y que se devuelva el struct ya armado, tal como se hacía con String::from(&str).
        Self { width: size, height: size }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    println!("El area del rectangulo es {:?} pixeles cuadrado", area(&rect1));

    // Añadir funcionalidad útil a través del uso de traits derivados

    dbg!(&rect1); // Imprime info debug en general. Se puede llamar sobre cualquier expresión para observar los resultados.
    println!("rect1 es {:?}", rect1); // Imprime info sobre el tipo del struct y sus valores.

    // Sintaxis de método

    println!("El area del rectangulo calculado por su método es {:?}", rect1.area()); // Acá como se especificó que sea una referencia, el método no se va a consumir la instancia y por lo tanto se puede llamar al método sin necesidad de usar referencias.
    dbg!(&rect1); // Verificar que rect1 aún existe.

    if rect1.width() {
        println!("El rectangulo tiene un ancho diferente a cero, es: {:?}", rect1.width);
    }

    // Metodos con más argumentos

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("¿Puede rect1 contener a rect2? {}", if rect1.can_hold(&rect2) { "Si" } else { "No" });
    println!("¿Puede rect1 contener a rect3? {}", if rect1.can_hold(&rect3) { "Si" } else { "No" });

    // Funciones asociadas
    let sq = Rectangle::square(3); // Crea un cuadrado de 3 de lado.
    dbg!(&sq); // Se va a ver que es una instancia de Rectangle.
}

fn area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}