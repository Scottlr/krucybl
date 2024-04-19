use rand;

fn main() {
    let array: [i8; 32] = rand::random();
    println!("{:?}", array);

}

fn quicksort(array: Vec<i8>) -> Vec<i8> {
    let pivotIndex = array.len();
    let comparisonIndex = 0;
    
    array
}