use directories::UserDirs;
use iced::{
    widget::{
        button, column, image, radio, row, scrollable, text, text_input, Column, Container, Rule,
    },
    Application, Command, Element, Settings, Theme,
};
use iced_native::widget::scrollable::Properties;
use rayon::prelude::*;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use wassily::prelude::{
    imageops, noise2d, noise2d_01, open, pt, Colorful, Coord, DynamicImage, ImageBuffer, NoiseOpts,
    Rgba, Warp, WarpNode,
};

mod gui;
mod noise;

use crate::gui::numeric_input::NumericInput;
use crate::noise::*;

static DEFAULT_IMAGE: &'static [u8] = include_bytes!("./default.raw");

pub fn main() -> iced::Result {
    env_logger::init();
    let mut settings = Settings::default();
    settings.window.size = (1430, 920);
    Warper::run(settings)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Coordinates {
    Polar,
    Cartesian,
    Absolute,
}

impl From<Coordinates> for String {
    fn from(coord: Coordinates) -> Self {
        match coord {
            Coordinates::Polar => "Polar".to_string(),
            Coordinates::Cartesian => "Cartesian".to_string(),
            Coordinates::Absolute => "Absolute".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    Angle(NoiseMessage),
    Radius(NoiseMessage),
    HueRotation(f32),
    Export,
    ExportComplete(()),
    PathSet(String),
    ImgPath,
    CoordinatesMessage(Coordinates),
    WidthSet(String),
    HeightSet(String),
    Null,
}

#[derive(Debug, Clone)]
struct Controls {
    img_path: String,
    theta_noise: NoiseControls,
    radius_noise: NoiseControls,
    hue_rotation: f32,
    coordinates: Option<Coordinates>,
    exporting: bool,
    export_width: String,
    export_height: String,
}

impl Default for Controls {
    fn default() -> Self {
        Self {
            img_path: String::from(""),
            theta_noise: NoiseControls::default(),
            radius_noise: NoiseControls {
                factor: 1000.0,
                ..Default::default()
            },
            hue_rotation: 0.0,
            coordinates: Some(Coordinates::Polar),
            export_width: String::from("inches / pixels"),
            export_height: String::from("inches / pixels"),
            exporting: false,
        }
    }
}

#[derive(Debug, Clone)]
struct Warper {
    controls: Controls,
    img: DynamicImage,
    image: image::Handle,
}

impl Warper {
    pub fn new() -> Self {
        let controls = Controls::default();
        let img = DynamicImage::ImageRgba8(
            ImageBuffer::from_raw(2000, 2000, DEFAULT_IMAGE.to_vec()).unwrap(),
        );
        let img_data = draw(&controls, &img);
        let image = image::Handle::from_pixels(img.width(), img.height(), img_data);
        Self {
            controls,
            img,
            image,
        }
    }

    pub fn draw(&mut self) {
        let img_data = draw(&self.controls, &self.img);
        self.image = image::Handle::from_pixels(self.img.width(), self.img.height(), img_data);
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
                return Command::perform(
                    print(self.controls.clone(), self.img.clone()),
                    ExportComplete,
                );
            }
            ExportComplete(_) => self.controls.exporting = false,
            PathSet(p) => self.controls.img_path = p,
            ImgPath => {
                self.img = match open(Path::new(&self.controls.img_path)) {
                    Ok(img) => img,
                    Err(_) => DynamicImage::ImageRgba8(
                        ImageBuffer::from_raw(2000, 2000, DEFAULT_IMAGE.to_vec()).unwrap(),
                    ),
                };
                self.draw()
            }
            CoordinatesMessage(c) => {
                self.controls.coordinates = Some(c);
                self.draw()
            }
            WidthSet(w) => {
                self.controls.export_width = w;
            }
            HeightSet(h) => self.controls.export_height = h,
            Null => {}
        }
        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        use Message::*;
        let img_view = image::viewer(self.image.clone()).min_scale(0.75);
        let img_container = Container::new(img_view).padding(20);
        let mut control_panel = Column::new()
            .push(text("Image Path").width(200))
            .spacing(15)
            .push(
                text_input("", &self.controls.img_path)
                    .on_input(PathSet)
                    .size(15)
                    .width(200)
                    .on_submit(ImgPath),
            )
            .push(Rule::horizontal(5))
            .push(
                column(
                    [
                        Coordinates::Polar,
                        Coordinates::Cartesian,
                        Coordinates::Absolute,
                    ]
                    .iter()
                    .cloned()
                    .map(|d| {
                        radio(d, d, self.controls.coordinates, CoordinatesMessage)
                            .text_size(15)
                            .size(15)
                    })
                    .map(Element::from)
                    .collect(),
                )
                .spacing(15),
            )
            .push(
                row!(
                    text("Width").size(15).width(90),
                    text("Height").size(15).width(90)
                )
                .spacing(15),
            )
            .push(
                row!(
                    text_input("", &self.controls.export_width)
                        .on_input(WidthSet)
                        .size(15)
                        .width(90),
                    text_input("", &self.controls.export_height)
                        .on_input(HeightSet)
                        .size(15)
                        .width(90)
                )
                .spacing(15),
            );
        let angle = NoiseControls::new(
            self.controls.theta_noise.function,
            self.controls.theta_noise.factor,
            self.controls.theta_noise.scale_x,
            self.controls.theta_noise.scale_y,
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
            .push(if self.controls.coordinates == Some(Coordinates::Polar) {
                text("Angle")
            } else {
                text("Warp Function")
            })
            .push(angle.view().map(Message::Angle));

        if self.controls.coordinates == Some(Coordinates::Polar) {
            let radius = NoiseControls::new(
                self.controls.radius_noise.function,
                self.controls.radius_noise.factor,
                self.controls.radius_noise.scale_x,
                self.controls.radius_noise.scale_y,
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
        }
        control_panel = control_panel.push(NumericInput::new(
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
        let scroll_panel = scrollable(control_panel)
            .vertical_scroll(Properties::new().width(5).margin(5).scroller_width(5));
        row!(scroll_panel, img_container).into()
    }

    fn theme(&self) -> Theme {
        Theme::Dark
    }
}

fn draw(controls: &Controls, img: &DynamicImage) -> Vec<u8> {
    let opts_theta = NoiseOpts::with_wh(img.width(), img.height())
        .factor(controls.theta_noise.factor)
        .y_scale(controls.theta_noise.scale_y)
        .x_scale(controls.theta_noise.scale_x);
    let nf_theta = choose_noise(&controls.theta_noise);
    let opts_r = NoiseOpts::with_wh(img.width(), img.height())
        .factor(controls.radius_noise.factor)
        .y_scale(controls.theta_noise.scale_y)
        .x_scale(controls.theta_noise.scale_x);
    let nf_r = choose_noise(&controls.radius_noise);
    let warp = match controls.coordinates.unwrap() {
        Coordinates::Polar => Warp::new(
            Arc::new(move |z| {
                pt(
                    noise2d(&nf_theta, &opts_theta, z.x, z.y),
                    noise2d_01(&nf_r, &opts_r, z.x + 1.71, z.y + 4.13),
                )
            }),
            WarpNode::Img(img, img.width() as f32, img.height() as f32),
            Coord::Polar,
        ),
        Coordinates::Cartesian => Warp::new(
            Arc::new(move |z| {
                pt(
                    noise2d(&nf_theta, &opts_theta, z.x, z.y),
                    noise2d(&nf_theta, &opts_theta, z.x + 1.71, z.y + 4.13),
                )
            }),
            WarpNode::Img(img, img.width() as f32, img.height() as f32),
            Coord::Cartesian,
        ),
        Coordinates::Absolute => Warp::new(
            Arc::new(move |z| {
                pt(
                    noise2d(&nf_theta, &opts_theta, z.x, z.y),
                    noise2d(&nf_theta, &opts_theta, z.x + 1.71, z.y + 4.13),
                )
            }),
            WarpNode::Img(img, img.width() as f32, img.height() as f32),
            Coord::Absolute,
        ),
    };
    let mut buffer: Vec<(u32, u32)> =
        Vec::with_capacity(img.width() as usize * img.height() as usize);
    for i in 0..img.height() {
        for j in 0..img.width() {
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

async fn print(controls: Controls, img: DynamicImage) {
    let mut img_buf: ImageBuffer<Rgba<u8>, Vec<u8>> =
        ImageBuffer::from_vec(img.width(), img.height(), draw(&controls, &img)).unwrap();

    let aspect_ratio = img.width() as f32 / img.height() as f32;
    let mut width = img.width() as u32;
    let mut height = img.height() as u32;
    if let Ok(w) = controls.export_width.parse::<f32>() {
        if w < 256.0 {
            width = (300.0 * w).round() as u32;
        } else {
            width = w as u32;
        }
        if let Ok(h) = controls.export_height.parse::<f32>() {
            if h < 256.0 {
                height = (300.0 * h).round() as u32;
            } else {
                height = h as u32;
            }
        } else {
            height = (width as f32 / aspect_ratio) as u32;
        }
    };
    img_buf = imageops::resize(&img_buf, width, height, imageops::FilterType::CatmullRom);
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
