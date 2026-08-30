fn main() {

    // Referencias y funciones
    let s1 = String::from("hola");
    let len = calculate_length(&s1); // Si no se usase una referencia, el valor de s1 se lo quedaría la función

    println!("La longitud de '{}' es: {}", s1, len);

    // Referencias mutables
    let mut s = String::from("Hola");
    println!("El string 's' es: '{s}'");
    change(&mut s);

    println!("El string 's' ahora es: '{s}'");

    // Prestamo en referencias inmutables
    let r = String::from("Hola");
    let r1 = &r;
    let r2 = &r; // Como son inmutables, se puede 'prestar' cuantas veces sea necesario.

    println!("El string 'r' es compartido entre dos Referencias: r1: {r1}, r2: {r2}");

    // Prestamo en referencias mutables
    let mut t = String::from("Chau");
    println!("El string 't' es: {t}");
    let t1 = &mut t;
    t1.push_str(", mundo"); // Se permite la modificación de t por medio de t1 ya que el prestamo se hace a una sola variable.

    println!("El string 't' es ahora: {t}");

    let mut u = String::from("Como");
    println!("El string 'u' es: {u}");
    
    let u1 = &u;
    let u2 = &u; // Acá no hay problema si prestamos dos veces un valor, pero como hay más de un prestamo no se puede designar ninguna referencia mutable hasta hacer uso de TODAS LAS REFERENCIAS que estan siendo prestadas.

    println!("Referencias de 'u' previo a la mutación: u1:'{u1}', u2:'{u2}'"); // Acá u1 y u2 sueltan el valor de u por lo que ahora puede ser mutado por una referencia mutable.

    let u_mut = &mut u;
    u_mut.push_str(" andas?");
    println!("Ahora 'u' es: {u}");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(", mundo")
}