use audio::{Canvas, colormap};
use palette::{Lch, LabHue};
use std::f32::consts::PI;

fn main() {
    let mut args = std::env::args().skip(1);
    let input = args.next().expect("no input given");
    let output = args.next().expect("no output given");

    let canvas = Canvas::load(&input);
    let map = vec![
        (0.0, Lch::new(0., 128., LabHue::from_radians(-2. * PI / 3.))),
        (0.3, Lch::new(20., 128., LabHue::from_radians(-1. * PI / 3.))),
        (0.6, Lch::new(50., 128., LabHue::from_radians(0.))),
        (0.9, Lch::new(80., 128., LabHue::from_radians(PI / 3.))),
        (1.0, Lch::new(100., 0., LabHue::from_radians(PI / 3.)))
    ];
    let map2 = vec![
        (0.0, Lch::new(100., 0.0, LabHue::from_radians(-2. * PI / 3.))),
        (0.1, Lch::new(90., 16., LabHue::from_radians(-2. * PI / 3.))),
        (0.2, Lch::new(80., 32., LabHue::from_radians(-2. * PI / 3.))),
        (1.0, Lch::new(0., 64., LabHue::from_radians(-2. * PI / 3.))),
    ];
    colormap::render(&canvas, map2).save(&output).expect("failed to write image");
}
