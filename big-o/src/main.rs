fn main() {
    let vec = vec!(1,2,3,4,5);

    println!("Vec SUM 1 {}", vec_sum_one(&vec));

    println!("Vec SUM 2 {}", vec_sum_two(&vec));
}

// BigO -> O(n)
fn vec_sum_one(n: &Vec<i32>) -> i32{
    let mut sum = 0;

    for item in n.iter() {
        sum += item;
    }

    sum
}

// BigO -> O(n^2)
fn vec_sum_two(n: &Vec<i32>) -> i32{
    let mut sum = 0;

    for item_one in n.iter() {
        sum += item_one;
        for item_two in n.iter() {
            sum += item_two;
        }
    }

    sum
}


// BigO -> O(n log n) 
// Quick Sort

// BigO ->O(log n)
// Binary search trees
