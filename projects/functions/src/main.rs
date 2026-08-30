fn main() {
    println!("Hello, world!");
    otra_funcion();
    funcion_con_parametros(5);
    imprimir_medidas_y_unidad(5, 'h');
}

fn otra_funcion() {
    println!("Otra función");
}

fn funcion_con_parametros(x: i32) {
    println!("El valor de x es: {x}");
}

fn imprimir_medidas_y_unidad(medida: i32, unidad: char) {
    println!("El valor medido es: {medida}{unidad}");
}