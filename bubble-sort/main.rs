fn main() {
    let list = vec!(1,4,7,10,12,15,18,20);
    let search = 12;
   
    let result = linear_search(list, &search).expect("Value not found");

    println!("The search {} is finded at position {}", search, result);
}

// Big O -> O(n)
fn linear_search(list: Vec<i32>, search: &i32) -> Option<usize>{

    for (i, x) in list.iter().enumerate() {
        if search == x {
            return Some(i);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_search() {
        let tests = vec![
            (vec![], 1, None),
            (vec![1], 1, Some(0)),
            (vec![1, 2], 1, Some(0)),
            (vec![1, 2], 2, Some(1)),
            (vec![1, 2, 3], 3, Some(2)),
            (vec![-1, 0, 3, 5, 9, 12], 9, Some(4)),
            (vec![-1, 0, 3, 5, 9, 12], 2, None),
        ];

        for (input, target, expected) in tests {
            assert_eq!(expected, linear_search(input, &target))
        }
    }
}