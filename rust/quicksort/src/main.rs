use rand;

fn main() {
    let array: [i32; 32] = rand::random();
    println!("Unsorted: \n{:?}", array);

    let sorted = quicksort(array.to_vec());
    println!("Sorted: \n{:?}", sorted)
}

fn quicksort(array: Vec<i32>) -> Vec<i32> {
    if array.len() <= 1 {
        return array;
    }
    let pivot = array.len() - 1;
    let (less, more, pivot) = partition(array.to_vec(), pivot);

    let less_sorted = quicksort(less);
    let more_sorted = quicksort(more);

    [less_sorted, vec![array[pivot]], more_sorted].concat()
}

fn partition(array: Vec<i32>, pivot: usize) -> (Vec<i32>, Vec<i32>, usize) {
    //create a ref to a slice of the array where we stop at the element before pivot.
    let scope = &array[0..pivot];

    //x is a reference to the original slice of the array, we need to then derefence it when using it.
    let (less, more): (Vec<i32>, Vec<i32>) = scope.iter().partition(|&x| *x < array[pivot]);

    (less, more, pivot)
}
