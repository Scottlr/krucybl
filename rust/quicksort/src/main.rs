use rand;
extern crate graphing;

const SLEEP_DURATION: u64 = 500;

fn main() {
    let array: [i8; 32] = rand::random();
    let sorted = quicksort(array.to_vec(), true);

    let start_time = std::time::Instant::now();
    quicksort(array.to_vec(), false);
    let elapsed_time = start_time.elapsed();
    
    //Performance check on a 32 is redundant, but regardless.
    println!("Sorted Array: \n{:?}", sorted);
    println!(
        "Time taken to QuickSort: {}ns. {}ms", 
        elapsed_time.as_nanos(), 
        elapsed_time.as_millis()
    );
}

fn quicksort(array: Vec<i8>, visualise_sorting: bool) -> Vec<i8> {
    if array.len() <= 1 {
        return array;
    }
    let pivot = array.len() - 1;
    let (less, more, pivot) = partition(array.to_vec(), pivot);

    let less_sorted = quicksort(less, visualise_sorting);
    let more_sorted = quicksort(more, visualise_sorting);
    let sorted = [less_sorted, vec![array[pivot]], more_sorted].concat();

    if visualise_sorting {
        graphing::visualise(&sorted, SLEEP_DURATION);
    }

    sorted
}

fn partition(array: Vec<i8>, pivot: usize) -> (Vec<i8>, Vec<i8>, usize) {
    //create a ref to a slice of the array where we stop at the element before pivot.
    let scope = &array[0..pivot];

    //x is a reference to the original slice of the array, we need to then derefence it when using it.
    let (less, more): (Vec<i8>, Vec<i8>) = scope.iter().partition(|&x| *x < array[pivot]);

    (less, more, pivot)
}