#![allow(dead_code)]

use std::marker::PhantomData;

use crate::gui::lpicklist::LPickList;
use crate::gui::numeric_input::NumericInput;
use iced::widget::{Column, Rule};
use iced::Element;
use wassily::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NoiseMessage {
    Function(NoiseFunctionName),
    Factor(f32),
    Scale(f32),
    Octaves(i32),
    Persistence(f32),
    Lacunarity(f32),
    Frequency(f32),
    SinXFreq(f32),
    SinYFreq(f32),
    SinXExp(i32),
    SinYExp(i32),
    Null,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NoiseControls {
    pub function: Option<NoiseFunctionName>,
    pub factor: f32,
    pub scale: f32,
    pub octaves: i32,
    pub persistence: f32,
    pub lacunarity: f32,
    pub frequency: f32,
    pub sin_x_freq: f32,
    pub sin_y_freq: f32,
    pub sin_x_exp: i32,
    pub sin_y_exp: i32,
}

impl Default for NoiseControls {
    fn default() -> Self {
        Self {
            function: Some(NoiseFunctionName::Fbm),
            factor: 50.0,
            scale: 8.0,
            octaves: 4,
            persistence: 0.5,
            lacunarity: 2.0,
            frequency: 1.0,
            sin_x_freq: 1.0,
            sin_y_exp: 2,
            sin_x_exp: 2,
            sin_y_freq: 1.0,
        }
    }
}

impl<'a> NoiseControls {
    pub fn new(
        function: Option<NoiseFunctionName>,
        factor: f32,
        scale: f32,
        octaves: i32,
        persistence: f32,
        lacunarity: f32,
        frequency: f32,
        sin_x_freq: f32,
        sin_y_freq: f32,
        sin_x_exp: i32,
        sin_y_exp: i32,
    ) -> Self {
        Self {
            function,
            factor,
            scale,
            octaves,
            persistence,
            lacunarity,
            frequency,
            sin_x_freq,
            sin_y_freq,
            sin_x_exp,
            sin_y_exp,
        }
    }

    pub fn set_noise_function(mut self, noise_function: NoiseFunctionName) -> Self {
        self.function = Some(noise_function);
        self
    }

    pub fn set_noise_factor(mut self, noise_factor: f32) -> Self {
        self.factor = noise_factor;
        self
    }

    pub fn set_noise_scale(mut self, noise_scale: f32) -> Self {
        self.scale = noise_scale;
        self
    }

    pub fn set_ocataves(mut self, octaves: i32) -> Self {
        self.octaves = octaves;
        self
    }

    pub fn set_persistence(mut self, persistence: f32) -> Self {
        self.persistence = persistence;
        self
    }

    pub fn set_lacunarity(mut self, lacunarity: f32) -> Self {
        self.lacunarity = lacunarity;
        self
    }
    pub fn set_frequency(mut self, frequency: f32) -> Self {
        self.frequency = frequency;
        self
    }

    pub fn set_sin_x_freq(mut self, sin_x_freq: f32) -> Self {
        self.sin_x_freq = sin_x_freq;
        self
    }

    pub fn set_sin_y_freq(mut self, sin_y_freq: f32) -> Self {
        self.sin_y_freq = sin_y_freq;
        self
    }

    pub fn set_sin_x_exp(mut self, sin_x_exp: i32) -> Self {
        self.sin_x_exp = sin_x_exp;
        self
    }

    pub fn set_sin_y_exp(mut self, sin_y_exp: i32) -> Self {
        self.sin_y_exp = sin_y_exp;
        self
    }

    pub fn update(&mut self, message: NoiseMessage) {
        use NoiseMessage::*;
        match message {
            Function(n) => self.function = Some(n),
            Factor(f) => self.factor = f,
            Scale(s) => self.scale = s,
            Octaves(octaves) => self.octaves = octaves,
            Persistence(persistence) => self.persistence = persistence,
            Lacunarity(lacunarity) => self.lacunarity = lacunarity,
            Frequency(frequency) => self.frequency = frequency,
            SinXExp(sin_x_exp) => self.sin_x_exp = sin_x_exp,
            SinYExp(sin_y_exp) => self.sin_y_exp = sin_y_exp,
            SinXFreq(sin_x_freq) => self.sin_x_freq = sin_x_freq,
            SinYFreq(sin_y_freq) => self.sin_y_freq = sin_y_freq,
            Null => {}
        }
    }

    pub fn view(&self) -> Element<'a, NoiseMessage> {
        use NoiseFunctionName::*;
        use NoiseMessage::*;
        let mut col = Column::new().push(Rule::horizontal(5));
        col = col
            .push(LPickList::new(
                "Noise Function".to_string(),
                vec![
                    Fbm, Billow, Ridged, Value, Cylinders, Curl, Magnet, Sinusoidal, SinFbm,
                ],
                self.function,
                |x| x.map_or(Null, Function),
            ))
            .push(NumericInput::new(
                "Noise Scale".to_string(),
                self.scale,
                0.5..=50.0,
                0.1,
                1,
                Scale,
            ))
            .push(NumericInput::new(
                "Noise Factor".to_string(),
                self.factor,
                1.0..=5000.0,
                1.0,
                0,
                Factor,
            ));
        let func = self.function.expect("Noise function not set");
        if func == Sinusoidal {
            col = col
                .push(NumericInput::new(
                    "Sine X Frequency".to_string(),
                    self.sin_x_freq,
                    0.1..=10.0,
                    0.1,
                    1,
                    SinXFreq,
                ))
                .push(NumericInput::new(
                    "Sine Y Frequency".to_string(),
                    self.sin_y_freq,
                    0.1..=10.0,
                    0.1,
                    1,
                    SinYFreq,
                ))
                .push(NumericInput::new(
                    "Sine X Exponent".to_string(),
                    self.sin_x_exp,
                    1..=10,
                    1,
                    0,
                    SinXExp,
                ))
                .push(NumericInput::new(
                    "Sine Y Exponent".to_string(),
                    self.sin_y_exp,
                    1..=10,
                    1,
                    0,
                    SinYExp,
                ));
        }
        if func == Fbm || func == Ridged || func == Billow || func == Curl || func == SinFbm {
            col = col.push(NumericInput::new(
                "Octaves".to_string(),
                self.octaves,
                1..=8,
                1,
                0,
                Octaves,
            ));
            if self.octaves > 1 {
                col = col
                    .push(NumericInput::new(
                        "Persistence".to_string(),
                        self.persistence,
                        0.05..=0.95,
                        0.05,
                        1,
                        Persistence,
                    ))
                    .push(NumericInput::new(
                        "Lacunarity".to_string(),
                        self.lacunarity,
                        0.1..=4.00,
                        0.1,
                        1,
                        Lacunarity,
                    ))
                    .push(NumericInput::new(
                        "Frequency".to_string(),
                        self.frequency,
                        0.1..=4.00,
                        0.1,
                        1,
                        Frequency,
                    ))
            }
        }
        col.spacing(10).into()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum NoiseFunctionName {
    Fbm,
    Billow,
    Ridged,
    Value,
    Cylinders,
    Curl,
    Magnet,
    Sinusoidal,
    SinFbm,
}

impl std::fmt::Display for NoiseFunctionName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                NoiseFunctionName::Fbm => "Fbm",
                NoiseFunctionName::Billow => "Billow",
                NoiseFunctionName::Ridged => "Ridged",
                NoiseFunctionName::Cylinders => "Cylinders",
                NoiseFunctionName::Value => "Value",
                NoiseFunctionName::Curl => "Curl",
                NoiseFunctionName::Magnet => "Magnet",
                NoiseFunctionName::Sinusoidal => "Sinusoidal",
                NoiseFunctionName::SinFbm => "SinFbm",
            }
        )
    }
}

