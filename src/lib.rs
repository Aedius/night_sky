use crate::PointKind::{Big, Small};
use rand::prelude::ThreadRng;
use rand::{Rng, random_range};
use std::f64;
use wasm_bindgen::Clamped;
use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;
use web_sys::{ImageData, console};

#[wasm_bindgen]
pub fn run() {
    let window = web_sys::window().unwrap();

    let global_width_f: f64 = window.inner_width().unwrap().try_into().unwrap();
    let global_height_f: f64 = window.inner_height().unwrap().try_into().unwrap();

    let global_width: u32 = global_width_f as u32;
    let global_height: u32 = global_height_f as u32;

    let mut rng = rand::rng();

    let ctx = get_context(
        global_width,
        global_height,
    );

    generate_background_color(global_width, global_height, &ctx, &mut rng);

    generate_base_stars(global_width, global_height, &ctx, &mut rng);

    generate_cluster_stars(global_width, global_height, &ctx, &mut rng);

    generate_closest_stars(global_width, global_height, &ctx, &mut rng);
}

fn generate_closest_stars(
    global_width: u32,
    global_height: u32,
    ctx: &CanvasRenderingContext2d,
    mut rng: &mut ThreadRng,
) {
    for _ in 0..rng.random_range(10..40) {
        let p = Point {
            kind: Big,
            x: rng.random_range(0..global_width) as f64,
            y: rng.random_range(0..global_height) as f64,
        };
        p.draw(&ctx, &mut rng);
    }
}

fn generate_cluster_stars(
    global_width: u32,
    global_height: u32,
    ctx: &CanvasRenderingContext2d,
    mut rng: &mut ThreadRng,
) {
    for _ in 0..rng.random_range(15..40) {
        let p = Point {
            kind: Small,
            x: rng.random_range(0..global_width) as f64,
            y: rng.random_range(0..global_height) as f64,
        };
        let size = rng.random_range(40. ..75.);
        for _ in 5..rng.random_range(10..50) {
            let np = Point {
                kind: Small,
                x: p.x + rng.random_range(-size..size),
                y: p.y + rng.random_range(-size..size),
            };
            np.draw(&ctx, &mut rng);
        }
    }
}

fn generate_base_stars(
    global_width: u32,
    global_height: u32,
    ctx: &CanvasRenderingContext2d,
    mut rng: &mut ThreadRng,
) {
    for _ in 0..global_width * global_height / rng.random_range(300..900) {
        let p = Point {
            kind: Small,
            x: rng.random_range(0..global_width) as f64,
            y: rng.random_range(0..global_height) as f64,
        };

        p.draw(&ctx, &mut rng);
    }
}

fn generate_background_color(
    global_width: u32,
    global_height: u32,
    ctx: &CanvasRenderingContext2d,
    rng: &mut ThreadRng,
) {
    // based on : https://gist.github.com/donpark/1796361

    let image_data = ctx
        .get_image_data(0., 0., global_width as f64, global_height as f64)
        .map_err(|e| console::log_1(&e))
        .unwrap();

    let mut data = image_data.data();

    let alpha = 230u8;
    let base1 = rng.random_range(10u8..80u8);
    let base2 = rng.random_range(10u8..80u8);
    let base3 = rng.random_range(10u8..80u8);

    for (pos, chan) in data.iter_mut().enumerate() {
        match (pos + 1) % 4 {
            0 => {
                *chan = alpha;
            }
            1 => *chan = rng.random_range(0u8..base1),
            2 => *chan = rng.random_range(0u8..base2),
            _ => *chan = rng.random_range(0u8..base3),
        }
    }

    let slice_data = Clamped(&data.0[..]);
    let image_data =
        ImageData::new_with_u8_clamped_array_and_sh(slice_data, global_width, global_height)
            .map_err(|e| console::log_1(&e))
            .unwrap();

    ctx.put_image_data(&image_data, 0., 0.)
        .map_err(|e| console::log_1(&e))
        .unwrap();

    ctx.save();

    let mut size = 8;

    while size < global_width {
        let x = random_range(0..global_width - size);
        let y = random_range(0..global_height - size);
        ctx.set_global_alpha(4. / size as f64);
        ctx.draw_image_with_html_canvas_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
            &ctx.canvas().unwrap(),
            x as f64,
            y as f64,
            size as f64,
            size as f64,
            0.,
            0.,
            global_width as f64,
            global_height as f64,
        )
        .map_err(|e| console::log_1(&e))
        .unwrap();

        size *= 4;
    }

    ctx.restore()
}

fn get_context(
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

    ctx
}

impl Point {
    fn draw(&self, ctx: &CanvasRenderingContext2d, rng: &mut ThreadRng) {
        match self.kind {
            Small => self.draw_small(ctx, rng),
            Big => self.draw_big(ctx, rng),
        }
    }

    fn draw_small(&self, ctx: &CanvasRenderingContext2d, rng: &mut ThreadRng) {
        let color = format!(
            "rgb({r},{g},{b})",
            r = rng.random_range(150..255),
            g = rng.random_range(150..255),
            b = rng.random_range(150..255)
        );

        ctx.set_shadow_color(&color);
        ctx.set_shadow_blur(1.);
        ctx.set_fill_style_str(&color);

        let size = rng.random_range(0.1..1.9);
        ctx.fill_rect(self.x, self.y, size, size);

        ctx.set_shadow_blur(0.);
    }
    fn draw_big(&self, ctx: &CanvasRenderingContext2d, rng: &mut ThreadRng) {
        let size = rng.random_range(2. ..6.);

        let color_center = format!(
            "rgb({r},{g},{b})",
            r = rng.random_range(200..255),
            g = rng.random_range(200..255),
            b = rng.random_range(200..255)
        );
        let color_middle = format!(
            "rgba({r},{g},{b},0.5)",
            r = rng.random_range(150..255),
            g = rng.random_range(150..255),
            b = rng.random_range(150..255)
        );
        let color_outside = "rgb(0,0,0, 0)";

        let gradient = ctx
            .create_radial_gradient(self.x, self.y, size * 0.8, self.x, self.y, size)
            .map_err(|e| console::log_1(&e))
            .unwrap();

        gradient.add_color_stop(0., &color_center) .map_err(|e|
            console::log_1(&e)
        ).unwrap();
        gradient.add_color_stop(0.5, &color_middle) .map_err(|e|
            console::log_1(&e)
        ).unwrap();
        gradient.add_color_stop(1., color_outside) .map_err(|e|
            console::log_1(&e)
        ).unwrap();

        ctx.set_fill_style_canvas_gradient(&gradient);

        ctx.fill_rect(self.x - size, self.y - size, size * 2., size * 2.);
    }
}

#[derive(Debug)]
struct Point {
    kind: PointKind,
    x: f64,
    y: f64,
}

#[derive(Debug)]
enum PointKind {
    Small,
    Big,
}
