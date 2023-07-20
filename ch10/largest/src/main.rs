fn main() {
    let list1 = vec![1, 43, 3, 5435, 23, 23];
    let list2 = vec!['a', 'b', 'c', 'd'];

    println!("Largest number in list1 is: {}", largest(&list1));
    println!("Largest number in list2 is: {}", largest(&list2));
}

fn largest<T: Ord>(list: &[T]) -> &T {
    list.iter().max().unwrap()
}
