use rand::Rng;
use std::f64;
use rand::prelude::ThreadRng;
use wasm_bindgen::prelude::*;
use web_sys::CanvasRenderingContext2d;

#[wasm_bindgen]
pub fn run() {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let ctx = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    ctx.set_fill_style_str("black");
    ctx.fill_rect(0., 0., canvas.width() as f64, canvas.height() as f64);

    let mut rng = rand::rng();

    let side = rng.random_ratio(canvas.height(), canvas.height() + canvas.width());

    let (a, b) = if side {
        (
            P {
                x: -100.,
                y: rng.random_range(0..canvas.height()) as f64 - 100.,
            },
            P {
                x: canvas.width() as f64 + 100.,
                y: rng.random_range(0..canvas.height()) as f64 + 100.,
            },
        )
    } else {
        (
            P {
                x: rng.random_range(0..canvas.width()) as f64 - 100.,
                y: -100.,
            },
            P {
                x: rng.random_range(0..canvas.width()) as f64 + 100.,
                y: canvas.height() as f64 + 100.,
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
        ctx.move_to(a.x, a.y);
        ctx.line_to(b.x, b.y);
        ctx.stroke();
    }

    for _ in 0..canvas.width() * canvas.height() / rng.random_range(500..1500) {
        let p = P {
            x: rng.random_range(0..canvas.width()) as f64,
            y: rng.random_range(0..canvas.height()) as f64,
        };

        p.draw(&ctx, &mut rng, false);
    }

    for _ in 0 .. rng.random_range(6..20){
        let p = P {
            x: rng.random_range(0..canvas.width()) as f64,
            y: rng.random_range(0..canvas.height()) as f64,
        };
        let size = rng.random_range(40. ..75.);
        for _ in 5 .. rng.random_range(10..50){
            let np = P{
                x: p.x + rng.random_range(-size .. size),
                y: p.y + rng.random_range(-size .. size),
            };
            np.draw(&ctx, &mut rng, false);
        }
    }
    for _ in 0 .. rng.random_range(8..20){
        let p = P {
            x: rng.random_range(0..canvas.width()) as f64,
            y: rng.random_range(0..canvas.height()) as f64,
        };
        p.draw(&ctx, &mut rng, true);
    }
}

impl P {
    fn draw(&self, ctx: &CanvasRenderingContext2d, rng: &mut ThreadRng, big:bool) {
        let color = format!(
            "rgb({r},{g},{b})",
            r = rng.random_range(150..255),
            g = rng.random_range(150..255),
            b = rng.random_range(150..255)
        );

        ctx.set_fill_style_str(&color);
        if big{
            let size = 5.;
            ctx.fill_rect(self.x, self.y, size, size);
        }else {
            let size = rng.random_range(0.1..1.9);
            ctx.fill_rect(self.x, self.y, size, size);
        }
    }
}

struct P {
    x: f64,
    y: f64,
}
