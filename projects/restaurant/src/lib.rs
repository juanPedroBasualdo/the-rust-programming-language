mod front_of_house; // Se puede separar cada modulo en un archivo diferente.

// Accediendo a cosas de modulo y exponiendolas con la palabra clave pub
pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist(); // Path absoluto, es el que accede a una cosa a partir de especificar cada entidad que compone al modulo.
}

// Paths relativos y la palabra clave 'super'
pub fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        super::deliver_order(); // Como fix_incorrect_order() está en un scope inferior a la función que usa, se puede llamar a "super" para poder usar cualquier cosa que se encuentre en un scope superior a ella. La relación se llama madre e hija, como fix_incorrect_order() es hija de deliver_order(), está expuesta a ella y por lo tanto puede usarla.
    }

    fn cook_order() {}

    // Exponiendo structs y enums.
    pub struct Breakfast { // Si un struct se hace público, sus campos no necesariamente son públicos. Esto tiene sentido ya que se puede querer hacer publico solo al struct para poder usar sus comportamientos sin preocuparse de los campos que utiliza.
        pub toast: String, // Para hacer posible el acceso de cierto campo, hay que hacerlo público.
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast { // Los constructores en general se hacen públicos, en las 'impl' cada método elige si exponerse o no.
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("duraznos"),
            }
        }
    }

    pub enum Apetizer { // Si un enum se hace público, todas sus variantes son públicas. Si no lo fueran se obligaría en todo momento a usar el catch-all o simplemente no se debería de necesitar ser exhaustivo en el matching.
        Soup,
        Salad,
    }
}

pub fn deliver_to_front() {
    let mut meal = back_of_house::Breakfast::summer("Salvado"); // Se ordena una tostada en el verano de harina de salvado.

    meal.toast = String::from("Trigo"); // Se cambia de opinion, ahora se quiere que la tostada sea de harina de trigo.

    println!("Sale una tostada de {}", meal.toast); // pub hace posible la modificación y el acceso. Lo que no se puede hacer acá es 'meal.seasonal_fruit = String::from("moras")' porque ese campo es privado. 
}

// Traer paths a scope con la palabra clave 'use'

use crate::front_of_house::hosting; // Trae todo lo que está dentro del scope front_of_house->hosting.

pub fn serve_to_go() {
    hosting::add_to_waitlist(); // Ahora no hay que especificar tooodo el path sino lo que no se especificó en el use
    // Es como ahorrarse el 'crate::front_of_house::' porque ya se especificó en el 'use'. Usar 'use' verifica privacidad también.
}

// Crear Paths idiomáticos con 'use'

use crate::front_of_house::hosting::add_to_waitlist; // Esta es la forma correcta de traer un método o función a un scope, ya que al invocarlo solo va a ser necesario llamar a la función (en este caso) 

use crate::back_of_house::Breakfast; // Para structs y enums se usa hasta el nombre del enum, ya que por ahí una persona va a querer instanciar a una de estas entidades, no solo usar sus funciones asociativas, que es buena práctica usarlas con el prefijo del nombre del struct/enum. Una única excepción a la regla es si dos structs/enums usan el mismo nombre para alguna entidad, en ese caso hay dos alternativas:

use std::fmt;
use std::io;

fn _funct1() -> fmt::Result {
    Ok(())
}

fn _funct2() -> io::Result<i32> {
    Ok(0)
} // Usar un pathing más especifico es la primer opción

use std::fmt::Result;
use std::io::Result as IOResult; // Usar Aliasing con la palabra clave 'as' para hacer diferenciación.


// Usar Paths anidados para limpiar listas largas.
use std::{cmp::Ordering, io}; // Se pueden traer más de una librería por llamada a 'use'.

use std::io::{self, Write}; //Esto trae la librería io y lo que incluye Write.

use std::collections::*; // Esto trae todo lo de adentro de collections.