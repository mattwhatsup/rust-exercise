use std::collections::HashMap;
 
use rand::Rng;
 
pub fn find_mode_num(arr: &Vec<i32>) -> Vec<i32> {
    let mut ret: Vec<i32> = vec![];
    let mut pairs: HashMap<i32, i32> = HashMap::new();
    let mut max = 0;
    for i in arr {
        let count = pairs.entry(*i).or_insert(0);
        *count += 1;
        if *count > max {
            max = *count;
        }
    }
    for (key, value) in pairs.iter() {
        if *value == max {
            ret.push(*key);
        }
    }
    println!("{:?}", pairs);
    ret
}
 
pub fn run_find_mode_num() {
    let mut arr = vec![];
    for _ in 0..100 {
        arr.push(rand::thread_rng().gen_range(1..=10));
    }
    println!("{:?}", find_mode_num(&arr));
}
 
fn main() {
    run_find_mode_num();
}
