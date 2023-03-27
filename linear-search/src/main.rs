fn main() {
    let list = vec!(1,4,7,10,12,15,18,20);
    let search = 12;
   
    let result = linear_search(list, &search).expect("Value not found");

    println!("The search {} is finded at position {}", search, result);
}

fn linear_search(list: Vec<i32>, search: &i32) -> Option<usize>{

    for (i, x) in list.iter().enumerate() {
        if search == x {
            return Some(i);
        }
    }

    None
}
