use::std::env;
use::std::process;

use minigrep::Config;

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::build(&args).unwrap_or_else(|error| {
        eprintln!("Problema parseando los argumentos: {error}");
        process::exit(1);
    });

    println!("Buscando: {}", config.query);
    println!("En el archivo: {}", config.file_path);

    if let Err(e) = minigrep::run(config) {
        eprintln!("Error de aplicación: {e}");
        process::exit(1);
    }
}

