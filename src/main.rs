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

        let readline = match readline.trim().parse::<u32>() {
            Ok(num) => num,
            Err(_) => {
                println!("No es un número");
                continue
            }
        };

        if readline == 3 {
            break;
        }

        let cantidad = rl.readline("Introduce la cantidad que deseas convertir: ");

        let cantidad = match cantidad {
            Ok(line) => line,
            Err(err) => {
                println!("Un error ha ocurrido: {:?}", err);
                break
            },
        };

        let cantidad = match cantidad.trim().parse::<f64>() {
            Ok(num) => num,
            Err(_) => {
                println!("No es un número");
                break
            },
        };

        if readline == 1 {
            println!("{} °C a °F son: {}", cantidad, celsius(cantidad));
        } else if readline == 2 {
            println!("{} °F a °C son: {}", cantidad, fahrenheit(cantidad));
        }
	}
}

fn celsius(f: f64) -> f64 {
    5.0 * (f - 32.0) / 9.0
}

fn fahrenheit(c: f64) -> f64 {
    (9.0 * c / 5.0) + 32.0
}