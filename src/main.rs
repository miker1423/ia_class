use std::collections::VecDeque;

fn main() {
    let mut initial = vec![1, 8, 2, 3, 5, 6, 7, 4, 0];
    println!("{:?}", &initial);
    initial.swap(0, 5);
    println!("arr: {:?}", &initial);
    let zero_index = find_zero(&initial);
    if let Some(index) = zero_index {
        println!("{}", index);
    }
}

fn find_zero(array: &Vec<i32>) -> Option<usize> {
    for i in 0..array.len() {
        if array[i] == 0 {
            return Some(i)
        }
    }
    return None;
}
