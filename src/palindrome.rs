fn is_palindrome(text: &str) -> bool {
    let text = text.trim().to_lowercase();
    let text_rev: String = text.chars().rev().collect();
    text == text_rev
}

fn main() {
    println!("Ingresa una palabra para saber si es un palíndromo:");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();

    if is_palindrome(&input) {
        println!("Si es un palíndromo.")
    } else {
        println!("No es un palíndromo.")
    }
}
