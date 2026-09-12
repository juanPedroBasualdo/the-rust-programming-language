fn main() {
    // Vectores
    let _v: Vec<i32> = Vec::new();
    let _v = vec![1, 2, 3]; // Tipo inferido por el valor que contiene.

    // Insertar valores a los vectores
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7); // Tipo inferido por el valor que agrega.

    // Leer valores de vectores
    let mut v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2]; // Por indice. Los indices van del 0-n. Devuelve referencia al propio valor.
    println!("El valor en 3er lugar es: {third}");

    let third: Option<&i32> = v.get(2); // Por get. Devuelve Option que contiene referencia al valor si Some o None.
    match third {
        Some(third) => println!("El valor en el 3er lugar es: {third}"),
        None => println!("No hay valor en ese índice.")
    } // Como todo match se debe garantizar usar todas las variantes;

    {
        let first = &v[0];
        println!("El valor en primera posición es {first}")
    }
    
    v.push(6); // Al hacer push, se hace un borrow mutable, ya que si el vector tiene que cambiar de capacidad se debe copiar los valores a un nuevo vector cuya capacidad es mayor, por eso si first no se dropea antes del push, el borrow checker da pánico al compilar ya que se tiene un borrow mutable y uno inmutable, por lo que el valor que presta a first puede estar apuntando a memoria invalida.

    // Usando enums para guardar distintos tipos.
    enum SpreadsheetCell {
        Int(i32),
        Float(f32),
        Text(String),
    }

    let _row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.2),
    ]; // Como las variantes de un enum pueden llevar distintos tipos, se pueden guardar distintos tipos en un vector usando variantes de un mismo enum.

    // Soltando vectores y sus elementos
    {
        let _v = vec![1, 2, 3, 4, 5];

        // Hacer cosas con v
    } // <- v sale de scope y por lo tanto se dropea junto con sus elementos.

}