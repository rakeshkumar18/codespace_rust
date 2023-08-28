// Hash Maps

// use std::{collections::HashMap, hash::Hash};

// fn main() {
//     let mut person:HashMap<&str,i32> = HashMap::new();

//     person.insert("Rakesh", 29);
//     person.insert("Rocky", 32);
//     person.insert("rc2d", 27);

//     println!("The age is {:?} ", person.get("Rakesh").unwrap());
//     if person.contains_key("rc2d" ) {
//         println!("The value exist");

//     } else{
//         println!("The value does not exist");
//     }

// }


fn main() {
    let some_vec = vec![3,4,5,12,3,3,4,5,2,7,8,9,90];
    let mut freq_vec:HashMap <i32, u32> = HashMap::new();

    for i in &some_vec {
        let freq: &mut u32 = freq_vec.entry(*i).or_insert(0);
        *freq += 1;
    }
   println!("{:?}", freq_vec); 
}