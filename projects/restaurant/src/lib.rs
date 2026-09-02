mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {} // En este caso se debió exponer al publico esta función ya que se encuentra en un scope "inferior" a quién lo usa. front_of_house e eat_at_restaurant() son "hermanas" ya que se encuentran en el mismo scope y están expuestas una a la otra

        fn seat_at_table() {} // En el mismo sentido, esta función es hermana de add_to_waitlist()
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

// Accediendo a cosas de modulo y exponiendolas con la palabra clave pub
pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist(); // Path absoluto, es el que accede a una cosa a partir de especificar cada entidad que compone al modulo.
}

