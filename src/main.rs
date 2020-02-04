extern crate plotters;
use plotters::prelude::*;
mod complex;
use complex::Complex;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("png/fft.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .margin(20)
        .x_label_area_size(10)
        .y_label_area_size(10)
        .build_ranged(-2.1f64..0.6f64, -1.2f64..1.2f64)?;

    chart
        .configure_mesh()
        .disable_x_mesh()
        .disable_y_mesh()
        .draw()?;

    let c = Complex::new(2.0f64, 3.0f64);
    let d = Complex::new(2.5f64, 3.0f64);
    println!("{:?}", c + d);
    println!("{:?}", c - d);
    println!("{:?}", c * d);
    println!("{:?}", c / d);
    println!("{:?}", c.norm());
    println!("{:?}", c.norm2());

    Ok(())
}
