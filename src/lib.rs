use rand::prelude::ThreadRng;
use rand::Rng;
use std::f64;
use wasm_bindgen::prelude::*;
use web_sys::console;
use web_sys::js_sys::Math::{pow, sqrt};
use web_sys::CanvasRenderingContext2d;

#[wasm_bindgen]
pub fn run() {
    let window = web_sys::window().unwrap();

    let global_width_f: f64 = window.inner_width().unwrap().try_into().unwrap();
    let global_height_f: f64 = window.inner_height().unwrap().try_into().unwrap();

    let global_width: u32 = global_width_f as u32;
    let global_height: u32 = global_height_f as u32;

    let ctx = get_context(global_width_f, global_height_f, global_width, global_height);

    let mut rng = rand::rng();

    generate_galaxy(global_width, global_height, &ctx, &mut rng);

    generate_base_stars(global_width, global_height, &ctx, &mut rng);

    generate_cluster_stars(global_width, global_height, &ctx, &mut rng);

    generate_closest_stars(global_width, global_height, &ctx, &mut rng);

    generate_cloud(global_width, global_height, &ctx, &mut rng);
}

fn generate_cloud(
    global_width: u32,
    global_height: u32,
    ctx: &CanvasRenderingContext2d,
    mut rng: &mut ThreadRng,
) {
    let min_x = -0.5 * global_width as f64;
    let min_y = -0.5 * global_height as f64;
    let max_x = 1.5 * global_width as f64;
    let maw_y = 1.5 * global_height as f64;

    for _ in 0..rng.random_range(1..3) {
        ctx.begin_path();
        let start = Point {
            x: rng.random_range(min_x..max_x),
            y: rng.random_range(min_y..maw_y),
        };
        ctx.move_to(start.x, start.y);

        for _ in 0..rng.random_range(1..3) {
            let next = Point {
                x: rng.random_range(min_x..max_x),
                y: rng.random_range(min_y..maw_y),
            };

            let bezier = Point {
                x: rng.random_range(min_x..max_x),
                y: rng.random_range(min_y..maw_y),
            };

            ctx.quadratic_curve_to(bezier.x, bezier.y, next.x, next.y)
        }
        let bezier = Point {
            x: rng.random_range(min_x..max_x),
            y: rng.random_range(min_y..maw_y),
        };
        ctx.quadratic_curve_to(bezier.x, bezier.y, start.x, start.y);

        let grey = rng.random_range(0..50);

        let color = format!(
            "rgba({r},{g},{b},{a})",
            r = grey,
            g = grey,
            b = grey,
            a = rng.random_range(0.01..0.08),
        );

        ctx.set_shadow_color(&color);
        ctx.set_shadow_blur(200.);
        ctx.set_fill_style_str(&color);

        ctx.fill();
    }
}

fn generate_closest_stars(
    global_width: u32,
    global_height: u32,
    ctx: &CanvasRenderingContext2d,
    mut rng: &mut ThreadRng,
) {
    for _ in 0..rng.random_range(8..20) {
        let p = Point {
            x: rng.random_range(0..global_width) as f64,
            y: rng.random_range(0..global_height) as f64,
        };
        p.draw(&ctx, &mut rng, true);
    }
}

fn generate_cluster_stars(
    global_width: u32,
    global_height: u32,
    ctx: &CanvasRenderingContext2d,
    mut rng: &mut ThreadRng,
) {
    for _ in 0..rng.random_range(6..20) {
        let p = Point {
            x: rng.random_range(0..global_width) as f64,
            y: rng.random_range(0..global_height) as f64,
        };
        let size = rng.random_range(40. ..75.);
        for _ in 5..rng.random_range(10..50) {
            let np = Point {
                x: p.x + rng.random_range(-size..size),
                y: p.y + rng.random_range(-size..size),
            };
            np.draw(&ctx, &mut rng, false);
        }
    }
}

fn generate_base_stars(
    global_width: u32,
    global_height: u32,
    ctx: &CanvasRenderingContext2d,
    mut rng: &mut ThreadRng,
) {
    for _ in 0..global_width * global_height / rng.random_range(500..1500) {
        let p = Point {
            x: rng.random_range(0..global_width) as f64,
            y: rng.random_range(0..global_height) as f64,
        };

        p.draw(&ctx, &mut rng, false);
    }
}

fn generate_galaxy(
    global_width: u32,
    global_height: u32,
    ctx: &CanvasRenderingContext2d,
    rng: &mut ThreadRng,
) {
    let side = rng.random_ratio(global_height, global_height + global_width);

    let (start_side, end_side) = if side {
        (
            Point {
                x: -100.,
                y: rng.random_range(0..global_height) as f64 - 100.,
            },
            Point {
                x: global_width as f64 + 100.,
                y: rng.random_range(0..global_height) as f64 + 100.,
            },
        )
    } else {
        (
            Point {
                x: rng.random_range(0..global_width) as f64 - 100.,
                y: -100.,
            },
            Point {
                x: rng.random_range(0..global_width) as f64 + 100.,
                y: global_height as f64 + 100.,
            },
        )
    };

    let color = format!(
        "rgba({r},{g},{b},{a})",
        r = rng.random_range(150..255),
        g = rng.random_range(150..255),
        b = rng.random_range(150..255),
        a = rng.random_range(0.005..0.01),
    );
    ctx.set_stroke_style_str(&color);
    for i in 0..20 {
        ctx.set_line_width(i as f64 * 10.);
        ctx.move_to(start_side.x, start_side.y);
        ctx.line_to(end_side.x, end_side.y);
        ctx.stroke();
    }

    let length = sqrt(pow(end_side.x - start_side.x, 2.) + pow(end_side.y - start_side.y, 2.));

    let nb = rng.random_range(length / 7. ..length / 2.);

    let x_step = (end_side.x - start_side.x) / nb;
    let y_step = (end_side.y - start_side.y) / nb;

    let size = rng.random_range(25. ..40.);
    console::log_1(&format!("nb {nb} size {size}").into());

    for i in 0..nb as u32 {
        let s = Point {
            x: start_side.x + i as f64 * x_step + rng.random_range(-size..size),
            y: start_side.y + i as f64 * y_step + rng.random_range(-size..size),
        };
        s.draw(ctx, rng, false);
    }
}

fn get_context(
    global_width_f: f64,
    global_height_f: f64,
    global_width: u32,
    global_height: u32,
) -> CanvasRenderingContext2d {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();

    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    canvas.set_width(global_width);
    canvas.set_height(global_height);

    let ctx = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    ctx.set_fill_style_str("black");
    ctx.fill_rect(0., 0., global_width_f, global_height_f);
    ctx
}

impl Point {
    fn draw(&self, ctx: &CanvasRenderingContext2d, rng: &mut ThreadRng, big: bool) {
        let color = format!(
            "rgb({r},{g},{b})",
            r = rng.random_range(150..255),
            g = rng.random_range(150..255),
            b = rng.random_range(150..255)
        );

        ctx.set_fill_style_str(&color);
        if big {
            let size = 5.;
            ctx.fill_rect(self.x, self.y, size, size);
        } else {
            let size = rng.random_range(0.1..1.9);
            ctx.fill_rect(self.x, self.y, size, size);
        }
    }
}

#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}
