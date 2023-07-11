#![allow(dead_code)]

use std::marker::PhantomData;

use crate::gui::lpicklist::LPickList;
use crate::gui::numeric_input::NumericInput;
use iced::widget::{text, text_input, Column, Rule};
use iced::Element;
use wassily::prelude::img_noise::{ColorMap, ImgNoise};
use wassily::prelude::*;

static DEFAULT_IMAGE: &[u8] = include_bytes!("./default.raw");

#[derive(Debug, Clone, PartialEq)]
pub enum NoiseMessage {
    Function(NoiseFunctionName),
    Factor(f32),
    ScaleX(f32),
    ScaleY(f32),
    Octaves(i32),
    Frequency(f32),
    SinXFreq(f32),
    SinYFreq(f32),
    ImgNoisePathSet(String),
    ImgNoisePath,
    ImgColorMap(ColorMap),
    Null,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NoiseControls {
    pub function: Option<NoiseFunctionName>,
    pub factor: f32,
    pub scale_x: f32,
    pub scale_y: f32,
    pub octaves: i32,
    pub frequency: f32,
    pub sin_x_freq: f32,
    pub sin_y_freq: f32,
    pub img_noise_path: String,
    pub img: DynamicImage,
    pub img_color_map: Option<ColorMap>,
    pub dirty: bool,
}

impl Default for NoiseControls {
    fn default() -> Self {
        let img = DynamicImage::ImageRgba8(
            ImageBuffer::from_raw(1200, 1000, DEFAULT_IMAGE.to_vec()).unwrap(),
        );
        Self {
            function: Some(NoiseFunctionName::Fbm),
            factor: 50.0,
            scale_x: 8.0,
            scale_y: 8.0,
            octaves: 1,
            frequency: 1.0,
            sin_x_freq: 1.0,
            sin_y_freq: 1.0,
            img_noise_path: String::from(""),
            img,
            img_color_map: Some(ColorMap::Lightness),
            dirty: false,
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
        frequency: f32,
        sin_x_freq: f32,
        sin_y_freq: f32,
        img_noise_path: String,
        img: DynamicImage,
        img_color_map: Option<ColorMap>,
        dirty: bool,
    ) -> Self {
        Self {
            function,
            factor,
            scale_x,
            scale_y,
            octaves,
            frequency,
            sin_x_freq,
            sin_y_freq,
            img_noise_path,
            img,
            img_color_map,
            dirty,
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

    pub fn set_img_noise_path(mut self, img_noise_path: String) -> Self {
        self.img_noise_path = img_noise_path;
        self
    }

    pub fn update(&mut self, message: NoiseMessage) {
        use NoiseMessage::*;
        self.dirty = true;
        match message {
            Function(n) => {
                self.function = Some(n);
                if n == NoiseFunctionName::Image {
                    self.scale_x = 1.0;
                    self.scale_y = 1.0;
                }
            }
            Factor(f) => self.factor = f,
            ScaleX(s) => self.scale_x = s,
            ScaleY(s) => self.scale_y = s,
            Octaves(octaves) => self.octaves = octaves,
            Frequency(frequency) => self.frequency = frequency,
            SinXFreq(sin_x_freq) => self.sin_x_freq = sin_x_freq,
            SinYFreq(sin_y_freq) => self.sin_y_freq = sin_y_freq,
            ImgNoisePathSet(img_noise_path) => {
                self.img_noise_path = img_noise_path;
                self.dirty = false
            }
            ImgNoisePath => {
                self.img = match open(std::path::Path::new(&self.img_noise_path)) {
                    Ok(img) => img,
                    Err(_) => DynamicImage::ImageRgba8(
                        ImageBuffer::from_raw(1200, 1000, DEFAULT_IMAGE.to_vec()).unwrap(),
                    ),
                };
            }
            ImgColorMap(cm) => {
                self.img_color_map = Some(cm);
            }
            Null => {}
        }
    }

    pub fn view(&self) -> Element<'a, NoiseMessage> {
        use NoiseFunctionName::*;
        use NoiseMessage::*;
        let mut col = Column::new().push(Rule::horizontal(5));
        col = col.push(LPickList::new(
            "Noise Function".to_string(),
            vec![
                Fbm, Billow, Ridged, Value, Cylinders, Curl, Sinusoidal, SinFbm, Image,
            ],
            self.function,
            |x| x.map_or(Null, Function),
        ));
        let func = self.function.expect("Noise function not set");
        if func == Image {
            col = col
                .push(text("Image Path").width(200))
                .push(
                    text_input("", &self.img_noise_path)
                        .on_input(ImgNoisePathSet)
                        .size(15)
                        .width(200)
                        .on_submit(ImgNoisePath),
                )
                .push(LPickList::new(
                    "Color Map".to_string(),
                    vec![
                        ColorMap::Lightness,
                        ColorMap::RedGreen,
                        ColorMap::YellowBlue,
                    ],
                    self.img_color_map,
                    |x| x.map_or(Null, ImgColorMap),
                ));
        }
        if func != Image {
            col = col
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
        }
        col = col.push(NumericInput::new(
            "Noise Factor".to_string(),
            self.factor,
            1.0..=5000.0,
            1.0,
            0,
            Factor,
        ));
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
                col = col.push(NumericInput::new(
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
    Image,
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
                NoiseFunctionName::Image => "Image",
            }
        )
    }
}

#[derive(Clone)]
pub enum NoiseFunction {
    Fbm(Fbm<Perlin>),
    Billow(Billow<Perlin>),
    Ridged(RidgedMulti<Perlin>),
    Value(Value),
    Cylinders(TranslatePoint<Cylinders>),
    Curl(Curl<Fbm<Perlin>>),
    Sinusoidal(Sinusoidal),
    SinFbm(Sin<f64, Fbm<Perlin>, 2>),
    Image(ImgNoise),
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
            NoiseFunction::Image(n) => n.get(point),
        }
    }
}

impl Seedable for NoiseFunction {
    fn set_seed(self, seed: u32) -> Self {
        match self {
            NoiseFunction::Fbm(n) => NoiseFunction::Fbm(n.set_seed(seed)),
            NoiseFunction::Billow(n) => NoiseFunction::Billow(n.set_seed(seed)),
            NoiseFunction::Ridged(n) => NoiseFunction::Ridged(n.set_seed(seed)),
            NoiseFunction::Value(n) => NoiseFunction::Value(n.set_seed(seed)),
            NoiseFunction::Cylinders(n) => NoiseFunction::Cylinders(n),
            NoiseFunction::Curl(n) => NoiseFunction::Curl(n.set_seed(seed)),
            NoiseFunction::Sinusoidal(n) => NoiseFunction::Sinusoidal(n),
            NoiseFunction::SinFbm(n) => NoiseFunction::SinFbm(n),
            NoiseFunction::Image(n) => NoiseFunction::Image(n),
        }
    }

