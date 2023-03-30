fn main() {
    let list = vec!(1, 3, 7, 4, 10, 6, 22, 30);
    let response = bubble_sort(list);

    println!("response {:?}", response);
}

fn bubble_sort(mut list: Vec<i32>) -> Vec<i32>{
    for i in  0..list.len(){
        for j in  0..list.len() -1 -i{
            if list[j] > list[j+1] {
                list.swap(j, j+1);
            }
        }
    }

    list
}
