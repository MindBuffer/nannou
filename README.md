# nannou [![Build Status](https://travis-ci.org/nannou-org/nannou.svg?branch=master)](https://travis-ci.org/nannou-org/nannou) [![Crates.io](https://img.shields.io/crates/v/nannou.svg)](https://crates.io/crates/nannou) [![Crates.io](https://img.shields.io/crates/l/nannou.svg)](https://github.com/nannou-org/nannou/blob/master/LICENSE-MIT) [![docs.rs](https://docs.rs/nannou/badge.svg)](https://docs.rs/nannou/)

An open-source creative-coding toolkit for Rust.

**nannou** is a collection of code aimed at making it easy for artists to
express themselves with simple, fast, reliable, portable code.  Whether working
on a 12-month laser installation or a 5 minute sketch, this framework aims to
give artists easy access to the tools they need.

The project was started out of a desire for a creative coding framework inspired
by Processing, OpenFrameworks and Cinder, but for Rust. <sup>Named after
[this](https://www.youtube.com/watch?v=A-Pkx37kYf4).</sup>

### A Quick Note

This project is brand new and there is a lot of work to be done. Feel free to
help out!

## Getting Started

See what the code looks like by checking out [the
examples](https://github.com/nannou-org/nannou/tree/master/examples). See the
[following section](https://github.com/nannou-org/nannou#running-examples) to
learn how to run these examples.

If you're new to Rust, maybe check out [the official
  book](https://doc.rust-lang.org/book/)?

Start your own project with:
```
cargo new my_project
cd my_project
```
Add `nannou = "0.5"` under the `[dependencies]` line in your Cargo.toml.  This
is everything you need to use the framework in your own project or sketch.
Rust's package manager cargo will automatically download and install
everything you need!

## Running Examples

Clone this git repo (which contains the examples) to your computer.
```
git clone https://github.com/nannou-org/nannou
```
Change the current directory to `nannou`.
```
cd nannou
```
Run the example using cargo.
```
cargo run --release --example simple_window
```
The `--release` flag means we want to build with optimisations enabled.

To run any of the other examples, replace `simple_window` with the name of the
desired example.

## Goals

Nannou aims to provide easy, cross-platform access to the things that artists need:

- [x] **Windowing & Events** via [winit](https://crates.io/crates/winit).
- [x] **Audio** via [CPAL](https://crates.io/crates/cpal). *Input and
  output streams. Duplex are not yet supported.*
- [ ] **Video** input, playback and processing (*would love suggestions and
  ideas*).
- [x] **GUI** via [conrod](https://crates.io/crates/conrod). *May switch to a
  custom nannou solution in the future*.
- **Geometry** with functions and iterators for producing vertices and indices:
  - [x] 1D - `Scalar`, `Range`.
  - [x] 2D - `Rect`, `Line`, `Ellipse`, `Polygon`, `Polyline`, `Quad`,
    `Tri`.
  - [x] 3D - `Cuboid`.
  - [ ] 3D TODO - `Ellipsoid`, `Cube`, Prisms, Pyramids, *Hedrons, etc.
  - [x] Vertex & index iterators.
  - [x] [Graph](https://docs.rs/nannou/0.5.2/nannou/geom/graph/index.html) for
    composing geometry.
- **Graphics** currently via [glium](https://crates.io/crates/glium), will
  switch to [vulkano](https://github.com/vulkano-rs/vulkano) soon:
  - [x] [Draw](https://docs.rs/nannou/0.5.2/nannou/draw/index.html) API. E.g.
    `draw.ellipse().w_h(20.0, 20.0).color(RED)`.
  - [x] [Mesh](https://docs.rs/nannou/0.5.2/nannou/mesh/index.html) API.
  - [ ] Image API (currently only supported via GUI).
  - [ ] Framebuffer object API.
- **Protocols**:
  - [x] [OSC](https://docs.rs/nannou/0.5.2/nannou/osc/index.html) - Open Sound
    Control.
  - [x] [CITP](https://github.com/nannou-org/citp) - Controller Interface
    Transport Protocol (network implementation is in progress).
  - [x] [Ether-Dream](https://github.com/nannou-org/ether-dream) Laser DAC
    protocol and network implementation.
  - [x] [DMX via sACN](https://github.com/lschmierer/sacn) - commonly used for
    lighting and effects.
  - [x] [Serial](https://crates.io/crates/serial) - commonly used for
    interfacing with LEDs and other hardware.
  - [x] [MIDI](https://crates.io/crates/midir) - Musical Instrument Digital
    Interface.
  - [x] [UDP](https://doc.rust-lang.org/std/net/struct.UdpSocket.html) via
    std.
  - [x] TCP
    [streams](https://doc.rust-lang.org/std/net/struct.TcpStream.html) and
    [listeners](https://doc.rust-lang.org/std/net/struct.TcpListener.html)
    via std.
- **Device & I/O stream APIs**:
  - [x] [Audio](https://docs.rs/nannou/0.5.2/nannou/app/struct.Audio.html).
  - [ ] Video.
  - [ ] Lasers.
  - [ ] Lights.
  - [ ] LEDs.
- [ ] **Graphical Node Graph** via [gantz](https://github.com/nannou-org/gantz).
- [ ] **GUI Editor**.

Nannou aims to **use only pure-rust libraries**. New users should require
nothing more than `cargo build` to get going. Falling back to C-bindings will be
considered as a temporary solution in the case that there are no Rust
alternatives yet in development. We prefer to drive forward development of less
mature rust-alternatives than depend on bindings to C code. This should make it
easier for nannou *users* to become nannou *contributors* as they do not have to
learn a second language in order to contribute upstream.

Nannou **will not contain `unsafe` code** with the exception of bindings to
operating systems or hardware APIs if necessary.

Nannou wishes to **remove the need to decide between lots of different backends
that provide access to the same hardware**. Instead, we want to focus on a
specific set of backends and make sure that they work well.

## Why Rust?

Rust is a language that is both highly expressive and blazingly fast. Here are
some of the reasons why we choose to use it:

- **Super fast**, as in [C and
  C++ fast](https://benchmarksgame.alioth.debian.org/u64q/compare.php?lang=rust&lang2=gpp).
- [**A standard package manager**](https://crates.io/) that makes it very
  easy to handle dependencies and share your own projects in seconds.
- **Highly portable.** Easily build for MacOS, Linux, Windows, Android, iOS and
  [so many others](https://forge.rust-lang.org/platform-support.html).
- **No header files** (and no weird linking errors).
- **Sum Types and Pattern Matching** (and no `NULL`).
- **Local type inference**. Only write types where it matters, no need to repeat
  yourself.
- A more modern, **ƒunctional and expressive style**.
- **Memory safe and data-race-free!** Get your ideas down without the fear of
  creating pointer spaghetti or segfault time-sinks.
- **Immutability by default.** Easily distinguish between variables that can
  change and those that can't at a glance.
- **Module system** resulting in very clean and concise name spaces.
- One of the kindest internet communities we've come across (please visit
  mozilla's #rust or /r/rust if you're starting out and need any pointers)
