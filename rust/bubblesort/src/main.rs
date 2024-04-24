use rand;
extern crate graphing;

const SLEEP_DURATION: u64 = 500;

fn main() {
    let array: [i8; 32] = rand::random();
    let sorted = bubble_sort(array.to_vec(), true);

    let start_time = std::time::Instant::now();
    bubble_sort(array.to_vec(), false);
    let elapsed_time = start_time.elapsed();
    
    //Performance check on a 32 is redundant, but regardless.
    println!("Sorted Array: \n{:?}", sorted);
    println!(
        "Time taken to BubbleSort: {}ns. {}ms", 
        elapsed_time.as_nanos(), 
        elapsed_time.as_millis()
    );
}

fn bubble_sort(mut array: Vec<i8>, visualise: bool) -> Vec<i8> {
    let max = array.len() - 1;
    for _ in 0..max {
        let mut swapped = false;
        for comparator in 0..(max) {
            if array[comparator] > array[comparator + 1] {
                swapped = true;
                array.swap(comparator, comparator + 1)
            }
        }

        if visualise {
            graphing::visualise(&array);
        }

        if !swapped {
            break;
        } 
    }
    array
}