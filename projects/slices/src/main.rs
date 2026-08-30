fn main() {
    let s = String::from("Hola mundo de rust");

    // Slices de string
    let first_word = first_word(&s);
    let second_word = second_word(&s);

    println!("La frase '{s}' tiene como primera palabra '{first_word}' y como segunda palabra '{second_word}'.");

    // Otros slices
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..3]; // Slice de un arreglo
    assert_eq!(slice, &[2, 3]);
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i]
        }
    }

    &s[..]
}

fn second_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    let mut start: usize = 0;
    for(i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            if start == 0 {
               start = i + 1; 
            } else {
                return &s[start..i]
            }
        }
    }

    if start != 0 {
        return &s[start..];
    }
    &s[..]
}