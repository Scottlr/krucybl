use rand;

fn main() {
    let array: [i32; 32] = rand::random();
    println!("Unsorted: \n{:?}", array);

    let sorted = quicksort(array.to_vec());
    println!("Sorted: \n{:?}", sorted)
}