use rand::seq::IteratorRandom;
use rand::RngCore;
use std::{collections::HashSet, ops::Range};
use wassily::prelude::*;

/// Generate a random opaque color from the Okhsl color space.
pub fn rand_okhsl_hue<R: RngCore>(rng: &mut R, hue_range: Range<f32>) -> Color {
    let normal = Normal::new(0.0, 0.25).unwrap();
    let h: f32 = rng.gen_range(hue_range);
    let s: f32 = 0.65 + normal.sample(rng);
    let l: f32 = 0.5 + normal.sample(rng);
    Okhsl::new(h, s.clamp(0.0, 1.0), l.clamp(0.0, 0.9)).to_color()
}

pub fn draw_dominos(width: u32, height: u32, size: u32) -> Canvas {
    let mut canvas = Canvas::new(width, height);
    let mut dominos = mk_dominos(width / (2 * size), height / size);
    let mut rng = SmallRng::seed_from_u64(0);
    for _ in 0..10_000 {
        let sq = squares(&dominos).into_iter().choose(&mut rng).unwrap();
        flip(&mut dominos, &sq);
    }
    for d in dominos {
        let p = pt(d.x as f32 * size as f32, d.y as f32 * size as f32);
        let color1 = rand_okhsl_hue(&mut rng, 30.0..60.0);
        let color2 = rand_okhsl_hue(&mut rng, 200.0..230.0);
        let stops = vec![
            GradientStop::new(0.0, color1),
            GradientStop::new(1.0, color2),
        ];
        match d.orientation {
            Orientation::Horizontal => {
                let grad = LinearGradient::new(
                    p,
                    p + pt(2.0 * size as f32, 0.0),
                    stops,
                    SpreadMode::Pad,
                    Transform::identity(),
                );
                let shader = paint_shader(grad.unwrap());
                Shape::new()
                    .rect_xywh(p, pt(2.0 * size as f32, size as f32))
                    .fill_paint(&shader)
                    .no_stroke()
                    .draw(&mut canvas)
            }
            Orientation::Vertical => {
                let grad = LinearGradient::new(
                    p,
                    p + pt(0.0, 2.0 * size as f32),
                    stops,
                    SpreadMode::Pad,
                    Transform::identity(),
                );
                let shader = paint_shader(grad.unwrap());
                Shape::new()
                    .rect_xywh(p, pt(size as f32, 2.0 * size as f32))
                    .fill_paint(&shader)
                    .no_stroke()
                    .draw(&mut canvas)
            }
        };
    }
    canvas
}

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Domino {
    x: i32,
    y: i32,
    orientation: Orientation,
}

impl Domino {
    fn new(x: i32, y: i32, orientation: Orientation) -> Self {
        Self { x, y, orientation }
    }
}

fn squares(dominos: &HashSet<Domino>) -> HashSet<(Domino, Domino)> {
    let mut sq = HashSet::new();
    for d1 in dominos.iter() {
        for d2 in dominos.iter() {
            if d1.orientation == Orientation::Horizontal {
                if (d2.orientation == Orientation::Horizontal) && d1.x == d2.x {
                    if d2.y == d1.y - 1 {
                        sq.insert((d2.clone(), d1.clone()));
                    } else if d2.y == d1.y + 1 {
                        sq.insert((d1.clone(), d2.clone()));
                    }
                }
            }
            if d1.orientation == Orientation::Vertical {
                if d2.orientation == Orientation::Vertical && d1.y == d2.y {
                    if d2.x == d1.x - 1 {
                        sq.insert((d2.clone(), d1.clone()));
                    } else if d2.x == d1.x + 1 {
                        sq.insert((d1.clone(), d2.clone()));
                    }
                }
            }
        }
    }
    sq
}

fn flip(dominos: &mut HashSet<Domino>, square: &(Domino, Domino)) {
    dominos.remove(&square.0);
    dominos.remove(&square.1);
    let o = &square.0.orientation;
    match o {
        Orientation::Horizontal => {
            dominos.insert(Domino::new(square.0.x, square.0.y, Orientation::Vertical));
            dominos.insert(Domino::new(
                square.0.x + 1,
                square.0.y,
                Orientation::Vertical,
            ));
        }
        Orientation::Vertical => {
            dominos.insert(Domino::new(square.0.x, square.0.y, Orientation::Horizontal));
            dominos.insert(Domino::new(
                square.0.x,
                square.0.y + 1,
                Orientation::Horizontal,
            ));
        }
    }
}

fn mk_dominos(x: u32, y: u32) -> HashSet<Domino> {
    let mut dominos = HashSet::new();
    for r in 0..x {
        for c in 0..y {
            dominos.insert(Domino::new(2 * r as i32, c as i32, Orientation::Horizontal));
        }
    }
    dominos
}
