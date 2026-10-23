#![allow(unused)]
use macroquad::{color::hsl_to_rgb, miniquad::window::screen_size, prelude::*};
use num_complex::Complex;
use std::f32::consts::PI;
mod cache;
mod zeta;
use cache::{Cache, View};

// Constants
const ZOOM_SPEED: f32 = std::f32::consts::LN_2; // 2× per second
const PAN_SPEED: f32 = 1.; // Half the viewport width per second
const MAX_INPUT_DT: f32 = 0.25; // Avoid jumps after a pause or long render
const INF : f32 = f32::INFINITY;
const K: i32 = 2;
const MAX_LIGHTNESS: f32 = 0.7;
const SATURATION: f32 = 1.;
const START_BOUNDARIES: f32 = 10.;
const NUM_DIVISIONS : i32 = 4;
const TRANSPARENT_GREY: Color = Color{r : 220., g : 220., b : 220., a : 0.2};

// Map value from one range to another function that will be used a lot
fn map_value(value: f32, from_min: f32, from_max: f32, to_min: f32, to_max: f32) -> f32 {
    // Normalize value to the range [0, 1]
    let normalized_value = (value - from_min) / (from_max - from_min);

    // Scale normalized value to the new range
    normalized_value * (to_max - to_min) + to_min
}

// Function that will be applied to the complex plane
fn f(z : Complex<f32>) -> Complex<f32> {
    (z * z * z - 100.) / (z.powi(2) + 40.)
    // ((-1. / 2.) * (z * z)).exp()
    // z / z.cos()
    // z.sin() - 0.5

    // Riemann zeta:
    // let value = zeta::zeta(Complex::new(z.re as f64, z.im as f64));
    // Complex::new(value.re as f32, value.im as f32)
}

// Map a point value to a color
fn val_to_color(z : Complex<f32>) -> Color {
    let hue = map_value(z.arg(), -PI, PI, 0., 1.); // Map angle to hue

    let norm = z.norm();

    // Map norm to a lightness according to the function ||z||^K/(||z||^K+1) (Stereographic projection onto the Riemann Sphere with K = 2)
    let z_k = norm.powi(K);
    // Check if the value will be infinity, if so then make it very light, else let it do the function
    let lightness = if z_k != INF { MAX_LIGHTNESS * z_k / (z_k + 1.) } else {MAX_LIGHTNESS+0.1};

    // Get color according to norm and angle
    hsl_to_rgb(hue, SATURATION, lightness)
}

