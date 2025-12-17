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
* Install Rust and Cargo (see [requirements](#Requirements))
* Run the applcation:
  ```console
  cargo run --release
  ```
Optionally, generate a standalone application bundle:
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

$$f(z) = (z^3 - 100) / (z^2 + 40)$$

<img src="/images/Cube_plot.png" alt="Plot1" width="400" height="300">
  
### 2. Transcendental function

$$f(z) = \frac{z}{\cos(z)}$$
 
<img src="/images/Cosine_plot.png" alt="Plot2" width="400" height="300">
  
### 3. Riemann Zeta function

$$\zeta(s) = \sum_{n=1}^{\infty}\frac{1}{n^s}$$
  
<img src="/images/Riemann_plot.png" alt="Plot3" width="400" height="300">

---

## Requirements

- Rust / Cargo

Install via:
  ```console
  curl https://sh.rustup.rs -sSf | sh
  ```
(Windows users: download [rustup-init.exe](https://win.rustup.rs/) from the official Rust website.) 

---

## Technical Notes

- Domain coloring encodes magnitude as lightness and argument (phase) as hue
- Designed as a mathematical exploration tool rather than a plotting library
- If plotting computationally expensive functions (such as Riemann Zeta), it can be helpful to reduce window size to increase performance.

---

## License

MIT License