pub enum NoiseFunction {
    Fbm(Fbm<Perlin>),
    Billow(Billow<Perlin>),
    Ridged(RidgedMulti<Perlin>),
    Value(Value),
    Cylinders(TranslatePoint<Cylinders>),
    Curl(Curl<Fbm<Perlin>>),
    Magnet(Magnet),
    Sinusoidal(Sinusoidal),
    SinFbm(Sin<f64, Fbm<Perlin>, 2>),
}

impl NoiseFn<f64, 2> for NoiseFunction {
    fn get(&self, point: [f64; 2]) -> f64 {
        match self {
            NoiseFunction::Fbm(n) => n.get(point),
            NoiseFunction::Billow(n) => n.get(point),
            NoiseFunction::Ridged(n) => n.get(point),
            NoiseFunction::Value(n) => n.get(point),
            NoiseFunction::Cylinders(n) => n.get(point),
            NoiseFunction::Curl(n) => n.get(point),
            NoiseFunction::Magnet(n) => n.get(point),
            NoiseFunction::Sinusoidal(n) => n.get(point),
            NoiseFunction::SinFbm(n) => n.get(point),
        }
    }
}

pub fn choose_noise(controls: &NoiseControls) -> NoiseFunction {
    match controls.function.unwrap() {
        NoiseFunctionName::Fbm => NoiseFunction::Fbm(
            Fbm::<Perlin>::default()
                .set_octaves(controls.octaves as usize)
                .set_persistence(controls.persistence as f64)
                .set_lacunarity(controls.lacunarity as f64)
                .set_frequency(controls.frequency as f64),
        ),
        NoiseFunctionName::Billow => NoiseFunction::Billow(
            Billow::<Perlin>::default()
                .set_octaves(controls.octaves as usize)
                .set_lacunarity(controls.lacunarity as f64)
                .set_frequency(controls.frequency as f64)
                .set_persistence(controls.persistence as f64),
        ),
        NoiseFunctionName::Ridged => NoiseFunction::Ridged(
            RidgedMulti::<Perlin>::default()
                .set_octaves(controls.octaves as usize)
                .set_lacunarity(controls.lacunarity as f64)
                .set_frequency(controls.frequency as f64)
                .set_persistence(controls.persistence as f64),
        ),
        NoiseFunctionName::Value => NoiseFunction::Value(Value::default()),
        NoiseFunctionName::Cylinders => NoiseFunction::Cylinders(
            TranslatePoint::new(Cylinders::default().set_frequency(controls.octaves as f64 / 2.0))
                .set_x_translation(4032.0 / 2.0)
                .set_y_translation(3024.0 / 2.0),
        ),
        NoiseFunctionName::Curl => {
            let nf = Fbm::<Perlin>::default()
                .set_octaves(controls.octaves as usize)
                .set_lacunarity(controls.lacunarity as f64)
                .set_frequency(controls.frequency as f64)
                .set_persistence(controls.persistence as f64);
            NoiseFunction::Curl(Curl::new(nf))
        }
        NoiseFunctionName::Magnet => NoiseFunction::Magnet(Magnet::new(vec![
            pt(1000, 750),
            pt(1000, 2250),
            pt(3000, 750),
            pt(3000, 2250),
        ])),
        NoiseFunctionName::Sinusoidal => NoiseFunction::Sinusoidal(Sinusoidal::new(
            controls.sin_x_freq as f64,
            controls.sin_y_freq as f64,
            controls.sin_x_exp as i32,
            controls.sin_y_exp as i32,
        )),
        NoiseFunctionName::SinFbm => NoiseFunction::SinFbm(Sin::new(
            Fbm::<Perlin>::default()
                .set_octaves(controls.octaves as usize)
                .set_lacunarity(controls.lacunarity as f64)
                .set_frequency(controls.frequency as f64)
                .set_persistence(controls.persistence as f64),
        )),
    }
}

