// Definiendo traits
pub mod aggregator {
    use std::fmt::Display;

	pub trait Summary { // Un trait es una entidad que solo contiene métodos en su interior.
		
		fn summarize_author(&self) -> String; // Si no se implementa un comportamiento, quien implemente el trait está obligado a implementarlo.

		// Implementaciones por defecto
		fn summarize(&self) -> String {
			String::from("Leer más...") // Si se implementa un comportamiento al propio trait, entonces ese será su comportamiento por defecto.
		}
	}

	pub struct NewsArticle {
		pub headline: String,
		pub location: String,
		pub author: String,
		pub content: String, 
	}

	impl Summary for NewsArticle {
		fn summarize_author(&self) -> String {
		    String::from("Leer más del autor...")
		}

		fn summarize(&self) -> String {
			format!(
				"{}, by {} ({})",
				self.headline,
				self.author,
				self.location
			)
		}
	}

	pub struct Tweet {
		pub username: String,
		pub content: String,
		pub reply: bool,
		pub retweeet: bool,
	}

	impl Summary for Tweet {
		fn summarize_author(&self) -> String {
		    format!("@{}", self.username)
		}

		fn summarize(&self) -> String {
			format!("{}: {} (Leer más...)", self.username, self.content)
		}
	}

	// Traits como parametros
	pub fn notify<T: Summary>(item: &T) { // Este método solo lo pueden llamar quienen implementen Summary
		// Otra forma de ponerlo sería notify(item: &impl Summary)

		println!("¡Ultima noticia! {}", item.summarize())
	} 

	// Tambien se puede pedir muchos traits usando '+' y se puede definir los traits que tiene que implementar usando 'where'.
	pub fn alguna_funcion<T>(item: &T) -> i32 
	where T: Summary + Display {
		println!("{}", item);
		5
	}

	// Retornar implementadores de traits
	pub fn returns_summarizable() -> impl Summary {
			Tweet { // Solo se puede retornar un solo tipo de implementador por función.
				username: String::from("Hello"),
				content: String::from("How are you?"),
				reply: false,
				retweeet: false,
		}
	}
}

pub mod bounds {
    use std::fmt::Display;

	// Usar Trait bounds para implementar métodos de manera condicional
	pub struct Pair<T> {
		x: T,
		y: T,
	}

	impl<T> Pair<T> { // Los métodos dentro de esta impl van a ser generales para todos los tipos de datos.
		pub fn new(x: T, y: T) -> Self {
			Self { x, y }
		}
	}

	impl<T: Display + PartialOrd> Pair<T> { // Este impl solo lo va a poder usar quienes implementen estos traits
	    pub fn cmp_display_greater_of_pair(&self) {
	    	if self.x > self.y {
	    		println!("El mayor de los dos es x: {}", self.x);
	    	} else if self.x < self.y {
	    		println!("El mayor de los dos es y: {}", self.y);
	    	} else {
	    		println!("Ambos tienen el mismo valor: {}", self.x)
	    	}
	    }
	}
}