//! Minimal GameMaker particle system: just the features the finale (`particlez`) uses.

use serde::{Deserialize, Serialize};

use crate::canvas::{Canvas, Texture};
use crate::gml::{irandom_range, random_range};

#[derive(Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Shape {
    Line,
    Flare,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct PartType {
    pub shape: Shape,
    pub size: (f64, f64, f64),
    pub scale: (f64, f64),
    pub colors: [u32; 3],
    pub alphas: [f64; 3],
    pub speed: (f64, f64, f64),
    pub direction: (f64, f64, f64),
    pub orientation: (f64, f64, f64, bool),
    pub additive: bool,
    pub life: (i32, i32),
}

#[derive(Clone, Serialize, Deserialize)]
struct Particle {
    typ: usize,
    x: f64,
    y: f64,
    speed: f64,
    direction: f64,
    angle: f64,
    size: f64,
    age: i32,
    life: i32,
}

#[derive(Clone, Serialize, Deserialize)]
struct Emitter {
    x: f64,
    y: f64,
    typ: usize,
    per_step: i32,
}

#[derive(Clone, Default, Serialize, Deserialize)]
pub struct PartSystem {
    pub types: Vec<PartType>,
    emitters: Vec<Emitter>,
    particles: Vec<Particle>,
}

impl PartSystem {
    pub fn add_type(&mut self, t: PartType) -> usize {
        self.types.push(t);
        self.types.len() - 1
    }

    /// `part_emitter_create` + `part_emitter_region` (a point) + `part_emitter_stream`.
    pub fn stream(&mut self, x: f64, y: f64, typ: usize, per_step: i32) {
        self.emitters.push(Emitter { x, y, typ, per_step });
    }

    /// `part_particles_clear` for one type.
    pub fn clear_type(&mut self, typ: usize) {
        self.particles.retain(|p| p.typ != typ);
    }

    /// `part_system_clear`: removes particles and emitters.
    pub fn clear(&mut self) {
        self.particles.clear();
        self.emitters.clear();
    }

    fn spawn(&mut self, typ: usize, x: f64, y: f64) {
        let t = &self.types[typ];
        self.particles.push(Particle {
            typ,
            x,
            y,
            speed: random_range(t.speed.0, t.speed.1),
            direction: random_range(t.direction.0, t.direction.1),
            angle: random_range(t.orientation.0, t.orientation.1),
            size: random_range(t.size.0, t.size.1),
            age: 0,
            life: irandom_range(t.life.0, t.life.1),
        });
    }

    /// One step of automatic update.
    pub fn update(&mut self) {
        let types = &self.types;
        self.particles.retain_mut(|p| {
            p.age += 1;
            if p.age >= p.life {
                return false;
            }
            let t = &types[p.typ];
            p.speed = (p.speed + t.speed.2).max(0.0);
            p.direction += t.direction.2;
            p.angle += t.orientation.2;
            p.size = (p.size + t.size.2).max(0.0);
            let r = p.direction.to_radians();
            p.x += p.speed * r.cos();
            p.y -= p.speed * r.sin();
            true
        });
        for i in 0..self.emitters.len() {
            let e = self.emitters[i].clone();
            for _ in 0..e.per_step {
                self.spawn(e.typ, e.x, e.y);
            }
        }
    }

    pub fn draw(&self, cv: &mut Canvas, textures: &ParticleTextures) {
        for p in &self.particles {
            let t = &self.types[p.typ];
            let k = p.age as f64 / p.life as f64;
            let (color, alpha) = if k < 0.5 {
                (lerp_color(t.colors[0], t.colors[1], k * 2.0), lerp(t.alphas[0], t.alphas[1], k * 2.0))
            } else {
                (lerp_color(t.colors[1], t.colors[2], k * 2.0 - 1.0), lerp(t.alphas[1], t.alphas[2], k * 2.0 - 1.0))
            };
            let angle = p.angle + if t.orientation.3 { p.direction } else { 0.0 };
            let tex = match t.shape {
                Shape::Line => &textures.line,
                Shape::Flare => &textures.flare,
            };
            let prev = cv.additive;
            cv.additive = t.additive;
            cv.draw_transformed(tex, p.x, p.y, p.size * t.scale.0, p.size * t.scale.1, angle, color, alpha);
            cv.additive = prev;
        }
    }
}

fn lerp(a: f64, b: f64, k: f64) -> f64 {
    a + (b - a) * k
}

fn lerp_color(a: u32, b: u32, k: f64) -> u32 {
    crate::canvas::merge_color(a, b, k)
}

/// Stand-ins for GameMaker's built-in particle shapes (64×64, white with alpha).
pub struct ParticleTextures {
    pub line: Texture,
    pub flare: Texture,
}

impl ParticleTextures {
    pub fn new() -> Self {
        ParticleTextures { line: make_texture(line_alpha), flare: make_texture(flare_alpha) }
    }
}

fn make_texture(f: fn(f64, f64) -> f64) -> Texture {
    const N: i32 = 64;
    let mut rgba = Vec::with_capacity((N * N * 4) as usize);
    for y in 0..N {
        for x in 0..N {
            // Normalised coordinates in [-1, 1].
            let u = (x as f64 + 0.5) / N as f64 * 2.0 - 1.0;
            let v = (y as f64 + 0.5) / N as f64 * 2.0 - 1.0;
            let a = (f(u, v).clamp(0.0, 1.0) * 255.0) as u8;
            rgba.extend_from_slice(&[255, 255, 255, a]);
        }
    }
    Texture { w: N, h: N, rgba }
}

fn line_alpha(u: f64, v: f64) -> f64 {
    let along = 1.0 - u.abs().powf(4.0);
    let across = (1.0 - v.abs() * 8.0).max(0.0);
    along * across
}

fn flare_alpha(u: f64, v: f64) -> f64 {
    let r = (u * u + v * v).sqrt();
    let glow = (1.0 - r).max(0.0).powi(3);
    let rays = (1.0 - u.abs() * 14.0).max(0.0) * (1.0 - v.abs()) + (1.0 - v.abs() * 14.0).max(0.0) * (1.0 - u.abs());
    glow + rays * 0.6
}
