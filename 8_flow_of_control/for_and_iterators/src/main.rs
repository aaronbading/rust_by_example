fn main() {
    let names = vec!["Bob", "Frank", "Ferris"];

    for name in names.iter() {
        match name {
            &"Ferris" => println!("There is a rustacean among us!"),
            // TODO ^ Try deleting the & and matching just "Ferris"
            _ => println!("Hello {}", name),
        }
    }
    
    println!("names: {:?}", names);
}

// into_iterator
// this means the names vector is consumed and no longer usable after the loop
// fn main() {
//     let names = vec!["Bob", "Frank", "Ferris"];

//     for name in names.iter() {
//         match name {
//             &"Ferris" => println!("There is a rustacean among us!"),
//             // TODO ^ Try deleting the & and matching just "Ferris"
//             _ => println!("Hello {}", name),
//         }
//     }
    
//     println!("names: {:?}", names);
// }


//iner_mut
// This mutably borrows each element of the collection, allowing for the collection to be modified in place.
// fn main() {
//     let mut names = vec!["Bob", "Frank", "Ferris"];

//     for name in names.iter_mut() {
//         *name = match name {
//             &mut "Ferris" => "There is a rustacean among us!",
//             _ => "Hello",
//         }
//     }

//     println!("names: {:?}", names);
// }
