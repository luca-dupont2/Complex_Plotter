# Complex function plotter

An interactive application for **visualizing complex-valued functions** 
$$f : \mathbb{C} \rightarrow \mathbb{C}$$ using **domain coloring**.

This tool is designed to explore the analytic structure of complex functions — zeros, poles, branch behavior, and growth — through real-time graphical feedback.

---

## Motivation

Complex functions are often difficult to reason about from formulas alone.
By mapping function values to color and intensity, this project provides
an intuitive way to:

- Inspect singularities and poles  
- Visualize conformality and local behavior  
- Experiment with arbitrary analytic expressions interactively

---

## Features

- **Real-time domain coloring**
- Interactive **zoom and pan**
- Support for arbitrary complex functions (edit source)
- High-performance Rust implementation

---

## Installation

```bash
git clone https://github.com/luca-dupont2/Complex_Plotter.git
cd Complex_Plotter
```

---

## Usage :
* Install Rust and Cargo (see [requirements](#requirements))
* Run the application:
  ```console
  cargo run --release --locked
  ```
To generate a standalone application bundle, install the optional
[`cargo-bundle` command](https://github.com/burtonageo/cargo-bundle) separately:
  ```console
  cargo install cargo-bundle --locked
  ```
Then generate the bundle:
  ```console
  cargo bundle --release
  ```
The bundle will be located in:
  ```console
  target/release/bundle/
  ```

---

## Controls :
* ```+``` / ```-``` : Zoom in / out
* ```←``` ```→``` ```↓``` ```↑``` : Pan view

---

## Customizing the function

The complex function being visualized can be modified directly in the source code.  
See ```main.rs``` line 30, ```fn f(z : Complex<f32>) -> Complex<f32>```.

This design choice keeps the renderer simple while allowing full mathematical
flexibility.

---

## Sample images

### 1. Rational function

$$f(z) = \frac{z^3 - 100}{z^2 + 40}$$

<img src="/images/Cube_plot.png" alt="Plot1" width="400" height="300">
  
### 2. Transcendental function

$$f(z) = \frac{z}{\cos(z)}$$
 
<img src="/images/Cosine_plot.png" alt="Plot2" width="400" height="300">
  
### 3. Riemann Zeta function

$$\zeta(s) = \sum_{n=1}^{\infty}\frac{1}{n^s}$$
  
<img src="/images/Riemann_plot.png" alt="Plot3" width="400" height="300">

---

## Requirements

- Current stable Rust and Cargo

Install via:
  ```console
  curl https://sh.rustup.rs -sSf | sh
  ```
(Windows users: download [rustup-init.exe](https://win.rustup.rs/) from the official Rust website.) 

If Rust is already installed through rustup, update it before building:

```console
rustup update stable
```

If Cargo reports that `feature edition2024 is required`, the active Cargo
version cannot parse a dependency's Rust 2024 manifest. Update Rust, then check
`cargo --version` in the terminal you use to run the project. Rust 2024 support
starts with Rust 1.85, but newer dependencies can require a newer compiler.

`cargo-bundle` is an optional packaging tool. Keep it out of `[dependencies]`
so `cargo run` does not resolve its packaging dependencies, including `time-core`.

---

## Technical Notes

- Domain coloring encodes magnitude as lightness and argument (phase) as hue
- Designed as a mathematical exploration tool rather than a plotting library
- If plotting computationally expensive functions (such as Riemann Zeta), it can be helpful to reduce window size to increase performance.

---

## License

MIT License


### Viewport cache

Unchanged views reuse the rendered texture. During pan, zoom, or resize, the
renderer reuses visible samples within 0.25 of the smaller current pixel spacing
and evaluates only misses in parallel. Samples keep their original coordinates
to prevent cumulative drift, and samples outside the new view are discarded.
Storage is bounded by the pixel count, with old and new buffers during an update.

This is approximate sampling, not an error bound on function values. Fine
fractal boundaries and singularities can differ from a fresh render. Set
`TOLERANCE` in `src/cache.rs` to `0.0` for exact-coordinate reuse only.
