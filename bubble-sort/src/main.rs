fn main() {
    let list = vec!(1, 3, 7, 4, 10, 6, 22, 30);
    let response = bubble_sort(list);

    println!("response {:?}", response);
}

fn bubble_sort(mut list: Vec<i32>) -> Vec<i32>{
    let mut swapped = true;
    
    while swapped {
        swapped = false;
        for i in 1..list.len() {
            if list[i - 1] > list[i] {
                list.swap(i - 1, i);
                swapped = true
            }
        }
    }

    list
}
