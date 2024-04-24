use rgb::RGB8;
use textplots::{ColorPlot, Chart, Shape};
use std::{thread, time::Duration};


pub fn visualise(array: &Vec<i8>, sleep_duration: u64) {
    std::process::Command::new("clear").status().unwrap();
    let data_points = format_to_data_series(array);
    visualise_graph(data_points);
    thread::sleep(Duration::from_millis(sleep_duration));
}


fn format_to_data_series(array: &Vec<i8>) -> Vec<(f32, f32)> {
    array
        .iter()
        .enumerate()
        .map(|(index, value)| (index as f32, *value as f32))
        .collect()
}

fn visualise_graph(data_points: Vec<(f32, f32)>) {
    Chart::new(64, 60, 0.0, 32.0).linecolorplot(
        &Shape::Bars(&data_points), 
        RGB8 { r: 255, g: 0, b: 0 }
    ).display();
}