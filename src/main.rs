extern crate rustyline;

use rustyline::error::ReadlineError;
use rustyline::Editor;

fn main() {
	let mut rl = Editor::<()>::new();

	loop {
		println!("🌡 TRANSFORMADOR DE TEMPERATURA 🌡");
		println!("Introduce tu opción:");
		println!("1. Celsius a Fahrenheit.");
		println!("2. Fahrenheit a Celsius.");
		println!("3. Salir.");

        let readline = rl.readline("Tu opción: ");

        let readline = match readline {
            Ok(line) => line,
            Err(err) => {
                println!("An error has ocurred: {:?}", err);
                break
            }
        };

        if readline == "3" {
            break;
        }
	}
}