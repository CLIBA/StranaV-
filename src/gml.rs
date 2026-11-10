//! GML runtime semantics the original code relies on.

use rand::Rng;

/// GML treats a real as true when it is greater than 0.5.
#[inline]
pub fn truthy(x: f64) -> bool {
    x > 0.5
}

#[inline]
pub fn not(x: f64) -> f64 {
    if truthy(x) { 0.0 } else { 1.0 }
}

/// `random(n)`: uniform real in [0, n).
pub fn random(n: f64) -> f64 {
    n * rand::rng().random::<f64>()
}

pub fn random_range(a: f64, b: f64) -> f64 {
    a + random(b - a)
}

/// `irandom_range(a, b)`: integer in [a, b], inclusive.
pub fn irandom_range(a: i32, b: i32) -> i32 {
    let (lo, hi) = if a <= b { (a, b) } else { (b, a) };
    rand::rng().random_range(lo..=hi)
}

pub fn choose(options: &[f64]) -> f64 {
    options[rand::rng().random_range(0..options.len())]
}

/// GML `round` (banker's rounding, like the runner's FPU default).
pub fn round(x: f64) -> f64 {
    x.round_ties_even()
}

/// GML `x div y`.
pub fn floor_div(x: f64, y: f64) -> f64 {
    (x / y).trunc()
}

/// `string(x)` for reals: integers print plainly, everything else with two decimals.
pub fn string(x: f64) -> String {
    if x.fract() == 0.0 && x.abs() < 1e15 {
        format!("{}", x as i64)
    } else {
        format!("{x:.2}")
    }
}

/// The game's `ps(fmt, ...)` script: substitutes each `&` in turn.
pub fn ps(fmt: &str, args: &[String]) -> String {
    let mut out = fmt.to_string();
    for a in args {
        if let Some(i) = out.find('&') {
            out.replace_range(i..i + 1, a);
        }
    }
    out
}

/// `point_direction` in degrees, counter-clockwise with y pointing down.
pub fn point_direction(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    let d = (-(y2 - y1)).atan2(x2 - x1).to_degrees();
    if d < 0.0 { d + 360.0 } else { d }
}

pub fn point_distance(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    ((x2 - x1).powi(2) + (y2 - y1).powi(2)).sqrt()
}
