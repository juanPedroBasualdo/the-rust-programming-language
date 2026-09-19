use adder;

// De manera idiomática, los tests unitarios se escriben dentro de los módulos y si se quiere hacer tests de integración se usa una carpeta llamada tests/ que rust identifica de manera automática como la carpeta de los test, por lo que no hay necesidad de prefijar un módulo con #[cfg(test)]

mod common; // Se pueden usar submodulos para mantener el orden en la carpeta de test de integración si así se desea.

#[test]
fn it_adds_two() {
	common::setup();
	assert_eq!(4, adder::add_two(2));
}