use directories::UserDirs;
use iced::{
    widget::{
        button, column, image, radio, row, scrollable, text, text_input, toggler, Column,
        Container, Rule,
    },
    Application, Command, Element, Settings, Theme,
};
use iced_native::widget::scrollable::Properties;
use rayon::prelude::*;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use wassily::prelude::{
    imageops, img_noise, noise2d, noise2d_01, open, pt, Colorful, Coord, DynamicImage, ImageBuffer,
    NoiseOpts, Rgba, Seedable, Warp, WarpNode,
};

mod dominos;
mod gui;
mod noise;

use crate::gui::numeric_input::NumericInput;
use crate::noise::*;
use dominos::draw_dominos;

const WIDTH: u32 = 4800;
const HEIGHT: u32 = 3600;
const SIZE: u32 = 400;

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
    Sync(bool),
    WarpTwice(bool),
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
    sync: bool,
    warp_twice: bool,
}

impl Default for Controls {
    fn default() -> Self {
        Self {
            img_path: String::from(""),
            theta_noise: NoiseControls {
                factor: 500.0,
                ..Default::default()
            },
            radius_noise: NoiseControls {
                factor: 1000.0,
                ..Default::default()
            },
            hue_rotation: 0.0,
            coordinates: Some(Coordinates::Cartesian),
            export_width: String::from("inches / pixels"),
            export_height: String::from("inches / pixels"),
            exporting: false,
            sync: true,
            warp_twice: false,
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
        let img_data = draw_dominos(WIDTH, HEIGHT, SIZE).pixmap.take();
        let img = DynamicImage::ImageRgba8(ImageBuffer::from_raw(WIDTH, HEIGHT, img_data).unwrap());
        let art_data = draw(&controls, &img);
        let image = image::Handle::from_pixels(img.width(), img.height(), art_data);
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

    async fn print(handle: image::Handle, controls: Controls) {
        let (w, h, pixels) = match handle.data() {
            iced_native::image::Data::Path(_) => unreachable!(),
            iced_native::image::Data::Bytes(_) => unreachable!(),
            iced_native::image::Data::Rgba {
                width,
                height,
                pixels,
            } => (width, height, pixels),
        };
        let img_buf: ImageBuffer<Rgba<u8>, Vec<u8>> =
            ImageBuffer::from_vec(*w, *h, pixels.to_vec()).unwrap();
        let aspect_ratio = *w as f32 / *h as f32;
        let (mut width, mut height) = (*w, *h);
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
        let img_buf = imageops::resize(&img_buf, width, height, imageops::FilterType::CatmullRom);
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
                if self.controls.theta_noise.dirty {
                    self.draw()
                }
            }
            Radius(r) => {
                self.controls.radius_noise.update(r);
                if self.controls.radius_noise.dirty {
                    self.draw()
                }
            }
            HueRotation(r) => {
                self.controls.hue_rotation = r;
                self.draw()
            }
            Export => {
                self.controls.exporting = true;
                let handel = self.image.clone();
                return Command::perform(
                    Warper::print(handel, self.controls.clone()),
                    ExportComplete,
                );
            }
            ExportComplete(_) => self.controls.exporting = false,
            PathSet(p) => {
                self.controls.img_path = p;
            }
            ImgPath => {
                self.img = match open(Path::new(&self.controls.img_path)) {
                    Ok(img) => img,
                    Err(_) => {
                        let img_data = draw_dominos(WIDTH, HEIGHT, SIZE).pixmap.take();
                        DynamicImage::ImageRgba8(
                            ImageBuffer::from_raw(WIDTH, HEIGHT, img_data).unwrap(),
                        )
                    }
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
            Sync(b) => {
                self.controls.sync = b;
                self.draw()
            }
            WarpTwice(b) => {
                self.controls.warp_twice = b;
                self.draw()
            }
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
            .push(Container::new(
                toggler("Sync".to_owned(), self.controls.sync, Sync).text_size(15),
            ))
            .push(Container::new(
                toggler("Warp Twide".to_owned(), self.controls.warp_twice, WarpTwice).text_size(15),
            ))
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
            self.controls.theta_noise.frequency,
            self.controls.theta_noise.sin_x_freq,
            self.controls.theta_noise.sin_y_freq,
            self.controls.theta_noise.img_noise_path.clone(),
            self.controls.theta_noise.img.clone(),
            self.controls.theta_noise.img_color_map,
            self.controls.theta_noise.dirty,
        );
        control_panel = control_panel
            .push(if self.controls.coordinates == Some(Coordinates::Polar) {
                text("Angle")
            } else {
                text("X Coordinate")
            })
            .push(angle.view().map(Message::Angle));

        if !self.controls.sync {
            let radius = NoiseControls::new(
                self.controls.radius_noise.function,
                self.controls.radius_noise.factor,
                self.controls.radius_noise.scale_x,
                self.controls.radius_noise.scale_y,
                self.controls.radius_noise.octaves,
                self.controls.radius_noise.frequency,
                self.controls.radius_noise.sin_x_freq,
                self.controls.radius_noise.sin_y_freq,
                self.controls.radius_noise.img_noise_path.clone(),
                self.controls.radius_noise.img.clone(),
                self.controls.radius_noise.img_color_map,
                self.controls.radius_noise.dirty,
            );
            control_panel =
                control_panel.push(if self.controls.coordinates == Some(Coordinates::Polar) {
                    text("Radius")
                } else {
                    text("Y Coordinate")
                });
            control_panel = control_panel.push(radius.view().map(Message::Radius))
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
    let opts_r = if controls.sync {
        let factor = if controls.coordinates == Some(Coordinates::Polar) {
            30.0 * opts_theta.factor
        } else {
            opts_theta.factor
        };
        opts_theta.factor(factor)
    } else {
        NoiseOpts::with_wh(img.width(), img.height())
            .factor(controls.radius_noise.factor)
            .y_scale(controls.radius_noise.scale_y)
            .x_scale(controls.radius_noise.scale_x)
    };
    let nf_r = if controls.sync {
        let mut tn = controls.theta_noise.clone();
        tn.img_color_map = Some(img_noise::ColorMap::RedGreen);
        choose_noise(&tn)
    } else {
        choose_noise(&controls.radius_noise).set_seed(98713)
    };

    let warpx = nf_theta.clone();
    let warpy = nf_r.clone();

    let warp = match controls.coordinates.unwrap() {
        Coordinates::Polar => Warp::new(
            Arc::new(move |z| {
                pt(
                    noise2d(&nf_theta, &opts_theta, z.x, z.y),
                    noise2d_01(&nf_r, &opts_r, z.x, z.y),
                )
            }),
            WarpNode::Img(img, img.width() as f32, img.height() as f32),
            Coord::Polar,
        ),
        Coordinates::Cartesian => Warp::new(
            Arc::new(move |z| {
                pt(
                    noise2d(&nf_theta, &opts_theta, z.x, z.y),
                    noise2d(&nf_r, &opts_r, z.x, z.y),
                )
            }),
            WarpNode::Img(img, img.width() as f32, img.height() as f32),
            Coord::Cartesian,
        ),
        Coordinates::Absolute => Warp::new(
            Arc::new(move |z| {
                pt(
                    noise2d(&nf_theta, &opts_theta, z.x, z.y),
                    noise2d(&nf_r, &opts_r, z.x, z.y),
                )
            }),
            WarpNode::Img(img, img.width() as f32, img.height() as f32),
            Coord::Absolute,
        ),
    };

    let warp2;
    if controls.warp_twice {
        warp2 = Warp::new(
            Arc::new(move |z| {
                pt(
                    noise2d(&warpx, &opts_theta, z.x, z.y),
                    noise2d(&warpy, &opts_r, z.x, z.y),
                )
            }),
            WarpNode::More(Arc::new(warp)),
            Coord::Cartesian,
        );
    } else {
        warp2 = warp;
    }

    let mut buffer: Vec<(u32, u32)> =
        Vec::with_capacity(img.width() as usize * img.height() as usize);
    for i in 0..img.height() {
        for j in 0..img.width() {
            buffer.push((j, i));
        }
    }
    let par_iter = buffer.par_iter().flat_map_iter(|p| {
        let t = warp2
            .get_wrapped(p.0 as f32, p.1 as f32)
            .rotate_hue(controls.hue_rotation)
            .as_u8s();
        vec![t.0, t.1, t.2, t.3]
    });
    let img_data: Vec<u8> = par_iter.collect();
    img_data
}
