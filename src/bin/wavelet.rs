
use std::f32::consts::*;
use std::ops::*;
use audio::{C, square, Canvas};
use rayon::prelude::*;

fn wavelet(f: f32, freq_acc: f32) -> impl Fn(f32) -> C {
    let c = 0.5 / freq_acc;
    move |t| C::polar(-square(c * f * t), 2.0 * PI * f * t)
}

fn main() {
    let f_min: f32 = 100.;
    let f_max: f32 = 20000.;
    let y_res = 2000;
    let sample_step = 10;

    let mut args = std::env::args().skip(1);
    let input = args.next().expect("no input given");
    let output = args.next().expect("no output given");
    let reader = hound::WavReader::open(&input).unwrap();
    let spec = reader.spec();
    assert_eq!(spec.channels, 1);
    let sample_rate = spec.sample_rate as f32;

    let data: Result<Vec<f32>, _> = reader.into_samples::<f32>().collect();
    let sample_data = data.unwrap();
    dbg!(sample_data.len());


    let t_max = sample_data.len() as f32 / sample_rate as f32;
    let width = sample_data.len() / sample_step;
    let mut canvas = Canvas::new(width, y_res);

    let y_step = (f_max.ln() - f_min.ln()) / (y_res as f32);

    let freq_acc = 1.0;

    canvas.par_rows_mut().enumerate().for_each(|(y, row)| {
        let f = f_min * ((y as f32) * y_step).exp();
        let w = 1.0 / f;
        let width = 10. * freq_acc * sample_rate * w;
        let wavelet_samples = width as usize;
        dbg!(f, w, width, wavelet_samples);
        let mut buffer = vec![C::new(0.0, 0.0); wavelet_samples];
        let gen = wavelet(f, freq_acc);

        let dt = f / sample_rate;
        let t0 = - ((wavelet_samples / 2) as f32) * dt;
        for i in 0 .. wavelet_samples {
            let t = t0 + dt * w * i as f32;
            buffer[i] = gen(t);
        }
        let norm = 1.0 / buffer.iter().map(|c| c.abs()).sum::<f32>();

        dbg!(sample_data.len().saturating_sub(wavelet_samples));
        for j in 0 .. sample_data.len().saturating_sub(wavelet_samples) {
            let val = buffer.iter().zip(sample_data[j ..].iter()).map(|(&a, &b)| a * b).sum::<C>().abs();
            let val = val * norm;

            let x = (j + wavelet_samples / 2) / sample_step;
            add_val(&mut row[x], val);
        }
    });

    canvas.save(&output);
}

fn add_val(out: &mut f32, val: f32) {
    if out.is_nan() {
        *out = val;
    } else {
        *out += val;
    }
}