use std::collections::HashSet;

fn remove_duplicates(some_vec: &mut Vec<i32>) {
    let set: HashSet<_> = some_vec.drain(..).collect();
    some_vec.extend(set.into_iter())
}

// Otra opción devolviendo el valor
// fn remove_duplicates(mut some_vec: Vec<i32>) -> Vec<i32> {
//     let set: HashSet<_> = some_vec.drain(..).collect();
//     set.into_iter().collect()
// }

fn main() {
    let mut random_vec = Vec::from([1, 2, 3, 4, 5, 22, 3, 1, 2, 21, 123, 421, 1, 12, 3, 4, 5]);

    println!("{:?}", random_vec);
    remove_duplicates(&mut random_vec);
    println!("{:?}", random_vec)

    // Con la alternativa
    // println!("{:?}", random_vec);
    // println!("{:?}", remove_duplicates(&mut random_vec));
}
