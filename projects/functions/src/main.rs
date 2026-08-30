fn main() {
    // Funciones
    println!("Hello, world!");
    otra_funcion();

    // Funciones con parametros
    funcion_con_parametros(5);
    imprimir_medidas_y_unidad(5, 'h');

    // Funcional (declaraciones y expresiones)
    let _y = {
        let x = 3;
        x // retorna x
    }; // Es lo mismo que hacer 'let y = (let x = 3)', pero rust no permite ese tipo de declaraciones, pues no retornan valores, la manera correcta es crear una función sin nombre con {} y hacer que retorne algún valor para que pueda asignarsele a la variable. En este caso el valor x se pierde luego de salir de la llave porque se usó dentro del contexto de esta función invisible, ver "Understanding Ownership (Cap-4)".
    // En mi opinión: Esto es un fenómeno del paradigma funcional puro y duro, hacer simplemente y = x = 3 no es una opción viable dadas las reglas de Rust de no retornar dentro de declaraciones, solo de expresiones. Esto tiene sus complicaciones pero traen por detrás un montón de ventajas a la hora de manejar memoria.

    println!("El valor de y es: {y}"); // 3

    // Funciones con retorno

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
