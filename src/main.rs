extern crate plotters;
use plotters::prelude::*;
mod complex;
use complex::*;
mod fft;
use fft::*;
mod runge_kutta;
use runge_kutta::*;
use std::ops::Fn;

const N: usize = 256;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = (0..N)
        .map(|i| 2. * std::f64::consts::PI * (i as f64) / N as f64)
        .map(|x| x.sin())
        .collect::<Vec<_>>();

    let func: Box<dyn Fn(ComplexVec, f64) -> ComplexVec> = Box::new(|u_hat, _t| {
        let mut uh = u_hat.vec.clone();
        uh.extend(vec![Complex::new(0.0, 0.0); N / 2]);
        let u = real_ifft(&uh);
        let u_x = real_ifft(
            &uh.iter()
                .enumerate()
                .map(|(k, &z)| z * Complex::new(0.0, k as f64))
                .collect::<Vec<_>>(),
        );
        let f = u
            .iter()
            .zip(u_x.iter())
            .map(|(&x, &y)| -x * y)
            .collect::<Vec<_>>();
        ComplexVec {
            vec: real_fft(&f).into_iter().take(N / 2 + 1).collect(),
        }
    });

    let mut rk = RungeKutta {
        x: ComplexVec {
            vec: real_fft(&input),
        },
        func: func,
        t: 0.0f64,
        dt: 0.01f64,
    };

    let root = BitMapBackend::gif("gif/animation.gif", (800, 600), 100)?.into_drawing_area();

    for _ in 0..105 {
        root.fill(&WHITE)?;

        let mut chart = ChartBuilder::on(&root)
            .caption(
                format!("u_t + u*(u_x) = 0 (u(0, x) = sin x) t = {}", rk.t),
                ("sans-serif", 30).into_font(),
            )
            .margin(20)
            .x_label_area_size(10)
            .y_label_area_size(10)
            .build_ranged(0.0f64..(N as f64), -2.0f64..2.0f64)?;

        chart.configure_mesh().draw()?;

        chart
            .draw_series(LineSeries::new(
                input.iter().enumerate().map(|(i, &x)| (i as f64, x)),
                &RED,
            ))?
            .label("input")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &RED));

        chart
            .draw_series(LineSeries::new(
                real_ifft(&rk.x.vec)
                    .iter()
                    .enumerate()
                    .map(|(i, &x)| (i as f64, x)),
                &BLUE,
            ))?
            .label("output")
            .legend(|(x, y)| PathElement::new(vec![(x, y), (x + 20, y)], &BLUE));

        chart
            .configure_series_labels()
            .background_style(&WHITE.mix(0.7))
            .border_style(&BLACK)
            .draw()?;

        root.present()?;

        rk.step();
    }

    Ok(())
}
