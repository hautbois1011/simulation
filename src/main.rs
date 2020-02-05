extern crate plotters;
use plotters::prelude::*;
mod complex;
use complex::Complex;
mod fft;
use fft::fft;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("png/fft.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .margin(20)
        .x_label_area_size(10)
        .y_label_area_size(10)
        .build_ranged(0.0f64..6.3f64, -20.0f64..20.0f64)?;

    chart.configure_mesh().draw()?;

    let tr: Vec<Complex> = fft(
        |x| {
            if -1. <= x && x <= 1. {
                x.cos()
            } else {
                0.
            }
        },
        12,
    );

    chart
        .draw_series(LineSeries::new(
            tr.iter()
                .enumerate()
                .map(|(i, x)| (2. * std::f64::consts::PI * i as f64 / 4096.0, x.re)),
            &RED,
        ))?
        .label("Re");

    chart
        .draw_series(LineSeries::new(
            tr.iter()
                .enumerate()
                .map(|(i, x)| (2. * std::f64::consts::PI * i as f64 / 4096.0, x.im)),
            &BLUE,
        ))?
        .label("Im");

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.7))
        .border_style(&BLACK)
        .draw()?;

    let c = Complex::new(2.0f64, 3.0f64);
    let d = Complex::new(2.5f64, 3.0f64);
    println!("{:?}", c + d);
    println!("{:?}", c - d);
    println!("{:?}", c * d);
    println!("{:?}", c / d);
    println!("{:?}", c.norm());
    println!("{:?}", c.norm2());
    println!("{:?}", c.exp());

    println!("{:?}", fft(|x| x.sin(), 5));

    Ok(())
}
