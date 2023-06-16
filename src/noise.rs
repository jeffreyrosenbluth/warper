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
    ScaleX(f32),
    ScaleY(f32),
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
    pub scale_x: f32,
    pub scale_y: f32,
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
            scale_x: 8.0,
            scale_y: 8.0,
            octaves: 1,
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
        scale_x: f32,
        scale_y: f32,
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
            scale_x,
            scale_y,
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

    pub fn set_noise_scale_x(mut self, noise_scale: f32) -> Self {
        self.scale_x = noise_scale;
        self
    }

    pub fn set_noise_scale_y(mut self, noise_scale: f32) -> Self {
        self.scale_y = noise_scale;
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
            ScaleX(s) => self.scale_x = s,
            ScaleY(s) => self.scale_y = s,
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
                    Fbm, Billow, Ridged, Value, Cylinders, Curl, Sinusoidal, SinFbm,
                ],
                self.function,
                |x| x.map_or(Null, Function),
            ))
            .push(NumericInput::new(
                "Noise Scale X".to_string(),
                self.scale_x,
                0.5..=50.0,
                0.1,
                1,
                ScaleX,
            ))
            .push(NumericInput::new(
                "Noise Scale Y".to_string(),
                self.scale_y,
                0.5..=50.0,
                0.1,
                1,
                ScaleY,
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
                1..=6,
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
        col.spacing(7).into()
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
        NoiseFunctionName::Cylinders => NoiseFunction::Cylinders(TranslatePoint::new(
            Cylinders::default().set_frequency(controls.octaves as f64 / 2.0),
        )),
        NoiseFunctionName::Curl => {
            let nf = Fbm::<Perlin>::default()
                .set_octaves(controls.octaves as usize)
                .set_lacunarity(controls.lacunarity as f64)
                .set_frequency(controls.frequency as f64)
                .set_persistence(controls.persistence as f64);
            NoiseFunction::Curl(Curl::new(nf))
        }
        NoiseFunctionName::Sinusoidal => NoiseFunction::Sinusoidal(Sinusoidal::new(
            controls.sin_x_freq as f64,
            controls.sin_y_freq as f64,
            controls.sin_x_exp,
            controls.sin_y_exp,
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
