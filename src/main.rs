use std::io;

fn main() {

	loop {
	    let opcion = menu();

	    if opcion == 3 {
	    	println!("Adiós.");
	    	break;
	    }

	    println!("Introduce la cantidad que deseas convertir: ");
	    let convertir = leer_entrada();

	    if opcion == 1 {
	    	let resultado = fahrenheit(convertir);
	    	println!("El resultado es: {} °F", resultado);
	    } else if opcion == 2 {
	    	let resultado = celsius(convertir);
	    	println!("El resultado es: {} °C", resultado);
	    }
	    println!("\n");
	}
}

fn menu() -> u8 {
	println!("🌡 TRANSFORMADOR DE TEMPERATURA 🌡");
	println!("Introduce tu opción:");
	println!("1. Celsius a Fahrenheit.");
	println!("2. Fahrenheit a Celsius.");
	println!("3. Salir.");

	let opcion = leer_entrada() as u8;
	opcion
}

fn leer_entrada() -> f64 {
	let mut opcion = String::new();
	loop {
		io::stdin().read_line(&mut opcion)
			.expect("Fallo al leer la entrada.");
		let opcion = match opcion.trim().parse() {
		    Ok(num) => num,
		    Err(_) => {
		    	println!("Has ingresado algo no válido.");
		    	continue;
		    }
		};
		return opcion;
	}
}

fn celsius(f: f64) -> f64 {
	5.0 * (f - 32.0) / 9.0
}

fn fahrenheit(c: f64) -> f64 {
	(9.0 * c / 5.0) + 32.0
}