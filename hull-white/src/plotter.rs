
extern crate ndarray;
extern crate plotters;
extern crate plotpy;


use ndarray::prelude::*;
use plotters::prelude::*;
use plotpy::{generate3d, Plot, StrError, Surface, Curve};


pub fn plot_multi_curve(t: Array1<f64>, r: Array2<f64>) {
    let root = BitMapBackend::new("plot.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .caption("Short Rate", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_ranged(0f64..1f64, -0.1f64..0.1f64)
        .unwrap();

    chart.configure_mesh().draw().unwrap();

    for i in 0..r.shape()[0] {
        let data: Vec<(f64, f64)> = t.iter().zip(r.row(i).iter()).map(|(x, y)| (*x, *y)).collect();
        chart.draw_series(LineSeries::new(data, &BLACK)).unwrap();
    }
}

pub fn plot_single_curve(t: Array1<f64>, r: Array1<f64>) {
    let root = BitMapBackend::new("plot.png", (640, 480)).into_drawing_area();
    root.fill(&WHITE).unwrap();

    let mut chart = ChartBuilder::on(&root)
        .caption("Short Rate", ("sans-serif", 50).into_font())
        .margin(5)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .build_ranged(0f64..1f64, -0.1f64..0.1f64)
        .unwrap();

    chart.configure_mesh().draw().unwrap();

    let data: Vec<(f64, f64)> = t.iter().zip(r.iter()).map(|(x, y)| (*x, *y)).collect();
    chart.draw_series(LineSeries::new(data, &BLACK)).unwrap();
}

pub fn plot_single_graph(x: Array1<f64>, y: Array1<f64>) {
    let mut curve = Curve::new();
    curve.set_label("single curve");

    let mut plot = Plot::new();
    plot.add(&curve).grid_labels_legend("time", "short rate");
    plot.show("./img.png");
}