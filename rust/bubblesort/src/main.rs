use rand;
use rgb::RGB8;
use textplots::{ColorPlot, Chart, Shape};
use std::{thread, time::Duration};

fn main() {
    let array: [i8; 32] = rand::random();
    let sorted = bubble_sort(array.to_vec());
    println!("Sorted: \n{:?}", sorted)


}

fn bubble_sort(mut array: Vec<i8>) -> Vec<i8> {
    let max = array.len() - 1;
    for _ in 0..max {
        let mut swapped = false;
        for comparator in 0..(max) {
            if array[comparator] > array[comparator + 1] {
                swapped = true;
                array.swap(comparator, comparator + 1)
            }
        }

        visualise(&array);

        if !swapped {
            break;
        }
    }
    array
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