#[macroquad::main("Complex function plotter")]
async fn main() {
    // Initialize window and image
    let (width, height) = screen_size();

    let mut image = Image::gen_image_color(width as u16, height as u16, WHITE);
    let mut texture = Texture2D::from_image(&image);
    let mut cache = Cache::new();

    let (mut w, mut h) = (image.width(), image.height());

    // Initial graph boundaries
    let mut boundary = START_BOUNDARIES;

    // Graph translation offsets
    let mut x_offset = 0.;
    let mut y_offset = 0.;
    let mut last_scroll_time = get_time();

    loop {
        // Check for window resizing
        let (new_width, new_height) = screen_size();
        if new_width as usize != image.width() || new_height as usize != image.height() {
            image = Image::gen_image_color(new_width as u16, new_height as u16, WHITE);
            texture = Texture2D::from_image(&image);

            (w, h) = (image.width(), image.height());
        }

        clear_background(BLACK);

        // Rates use elapsed time, so rendering speed does not set movement speed.
        let dt = get_frame_time().min(MAX_INPUT_DT);
        let zoom = is_key_down(KeyCode::Minus) as i32 - is_key_down(KeyCode::Equal) as i32;
        let zoom_rate = zoom as f32 * ZOOM_SPEED;
        // Integrate the changing scale when panning and zooming together.
        let pan_distance = if zoom == 0 {
            boundary * dt
        } else {
            boundary * (zoom_rate * dt).exp_m1() / zoom_rate
        };
        boundary *= (zoom_rate * dt).exp();

        let direction = vec2(
            (is_key_down(KeyCode::Right) as i32 - is_key_down(KeyCode::Left) as i32) as f32,
            (is_key_down(KeyCode::Down) as i32 - is_key_down(KeyCode::Up) as i32) as f32,
        ).normalize_or_zero();
        let movement = direction * (pan_distance * PAN_SPEED);
        x_offset += movement.x;
        y_offset += movement.y;

        // Drag the plot with the left mouse button.
        if is_mouse_button_down(MouseButton::Left) && !is_mouse_button_pressed(MouseButton::Left) {
            let delta = mouse_delta_position() * boundary;
            x_offset += delta.x;
            y_offset += delta.y;
        }

        // Scroll to zoom, keeping the point under the cursor fixed.
        let scroll = mouse_wheel().1;
        if scroll != 0. {
            // Cap fast scrolling at 16× zoom per second.
            let now = get_time();
            let max_step = 4. * ZOOM_SPEED * (now - last_scroll_time).min(0.1) as f32;
            last_scroll_time = now;
            let new_boundary = boundary * (-scroll * 0.01).clamp(-max_step, max_step).exp();
            let anchor = mouse_position_local() * (boundary - new_boundary);
            x_offset += anchor.x;
            y_offset += anchor.y;
            boundary = new_boundary;
        }

        let view = View { width: w, height: h, boundary: boundary as f64, x: x_offset as f64, y: y_offset as f64 };
        if cache.update(view, |x, y| f(Complex::new(x as f32, y as f32))) {
            for (index, value) in cache.values().enumerate() {
                image.set_pixel((index % w) as u32, (index / w) as u32, val_to_color(value));
            }
            texture.update(&image);
        }

        draw_texture(&texture, 0., 0., WHITE);

        // Draw axes
        let x_axe_pos = map_value(
            0.,
            (-boundary) + x_offset,
            boundary + x_offset,
            0.,
            w as f32,
        );
        let y_axe_pos = map_value(
            0.,
            boundary - y_offset,
            -boundary - y_offset,
            0.,
            h as f32,
        );
        draw_line(x_axe_pos, 0., x_axe_pos, h as f32, 1., BLACK);
        draw_line(0., y_axe_pos, w as f32, y_axe_pos, 1., BLACK);

        // Get value at mouse position
        let (mut mx, mut my) = mouse_position();

        // Check if cursor is on screen, else put NaN
        if (0. ..w as f32).contains(&mx) && (0. ..h as f32).contains(&my) {
            mx = map_value(mx, 0., w as f32, -boundary + x_offset, boundary + x_offset);
            my = map_value(my, 0., h as f32, boundary - y_offset, -boundary - y_offset);
        } else {
            mx = f32::NAN;
            my = f32::NAN;
        }

        let z = Complex::new(mx,my);

        let res = f(z);

        let (re, im) = (res.re, res.im);

        // Output z and f(z) on the screen at mouse position
        let z_text = &format!("z = {mx:.3} + {my:.3}i");
        let fz_text = &format!("f(z) = {re:.3} + {im:.3}i");

        draw_rectangle(0.,0.,400., 75., TRANSPARENT_GREY);
        draw_text(z_text, 10., 30., 30., BLACK);
        draw_text(fz_text, 10., 60., 30., BLACK);

        // Draw axes divisions and values
        let x_division_spacing = w as f32 / NUM_DIVISIONS as f32;
        let y_division_spacing = h as f32 / NUM_DIVISIONS as f32;

        for i in 0..=NUM_DIVISIONS {
            let y = i as f32 * y_division_spacing;
            let x = i as f32 * x_division_spacing;

            // Draw labels
            let x_value = map_value(x, 0., w as f32, -boundary + x_offset, boundary + x_offset);
            let y_value = map_value(y, 0., h as f32, boundary - y_offset, -boundary - y_offset);

            draw_text(&format!("{:.1}", x_value), x-8., y_axe_pos - 10., 20., BLACK);
            draw_line(x, y_axe_pos - 5., x, y_axe_pos + 5., 2., BLACK);

            draw_text(&format!("{:.1}", y_value), x_axe_pos + 10., y+4., 20., BLACK);
            draw_line(x_axe_pos - 5., y, x_axe_pos+5., y, 2., BLACK);
        }

        next_frame().await;
    }
}
