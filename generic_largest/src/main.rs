fn largest_copy<T:PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn largest_clone<T:PartialOrd + Clone>(list: &[T]) -> T {
    let mut largest = list[0].clone();
    for item in list {
        if *item > largest {
            largest = item.clone();
        }
    }
    largest
}

fn largest<T:PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let char_list = vec!['y', 'm', 'a', 'q'];

    assert_eq!(largest_copy(&number_list), 100);
    assert_eq!(largest_copy(&char_list), 'y');

    assert_eq!(largest_clone(&number_list), 100);
    assert_eq!(largest_clone(&char_list), 'y');

    assert_eq!(*largest(&number_list), 100);
    assert_eq!(*largest(&char_list), 'y');
}
