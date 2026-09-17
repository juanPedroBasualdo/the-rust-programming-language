// Creando Tipos para la validación de errores.
pub struct Guess {
	value: i32,
}

impl Guess {
	pub fn new(value: i32) -> Guess {
		if value < 1 || value > 100 { // Un tipo de dato puede tener reglas internas que ayuden con la validación de errores.
			panic!(
				"El valor de prueba debe estar entre 1 y 100, se obtuvo: {value}"
			);
		}

		Guess { value }
	}

	pub fn value(&self) -> i32 {
		self.value
	}

}