// use std::collections::LinkedList;
#[derive(Debug)]
struct LinkedList<T> {
    value: T,
    next: Option<Box<LinkedList<T>>>
}
// struct DoubleLinkedList<T> {
//     value: T,
//     next: Option<Box<DoubleLinkedList<T>>>,
//     previous: Option<Box<DoubleLinkedList<T>>>
// }

impl <T> LinkedList<T> {
    fn append(self, value: T) -> LinkedList<T>{
        LinkedList {
            value,
            next: Some(Box::new(self))
        }
    }
    fn map(mut self, f: fn(T) -> T) -> LinkedList<T>
    {
        self.value = f(self.value);
        
        match self.next {
            None => self,
            Some(node) => {
                node.map(f)
            }
        }
    }
}

fn main() {
    let  list = LinkedList {
        value: 10,
        next: None
    };

    let list = list.append(1);
    let list = list.append(7);
    let list = list.append(5);

    let fn_to_map = |x: i32| x + 1;
    let mapped_list = list.map(fn_to_map);

    println!("Lista => {:#?}", mapped_list)
}