    fn seed(&self) -> u32 {
        todo!()
    }
}

pub fn choose_noise(controls: &NoiseControls) -> NoiseFunction {
    match controls.function.unwrap() {
        NoiseFunctionName::Fbm => NoiseFunction::Fbm(
            Fbm::<Perlin>::default()
                .set_octaves(controls.octaves as usize)
                .set_frequency(controls.frequency as f64),
        ),
        NoiseFunctionName::Billow => NoiseFunction::Billow(
            Billow::<Perlin>::default()
                .set_octaves(controls.octaves as usize)
                .set_frequency(controls.frequency as f64),
        ),
        NoiseFunctionName::Ridged => NoiseFunction::Ridged(
            RidgedMulti::<Perlin>::default()
                .set_octaves(controls.octaves as usize)
                .set_frequency(controls.frequency as f64),
        ),
        NoiseFunctionName::Value => NoiseFunction::Value(Value::default()),
        NoiseFunctionName::Cylinders => NoiseFunction::Cylinders(TranslatePoint::new(
            Cylinders::default().set_frequency(controls.octaves as f64 / 2.0),
        )),
        NoiseFunctionName::Curl => {
            let nf = Fbm::<Perlin>::default()
                .set_octaves(controls.octaves as usize)
                .set_frequency(controls.frequency as f64);
            NoiseFunction::Curl(Curl::new(nf))
        }
        NoiseFunctionName::Sinusoidal => NoiseFunction::Sinusoidal(Sinusoidal::new(
            controls.sin_x_freq as f64,
            controls.sin_y_freq as f64,
        )),
        NoiseFunctionName::SinFbm => NoiseFunction::SinFbm(Sin::new(
            Fbm::<Perlin>::default()
                .set_octaves(controls.octaves as usize)
                .set_frequency(controls.frequency as f64),
        )),
        NoiseFunctionName::Image => NoiseFunction::Image(
            ImgNoise::new(controls.img.clone()).set_map(controls.img_color_map.unwrap()),
        ),
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Sinusoidal {
    x_freq: f64,
    y_freq: f64,
}

impl Default for Sinusoidal {
    fn default() -> Self {
        Self {
            x_freq: 1.0,
            y_freq: 1.0,
        }
    }
}

impl Sinusoidal {
    pub fn new(x_freq: f64, y_freq: f64) -> Self {
        Self { x_freq, y_freq }
    }
}

impl NoiseFn<f64, 2> for Sinusoidal {
    fn get(&self, point: [f64; 2]) -> f64 {
        0.5 * ((self.x_freq * point[0]).sin() + (self.y_freq * point[1]).sin())
    }
}

#[derive(Debug, Clone, Copy)]
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
