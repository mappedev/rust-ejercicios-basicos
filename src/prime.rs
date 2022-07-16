fn is_prime(num: u64) -> bool {
    let mut result: Vec<u64> = Vec::new();

    for i in 2..num {
        if num % i == 0 {
            result.push(i)
        }
    }

    result.len() == 0
}

fn main() {
    println!("{}", is_prime(7));
}
