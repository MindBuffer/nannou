//! Color items, including everything from rgb, hsb/l/v, lap, alpha, luma and more, provided by the
//! palette crate. See [the palette docs](https://docs.rs/palette) for more details or see the
//! [**named**](./named/index.html) module for a set of provided color constants.

pub mod conv;

pub use self::conv::IntoLinSrgba;
pub use self::named::*;
#[doc(inline)]
pub use palette::*;

/// The default scalar value for working with color components, hues, etc.
pub type DefaultScalar = f32;

/// A color represented as red, green and blue intensities.
///
/// This type is an alias for the `Srgb` type, a type that represents the sRGB color space.
///
/// If you are looking for more advanced control over the RGB space and component type, please see
/// the `palette` crate's generic `Rgb` type.
pub type Rgb<S = DefaultScalar> = Srgb<S>;

/// The same as `Rgb`, but with an alpha value representing opacity.
///
/// This type is an alias for the `Srgba` type, a type that represents the sRGB color space
/// alongside an alpha value.
///
/// If you are looking for more advanced control over the RGB space and component type, please see
/// the `palette` crate's generic `Rgb` type.
pub type Rgba<S = DefaultScalar> = Srgba<S>;

/// The same as `Rgb`, but with `u8`'s.
pub type Rgb8 = Rgb<u8>;

/// A short-hand constructor for `Rgb::new`.
pub fn rgb<T>(r: T, g: T, b: T) -> Rgb<T>
where
    T: Component,
{
    srgb(r, g, b)
}

/// A short-hand constructor for `Rgb::<u8>::new` .
pub fn rgb8(r: u8, g: u8, b: u8) -> Rgb8 {
    srgb(r, g, b)
}

/// A short-hand constructor for `Rgba::new`.
pub fn rgba<T>(r: T, g: T, b: T, a: T) -> Rgba<T>
where
    T: Component,
{
    srgba(r, g, b, a)
}

/// A short-hand constructor for `Srgb::new`.
pub fn srgb<T>(r: T, g: T, b: T) -> Srgb<T>
where
    T: Component,
{
    Srgb::new(r, g, b)
}

/// A short-hand constructor for `Srgba::new`.
pub fn srgba<T>(r: T, g: T, b: T, a: T) -> Srgba<T>
where
    T: Component,
{
    Srgba::new(r, g, b, a)
}

/// A short-hand constructor for `LinSrgb::new`.
pub fn lin_srgb<T>(r: T, g: T, b: T) -> LinSrgb<T>
where
    T: Component,
{
    LinSrgb::new(r, g, b)
}

/// A short-hand constructor for `LinSrgba::new`.
pub fn lin_srgba<T>(r: T, g: T, b: T, a: T) -> LinSrgba<T>
where
    T: Component,
{
    LinSrgba::new(r, g, b, a)
}

/// A short-hand constructor for `Hsl::new(RgbHue::from_degrees(h * 360.0), s, l)`.
///
/// The given hue expects a value between `0.0` and `1.0` where `0.0` is 0 degress and `1.0` is
/// 360 degrees (or 2 PI radians).
pub fn hsl(h: f32, s: f32, l: f32) -> Hsl {
    Hsl::new(RgbHue::from_degrees(h * 360.0), s, l)
}

/// A short-hand constructor for `Hsla::new(RgbHue::from_degrees(h * 360.0), s, l, a)`.
///
/// The given hue expects a value between `0.0` and `1.0` where `0.0` is 0 degress and `1.0` is
/// 360 degrees (or 2 PI radians).
pub fn hsla(h: f32, s: f32, l: f32, a: f32) -> Hsla {
    Hsla::new(RgbHue::from_degrees(h * 360.0), s, l, a)
}

/// A short-hand constructor for `Hsv::new(RgbHue::from_degrees(h * 360.0), s, v)`.
///
/// The given hue expects a value between `0.0` and `1.0` where `0.0` is 0 degress and `1.0` is
/// 360 degrees (or 2 PI radians).
pub fn hsv(h: f32, s: f32, v: f32) -> Hsv {
    Hsv::new(RgbHue::from_degrees(h * 360.0), s, v)
}

/// A short-hand constructor for `Hsva::new(RgbHue::from_degrees(h * 360.0), s, v, a)`.
///
/// The given hue expects a value between `0.0` and `1.0` where `0.0` is 0 degress and `1.0` is
/// 360 degrees (or 2 PI radians).
pub fn hsva(h: f32, s: f32, v: f32, a: f32) -> Hsva {
    Hsva::new(RgbHue::from_degrees(h * 360.0), s, v, a)
}
