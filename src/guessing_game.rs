use rand::Rng;
use std::{cmp::Ordering, io};

fn main() {
    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("Adivina cuál es el número secreto desde 1 al 100.");

    loop {
        println!("Inserta un número: ");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("No se pudo leer el dato ingresado...");

        let guess = match guess.trim().parse::<u32>() {
            Ok(number) => number,
            Err(_) => continue,
        };

        match secret_number.cmp(&guess) {
            Ordering::Greater => println!("Más arriba..."),
            Ordering::Less => println!("Más abajo..."),
            Ordering::Equal => {
                println!("¡Ganaste!");
                break;
            }
        }
    }
}
