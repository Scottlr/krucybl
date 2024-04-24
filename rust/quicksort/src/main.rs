use rand;
use rgb::RGB8;
use textplots::{ColorPlot, Chart, Shape};
use std::{thread, time::Duration};

fn main() {
    let array: [i8; 32] = rand::random();
    println!("Unsorted: \n{:?}", array);

    let sorted = quicksort(array.to_vec());
    println!("Sorted: \n{:?}", sorted)
}

fn quicksort(array: Vec<i8>) -> Vec<i8> {
    if array.len() <= 1 {
        return array;
    }
    let pivot = array.len() - 1;
    let (less, more, pivot) = partition(array.to_vec(), pivot);

    let less_sorted = quicksort(less);
    let more_sorted = quicksort(more);
    let sorted = [less_sorted, vec![array[pivot]], more_sorted].concat();

    visualise(&sorted);

    sorted
}

fn partition(array: Vec<i8>, pivot: usize) -> (Vec<i8>, Vec<i8>, usize) {
    //create a ref to a slice of the array where we stop at the element before pivot.
    let scope = &array[0..pivot];

    //x is a reference to the original slice of the array, we need to then derefence it when using it.
    let (less, more): (Vec<i8>, Vec<i8>) = scope.iter().partition(|&x| *x < array[pivot]);

    (less, more, pivot)
}

fn visualise(array: &Vec<i8>) {
    std::process::Command::new("clear").status().unwrap();
    let mut data = Vec::with_capacity(array.len());
    for (index, value) in array.iter().enumerate() {
        data.push((index as f32, *value as f32));
    }

    let mut chart = Chart::new(64, 60, 0.0, 32.0);
    chart.linecolorplot(
        &Shape::Bars(&data), 
        RGB8 { r: 255, g: 0, b: 0 }
    ).display();
    thread::sleep(Duration::from_millis(333));
}
