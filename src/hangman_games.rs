use rand::seq::SliceRandom;
use rand::thread_rng;
use std::fs;
use std::io::stdin;

const DATA_PAT: &str = "./src/utils/data_hangman_game.txt";
const LIVES: u8 = 10;

fn select_word() -> String {
    let data = fs::read_to_string(DATA_PAT).expect("Something went wrong reading the file.");
    let words: Vec<&str> = data.split("\n").collect();

    let mut rng = thread_rng();
    let word = words
        .choose(&mut rng)
        .expect("No hay data para elegir un valor aleatorio.");

    word.to_string().to_lowercase()
}

fn main() {
    let word = select_word();
    let mut lives = LIVES;
    let mut word_guess_list: Vec<String> = word.chars().map(|_c| String::from("_")).collect();
    let mut words_inserted: Vec<String> = Vec::new();

    // * La linea de arriba es lo mismo que lo que está abajo
    // let mut word_guess_list: Vec<char> = Vec::new();

    // for _i in 0..word.len() {
    //     word_guess_list.push('_');
    // }

    println!("¡El ahorcado!");

    loop {
        println!("Vidas restantes: {}", lives);
        if words_inserted.len() > 0 {
            println!("Palabras o letras ingresadas: {}", words_inserted.join(" "));
        }
        println!("{} \n", word_guess_list.join(""));

        println!("Ingresa una letra o una palabra:");
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let guess_word = input.trim().to_lowercase();

        if guess_word.is_empty() || !guess_word.chars().all(|c| c.is_alphabetic()) {
            continue;
        }

        if guess_word.len() > 1 {
            if guess_word == word {
                println!("¡Ganaste! La palabra es: {}", word);
                break;
            } else {
                words_inserted.push(guess_word);
                lives -= 1;

                if lives <= 1 {
                    println!("Has perdido... La palabra era: {}", word);
                    break;
                }
                continue;
            }
        }

        if word.contains(guess_word.as_str()) {
            let word_bytes = word.as_bytes();
            let guess_word_byte = guess_word
                .as_bytes()
                .get(0)
                .expect("Error al momento de obtener el elemento ingresado por el usuario.");

            for (pos, byte) in word_bytes.iter().enumerate() {
                if byte == guess_word_byte {
                    word_guess_list[pos] = guess_word.clone();
                }
            }
        }

        if word_guess_list.join("") == word {
            println!("¡Ganaste! La palabra es: {}", word);
            break;
        }

        if !words_inserted.contains(&guess_word) {
            words_inserted.push(guess_word);
        }

        if lives <= 1 {
            println!("Has perdido... La palabra era: {}", word);
            break;
        }

        lives -= 1;
        println!();
    }
}
