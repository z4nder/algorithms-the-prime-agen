use std::result;

fn main() {
    let list = vec!(1,3,7,8,9,10,11);
    let search = 3;

    let result = binary_search(list, &search).expect("Not find");
}

fn binary_search(list: Vec<i32>, search: &i32) -> Option<usize>{
    let mut bottom = 0;
    
    let mut top = list.len();

    while bottom <= top {
        let middle = (top + bottom) / 2;

        if let Some(current) = list.get(middle) {
            if current == search {
                return Some(middle);
            }
            if current > search {
                top = middle - 1
            }
            if current < search {
                bottom = middle + 1
            }
        }
    }
    

    None
}

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
            assert_eq!(expected, binary_search(input, &target))
        }
    }
}