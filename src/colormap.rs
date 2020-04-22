use palette::{Lch, LabHue, Gradient, Srgb};
use std::f32::consts::PI;
use image::{RgbaImage, Rgba};
use crate::Canvas;

lazy_static!{
    pub static ref MAP_COLORFUL: Vec<(f32, Lch)> = vec![
        (0.0, Lch::new(0., 128., LabHue::from_radians(-2. * PI / 3.))),
        (0.3, Lch::new(20., 128., LabHue::from_radians(-1. * PI / 3.))),
        (0.6, Lch::new(50., 128., LabHue::from_radians(0.))),
        (0.9, Lch::new(80., 128., LabHue::from_radians(PI / 3.))),
        (1.0, Lch::new(100., 0., LabHue::from_radians(PI / 3.)))
    ];
    pub static ref MAP_STEEL: Vec<(f32, Lch)> = vec![
        (0.0, Lch::new(100., 0.0, LabHue::from_radians(-2. * PI / 3.))),
        (0.1, Lch::new(90., 16., LabHue::from_radians(-2. * PI / 3.))),
        (0.2, Lch::new(80., 32., LabHue::from_radians(-2. * PI / 3.))),
        (1.0, Lch::new(0., 64., LabHue::from_radians(-2. * PI / 3.))),
    ];
}

pub fn render(canvas: &Canvas, map: Vec<(f32, Lch)>) -> RgbaImage {
    // prepare fast LUT
    let steps = 256 * (map.len() - 1);

    let k = steps as f32;
    let gradient = Gradient::with_domain(map);
    
    // build LUT
    let lut: Vec<_> = (0 .. steps + 1).map(|i| {
        let rgb = Srgb::from(gradient.get(i as f32 / k));
        let (r, g, b) = rgb.into_format::<u8>().into_components();
        Rgba([r, g, b, 255])
    }).collect();

    // analyze data
    let stats = Stats::new(canvas.values());
    let scale = k / (stats.max - stats.min);

    let (width, height) = canvas.size();
    let mut imgbuf = RgbaImage::from_pixel(width as u32, height as u32, Rgba([0, 0, 0, 0]));
    for (y, row) in imgbuf.enumerate_rows_mut() {
        for (val, (_x, _y, pixel)) in canvas.row(height - y as usize - 1).zip(row) {
            if val.is_finite() {
                let lut_index = ((val - stats.min) * scale) as usize;
                *pixel = lut[lut_index];
            }
        }
    }

    imgbuf
}

struct Stats {
    min: f32,
    max: f32
}
impl Stats {
    fn new(values: impl Iterator<Item=f32>) -> Stats {
        let mut values = values.filter(|val| val.is_finite());
        let val = values.next().unwrap();
        let mut min = val;
        let mut max = val;

        for val in values {
            if val.is_finite() {
                min = min.min(val);
                max = max.max(val);
            }
        }
        
        Stats { min, max }
    }
}