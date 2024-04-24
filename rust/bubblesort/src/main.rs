use rand;

fn main() {
    let array: [i8; 32] = rand::random();
    println!("Unsorted: \n{:?}", array.to_vec());

    let sorted = bubble_sort(array.to_vec());
    println!("Sorted: \n{:?}", sorted)
}

fn bubble_sort(mut array: Vec<i8>) -> Vec<i8> {
    let max = array.len() - 1;
    for index in 0..max {
        let mut swapped = false;
        for comparator in 0..(max) {
            if array[comparator] > array[comparator + 1] {
                swapped = true;
                array.swap(comparator, comparator + 1)
            }
        }
        if !swapped {
            break;
        }
    }
    array
}