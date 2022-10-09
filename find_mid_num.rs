use rand::Rng;
 
// 中位数
pub fn find_mid_num(vec: &Vec<i32>) -> f64 {
    let mut local = vec.clone();
    local.sort_by(|a, b| a.partial_cmp(b).unwrap());
    if local.len() % 2 == 0 {
        (local.get(local.len() / 2 - 1).unwrap() + local.get(local.len() / 2).unwrap()) as f64 / 2.0
    } else {
        *local.get((local.len()) / 2).unwrap() as f64
    }
}
 
pub fn run_find_mid_num() {
    let mut arr = vec![];
    for _ in 0..10 {
        arr.push(rand::thread_rng().gen_range(1..=100));
    }
    println!("{}", find_mid_num(&arr));
}
 
fn main() {
    run_find_mid_num();
}
