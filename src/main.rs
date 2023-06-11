use directories::UserDirs;
use iced::{
    widget::{button, image, row, text, Column, Container},
    Application, Command, Element, Settings, Theme,
};
use num_complex::Complex32;
use rayon::prelude::*;
use std::path::PathBuf;
use std::sync::Arc;
use wassily::prelude::{
    imageops, noise2d, noise2d_01, open, Colorful, Coord, ImageBuffer, NoiseOpts, Rgba, Warp,
    WarpNode,
};

mod gui;
mod noise;

use crate::gui::numeric_input::NumericInput;
use crate::noise::*;

pub fn main() -> iced::Result {
    env_logger::init();
    let mut settings = Settings::default();
    settings.window.size = (1430, 1200);
    Warper::run(settings)
}

#[derive(Debug, Clone)]
pub enum Message {
    Angle(NoiseMessage),
    Radius(NoiseMessage),
    HueRotation(f32),
    Export,
    ExportComplete(()),
    Null,
}

#[derive(Debug, Clone)]
struct Controls {
    theta_noise: NoiseControls,
    radius_noise: NoiseControls,
    hue_rotation: f32,
    exporting: bool,
}

impl Default for Controls {
    fn default() -> Self {
        Self {
            theta_noise: NoiseControls::default(),
            radius_noise: NoiseControls {
                factor: 750.0,
                ..Default::default()
            },
            hue_rotation: 0.0,
            exporting: false,
        }
    }
}

#[derive(Debug, Clone)]
struct Warper {
    controls: Controls,
    image: image::Handle,
}

impl Warper {
    pub fn new() -> Self {
        let controls = Controls::default();
        let img_data = draw(&controls);
        let width = 4032;
        let height = 3024;
        Self {
            controls,
            image: image::Handle::from_pixels(width, height, img_data),
        }
    }

    pub fn draw(&mut self) {
        let img_data = draw(&self.controls);
        self.image = image::Handle::from_pixels(4032, 3024, img_data);
    }
}

impl Application for Warper {
    type Message = Message;
    type Theme = Theme;
    type Executor = iced::executor::Default;
    type Flags = ();

    fn new(_flags: ()) -> (Warper, Command<Message>) {
        (Self::new(), Command::none())
    }

    fn title(&self) -> String {
        String::from("Warper")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        use Message::*;
        match message {
            Angle(a) => {
                self.controls.theta_noise.update(a);
                self.draw()
            }
            Radius(r) => {
                self.controls.radius_noise.update(r);
                self.draw()
            }
            HueRotation(r) => {
                self.controls.hue_rotation = r;
                self.draw()
            }
            Export => {
                self.controls.exporting = true;
                return Command::perform(print(self.controls.clone()), ExportComplete);
            }
            ExportComplete(_) => self.controls.exporting = false,
            Null => {}
        }
        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        use Message::*;
        let img_view = image::viewer(self.image.clone()).min_scale(0.75);
        let img_container = Container::new(img_view).padding(20);
        let mut control_panel = Column::new();

        let angle = NoiseControls::new(
            self.controls.theta_noise.function,
            self.controls.theta_noise.factor,
            self.controls.theta_noise.scale,
            self.controls.theta_noise.octaves,
            self.controls.theta_noise.persistence,
            self.controls.theta_noise.lacunarity,
            self.controls.theta_noise.frequency,
            self.controls.theta_noise.sin_x_freq,
            self.controls.theta_noise.sin_y_freq,
            self.controls.theta_noise.sin_x_exp,
            self.controls.theta_noise.sin_y_exp,
        );
        control_panel = control_panel
            .push("Angle")
            .push(angle.view().map(Message::Angle));

        let radius = NoiseControls::new(
            self.controls.radius_noise.function,
            self.controls.radius_noise.factor,
            self.controls.radius_noise.scale,
            self.controls.radius_noise.octaves,
            self.controls.radius_noise.persistence,
            self.controls.radius_noise.lacunarity,
            self.controls.radius_noise.frequency,
            self.controls.radius_noise.sin_x_freq,
            self.controls.radius_noise.sin_y_freq,
            self.controls.radius_noise.sin_x_exp,
            self.controls.radius_noise.sin_y_exp,
        );
        control_panel = control_panel
            .push("Radius")
            .push(radius.view().map(Message::Radius))
            .push(NumericInput::new(
                "Hue Rotation".to_string(),
                self.controls.hue_rotation,
                0.0..=360.0,
                1.0,
                0,
                HueRotation,
            ));
        let export_button = if self.controls.exporting {
            button(text("Export").size(15))
        } else {
            button(text("Export").size(15)).on_press(Export)
        };
        control_panel = control_panel
            .push(export_button)
            .spacing(10)
            .padding(20)
            .width(250);
        row!(control_panel, img_container).into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}

fn draw(controls: &Controls) -> Vec<u8> {
    let img = open("./assets/greece.png").unwrap();
    let opts_theta = NoiseOpts::with_wh(img.width(), img.height())
        .factor(controls.theta_noise.factor)
        .scales(controls.theta_noise.scale);
    let nf_theta = choose_noise(&controls.theta_noise);
    let opts_r = NoiseOpts::with_wh(img.width(), img.height())
        .factor(controls.radius_noise.factor)
        .scales(controls.radius_noise.scale);
    let nf_r = choose_noise(&controls.radius_noise);
    let warp = Warp::new(
        Arc::new(move |z| {
            Complex32::new(
                noise2d(&nf_theta, &opts_theta, z.re, z.im),
                noise2d_01(&nf_r, &opts_r, z.re + 1.71, z.im + 4.13),
            )
        }),
        WarpNode::Img(&img, img.width() as f32, img.height() as f32),
        Coord::Polar,
    );

    let mut buffer: Vec<(u32, u32)> = Vec::with_capacity(4032 * 3024);
    for i in 0..3024 {
        for j in 0..4032 {
            buffer.push((j, i));
        }
    }
    let par_iter = buffer.par_iter().flat_map_iter(|p| {
        let t = warp
            .get(p.0 as f32, p.1 as f32)
            .rotate_hue(controls.hue_rotation)
            .as_u8s();
        vec![t.0, t.1, t.2, t.3]
    });
    let img_data: Vec<u8> = par_iter.collect();
    img_data
}

async fn print(controls: Controls) {
    let mut img_buf: ImageBuffer<Rgba<u8>, Vec<u8>> =
        ImageBuffer::from_vec(4032, 3024, draw(&controls)).unwrap();
    img_buf = imageops::resize(
        &img_buf,
        (4032.0 * 2.5) as u32,
        (3024.0 * 2.5) as u32,
        imageops::FilterType::CatmullRom,
    );
    let dirs = UserDirs::new().unwrap();
    let dir = dirs.download_dir().unwrap();
    let path = format!(r"{}/{}", dir.to_string_lossy(), "warp");
    let mut num = 0;
    let mut sketch = PathBuf::from(format!(r"{path}_{num}"));
    sketch.set_extension("png");
    while sketch.exists() {
        num += 1;
        sketch = PathBuf::from(format!(r"{path}_{num}"));
        sketch.set_extension("png");
    }
    img_buf.save(sketch).unwrap();
}
