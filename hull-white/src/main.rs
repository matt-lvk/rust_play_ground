extern crate ndarray;
extern crate rand;
extern crate csv;
extern crate ndarray_csv;

use ndarray::prelude::*;
use ndarray::{Array, Array2};
use ndarray_csv::{Array2Reader, Array2Writer};
use rand_distr::{Normal, Distribution};
use rand::prelude::*;
use csv::{ReaderBuilder, WriterBuilder};
use std::fs::File;
use std::error::Error;

mod plotter;

struct HullWhite {
    nsim : i32,
    a: f64,
    b: f64,
    sigma: f64,
    T: usize,
    dt: f64,
}
fn gaussian_number_generator(mean: f64, std_dev: f64) -> f64 {
    let normal: Normal<f64> = Normal::new(mean, std_dev).unwrap();
    let v: f64 = normal.sample(&mut rand::thread_rng());
    return v;
}

fn wiener_lattice_generator(nsim: i32, T: usize, dt: f64) -> Array2<f64> {
    let mut w: Array2<f64> = Array::zeros((nsim as usize, T));
    for i in 0..nsim {
        for j in 0..T {
            w[[i as usize, j]] = gaussian_number_generator(0.0, dt.sqrt());
        }
    }
    return w;
}

fn short_rate(hull_white: HullWhite, w: Array2<f64>) -> Array2<f64> {
    let mut r: Array2<f64> = Array::zeros((hull_white.nsim as usize, hull_white.T));
    for i in 0..hull_white.nsim {
        for j in 1..hull_white.T {
            let prev_r: f64 = r[[i as usize, j - 1]];
            r[[i as usize, j]] = prev_r + (hull_white.a + hull_white.b * prev_r * hull_white.dt)
                                     + hull_white.sigma * w[[i as usize, j]];
        }
    }
    return r;
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Start");
    let hull_white = HullWhite {
        nsim: 1000,
        a: 0.1,
        b: 1.0,
        sigma: 0.1,
        T: 250,
        dt: 0.01,
    };

    let t_linspace: Array1<f64> = Array::linspace(0.0, 1.0, hull_white.T);
    let wiener_mat: Array2<f64> = wiener_lattice_generator(hull_white.nsim, hull_white.T, hull_white.dt);
    let short_rate_array: Array2<f64> = short_rate(hull_white, wiener_mat);

    let expected_rate: Array1<f64> = short_rate_array.mean_axis(Axis(0)).unwrap();

    println!("t: {}", short_rate_array);

    {
        let file = File::create("test.csv")?;
        let mut writer = WriterBuilder::new().has_headers(false).from_writer(file);
        writer.serialize_array2(&short_rate_array)?;
    }

     // Read an array back from the file
    // let file = File::open("test.csv")?;
    // let mut reader = ReaderBuilder::new().has_headers(false).from_reader(file);
    // let array_read: Array2<f64> = reader.deserialize_array2((2, 3))?;

    // assert_eq!(array_read, short_rate_array);
    Ok(())

    // plotter::plot_single_graph(t_linspace, short_rate_array);
}