pub struct Magnet {
    sinks: Vec<Point>,
}

impl Magnet {
    pub fn new(sinks: Vec<Point>) -> Self {
        Self { sinks }
    }
}

impl NoiseFn<f64, 2> for Magnet {
    fn get(&self, point: [f64; 2]) -> f64 {
        let mut p = Point::zero();
        let mut min_sink = f64::MAX;
        for s in &self.sinks {
            let d = pt(point[0], point[1]).dist2(pt(s.x, s.y)) as f64;
            if d < min_sink {
                min_sink = d;
                p = *s;
            }
        }
        if min_sink == f64::MAX {
            return 0.0;
        }
        (p.y as f64 - point[1]).atan2(p.x as f64 - point[0]) / std::f64::consts::PI
    }
}

pub struct Sinusoidal {
    x_freq: f64,
    y_freq: f64,
    x_exp: i32,
    y_exp: i32,
}

impl Default for Sinusoidal {
    fn default() -> Self {
        Self {
            x_freq: 1.0,
            y_freq: 1.0,
            x_exp: 2,
            y_exp: 2,
        }
    }
}

impl Sinusoidal {
    pub fn new(x_freq: f64, y_freq: f64, x_exp: i32, y_exp: i32) -> Self {
        Self {
            x_freq,
            y_freq,
            x_exp,
            y_exp,
        }
    }
}

impl NoiseFn<f64, 2> for Sinusoidal {
    fn get(&self, point: [f64; 2]) -> f64 {
        0.5 * ((self.x_freq * point[0]).sin().powi(self.x_exp)
            + (self.y_freq * point[1]).sin().powi(self.y_exp))
    }
}
pub struct Sin<T, Source, const DIM: usize>
where
    Source: NoiseFn<T, DIM>,
{
    /// Outputs a value.
    pub source: Source,

    phantom: PhantomData<T>,
}

impl<T, Source, const DIM: usize> Sin<T, Source, DIM>
where
    Source: NoiseFn<T, DIM>,
{
    pub fn new(source: Source) -> Self {
        Self {
            source,
            phantom: PhantomData,
        }
    }
}

impl<T, Source, const DIM: usize> NoiseFn<T, DIM> for Sin<T, Source, DIM>
where
    Source: NoiseFn<T, DIM>,
{
    fn get(&self, point: [T; DIM]) -> f64 {
        (self.source.get(point) * std::f64::consts::PI).sin()
    }
}
