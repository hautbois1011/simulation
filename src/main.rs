extern crate plotters;
use plotters::prelude::*;
mod complex;
use complex::Complex;
mod fft;
use fft::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new("png/fft.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let mut chart = ChartBuilder::on(&root)
        .caption(
            "Fourier transform of y = sin x + cos x + 0.5cos 3x",
            ("sans-serif", 30).into_font(),
        )
        .margin(20)
        .x_label_area_size(10)
        .y_label_area_size(10)
        .build_ranged(0.0f64..4096.0f64, -2.0f64..2.0f64)?;

    chart.configure_mesh().draw()?;

    // let input = (0..4096)
    //     .map(|i| 2. * std::f64::consts::PI * (i as f64) / 4096.)
    //     .map(|x| (10. * x).sin())
    //     .map(|x| Complex::new(x, 0.0))
    //     .collect();

    let input = (0..4096)
        .map(|i| 2. * std::f64::consts::PI * (i as f64) / 4096.)
        .map(|x| x.sin() + x.cos() + 0.5 * (3. * x).cos())
        .collect();

    let output = real_ifft(&real_fft(&input));

    chart
        .draw_series(LineSeries::new(
            input.iter().enumerate().map(|(i, &x)| (i as f64, x)),
            &RED,
        ))?
        .label("input")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

    chart
        .draw_series(LineSeries::new(
            output.iter().enumerate().map(|(i, &x)| (i as f64, x)),
            &BLUE,
        ))?
        .label("output")
        .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

    chart
        .configure_series_labels()
        .background_style(&WHITE.mix(0.7))
        .border_style(&BLACK)
        .draw()?;

    // let c = Complex::new(2.0f64, 3.0f64);
    // let d = Complex::new(2.5f64, 3.0f64);
    // println!("{:?}", c + d);
    // println!("{:?}", c - d);
    // println!("{:?}", c * d);
    // println!("{:?}", c / d);
    // println!("{:?}", c.abs());
    // println!("{:?}", c.abs2());
    // println!("{:?}", c.exp());

    Ok(())
}
