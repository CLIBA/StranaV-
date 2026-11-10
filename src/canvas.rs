//! A small software renderer reproducing GameMaker: Studio 1.x immediate-mode drawing
//! (`draw_rectangle`, `draw_line`, `draw_text`, `draw_sprite`, blend modes and alpha)
//! into an RGBA framebuffer that Bevy then shows on screen.

use crate::gmdata::{FontDef, GameData, Page, Tpag};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum HAlign {
    Left,
    Center,
}

/// GameMaker colours are 0x00BBGGRR.
pub fn gm_rgb(c: u32) -> [u8; 3] {
    [(c & 0xff) as u8, ((c >> 8) & 0xff) as u8, ((c >> 16) & 0xff) as u8]
}

pub const C_BLACK: u32 = 0x000000;
pub const C_WHITE: u32 = 0xffffff;
pub const C_LTGRAY: u32 = 0xc0c0c0;
pub const C_GRAY: u32 = 0x808080;
pub const C_RED: u32 = 0x0000ff;
pub const C_BLUE: u32 = 0xff0000;
pub const C_YELLOW: u32 = 0x00ffff;
pub const C_AQUA: u32 = 0xffff00;
pub const C_MAROON: u32 = 0x000080;
pub const C_PURPLE: u32 = 0x800080;

/// `merge_color(a, b, amount)`.
pub fn merge_color(a: u32, b: u32, amount: f64) -> u32 {
    let (a, b) = (gm_rgb(a), gm_rgb(b));
    let ch = |i: usize| (a[i] as f64 * (1.0 - amount) + b[i] as f64 * amount) as u32;
    ch(0) | (ch(1) << 8) | (ch(2) << 16)
}

/// A texture the particle system draws with (generated, not from the game data).
pub struct Texture {
    pub w: i32,
    pub h: i32,
    pub rgba: Vec<u8>,
}

#[derive(Clone)]
pub struct Canvas {
    pub w: i32,
    pub h: i32,
    pub px: Vec<u8>,
    pub color: u32,
    pub alpha: f64,
    pub additive: bool,
    pub font: usize,
    pub halign: HAlign,
}

impl Canvas {
    pub fn new(w: i32, h: i32) -> Self {
        Canvas {
            w,
            h,
            px: vec![0; (w * h * 4) as usize],
            color: C_BLACK,
            alpha: 1.0,
            additive: false,
            font: 0,
            halign: HAlign::Left,
        }
    }

    pub fn clear(&mut self, color: u32) {
        let [r, g, b] = gm_rgb(color);
        for p in self.px.chunks_exact_mut(4) {
            p.copy_from_slice(&[r, g, b, 255]);
        }
    }

    #[inline]
    fn blend(&mut self, x: i32, y: i32, rgb: [f32; 3], a: f32) {
        if x < 0 || y < 0 || x >= self.w || y >= self.h || a <= 0.0 {
            return;
        }
        let i = ((y * self.w + x) * 4) as usize;
        let p = &mut self.px[i..i + 3];
        if self.additive {
            for c in 0..3 {
                p[c] = (p[c] as f32 + rgb[c] * a).min(255.0) as u8;
            }
        } else if a >= 1.0 {
            for c in 0..3 {
                p[c] = rgb[c] as u8;
            }
        } else {
            for c in 0..3 {
                p[c] = (rgb[c] * a + p[c] as f32 * (1.0 - a) + 0.5) as u8;
            }
        }
    }

    fn draw_rgb(&self) -> [f32; 3] {
        let [r, g, b] = gm_rgb(self.color);
        [r as f32, g as f32, b as f32]
    }

    /// `draw_rectangle(x1, y1, x2, y2, outline)`; both corners are inclusive.
    pub fn rect(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, outline: bool) {
        let (x1, x2) = (x1.min(x2).floor() as i32, x1.max(x2).floor() as i32);
        let (y1, y2) = (y1.min(y2).floor() as i32, y1.max(y2).floor() as i32);
        let rgb = self.draw_rgb();
        let a = self.alpha as f32;
        if outline {
            for x in x1..=x2 {
                self.blend(x, y1, rgb, a);
                if y2 != y1 {
                    self.blend(x, y2, rgb, a);
                }
            }
            for y in y1 + 1..y2 {
                self.blend(x1, y, rgb, a);
                if x2 != x1 {
                    self.blend(x2, y, rgb, a);
                }
            }
        } else {
            for y in y1.max(0)..=y2.min(self.h - 1) {
                for x in x1.max(0)..=x2.min(self.w - 1) {
                    self.blend(x, y, rgb, a);
                }
            }
        }
    }

    /// `draw_line(x1, y1, x2, y2)`, one pixel wide.
    pub fn line(&mut self, x1: f64, y1: f64, x2: f64, y2: f64) {
        let (mut x, mut y) = (x1.floor() as i32, y1.floor() as i32);
        let (xe, ye) = (x2.floor() as i32, y2.floor() as i32);
        let (dx, dy) = ((xe - x).abs(), -(ye - y).abs());
        let (sx, sy) = (if x < xe { 1 } else { -1 }, if y < ye { 1 } else { -1 });
        let mut e = dx + dy;
        let rgb = self.draw_rgb();
        let a = self.alpha as f32;
        loop {
            self.blend(x, y, rgb, a);
            if x == xe && y == ye {
                break;
            }
            let e2 = 2 * e;
            if e2 >= dy {
                e += dy;
                x += sx;
            }
            if e2 <= dx {
                e += dx;
                y += sy;
            }
        }
    }

    /// Copy a region of a texture page, tinted, at 1:1 scale.
    pub fn blit(&mut self, page: &Page, sx: i32, sy: i32, sw: i32, sh: i32, dx: i32, dy: i32, tint: u32, alpha: f64) {
        let [tr, tg, tb] = gm_rgb(tint);
        let (tr, tg, tb) = (tr as f32 / 255.0, tg as f32 / 255.0, tb as f32 / 255.0);
        for j in 0..sh {
            let y = dy + j;
            if y < 0 || y >= self.h {
                continue;
            }
            for i in 0..sw {
                let x = dx + i;
                if x < 0 || x >= self.w {
                    continue;
                }
                let s = (((sy + j) as u32 * page.w + (sx + i) as u32) * 4) as usize;
                let t = &page.rgba[s..s + 4];
                let a = t[3] as f32 / 255.0 * alpha as f32;
                self.blend(x, y, [t[0] as f32 * tr, t[1] as f32 * tg, t[2] as f32 * tb], a);
            }
        }
    }

    /// Texture page region stretched over a destination rectangle (nearest neighbour).
    pub fn blit_stretched(&mut self, page: &Page, t: &Tpag, dx: i32, dy: i32, dw: i32, dh: i32) {
        if dw <= 0 || dh <= 0 {
            return;
        }
        for j in 0..dh {
            let y = dy + j;
            if y < 0 || y >= self.h {
                continue;
            }
            let v = t.src_y as i32 + (j * t.src_h as i32 / dh);
            for i in 0..dw {
                let x = dx + i;
                if x < 0 || x >= self.w {
                    continue;
                }
                let u = t.src_x as i32 + (i * t.src_w as i32 / dw);
                let s = ((v as u32 * page.w + u as u32) * 4) as usize;
                let p = &page.rgba[s..s + 4];
                self.blend(x, y, [p[0] as f32, p[1] as f32, p[2] as f32], p[3] as f32 / 255.0);
            }
        }
    }

    /// Draw a texture page item placed in its frame box at (x, y) — sprites and backgrounds.
    pub fn draw_tpag(&mut self, data: &GameData, t: &Tpag, x: f64, y: f64, tint: u32, alpha: f64) {
        let page = &data.pages[t.page as usize];
        let dx = x.floor() as i32 + t.dst_x as i32;
        let dy = y.floor() as i32 + t.dst_y as i32;
        self.blit(page, t.src_x as i32, t.src_y as i32, t.src_w as i32, t.src_h as i32, dx, dy, tint, alpha);
    }

    /// Scaled and rotated texture centred on (cx, cy); used by the particle system.
    pub fn draw_transformed(
        &mut self,
        tex: &Texture,
        cx: f64,
        cy: f64,
        xscale: f64,
        yscale: f64,
        angle_deg: f64,
        color: u32,
        alpha: f64,
    ) {
        if xscale == 0.0 || yscale == 0.0 || alpha <= 0.0 {
            return;
        }
        let (s, c) = (-angle_deg.to_radians()).sin_cos();
        let hw = tex.w as f64 * xscale.abs() / 2.0;
        let hh = tex.h as f64 * yscale.abs() / 2.0;
        let ex = (hw * c).abs() + (hh * s).abs();
        let ey = (hw * s).abs() + (hh * c).abs();
        let x0 = ((cx - ex).floor() as i32).max(0);
        let x1 = ((cx + ex).ceil() as i32).min(self.w - 1);
        let y0 = ((cy - ey).floor() as i32).max(0);
        let y1 = ((cy + ey).ceil() as i32).min(self.h - 1);
        let [r, g, b] = gm_rgb(color);
        let (r, g, b) = (r as f32 / 255.0, g as f32 / 255.0, b as f32 / 255.0);
        for y in y0..=y1 {
            for x in x0..=x1 {
                let (px, py) = (x as f64 + 0.5 - cx, y as f64 + 0.5 - cy);
                // Rotate back into texture space.
                let lx = px * c + py * s;
                let ly = -px * s + py * c;
                let u = (lx / xscale + tex.w as f64 / 2.0).floor() as i32;
                let v = (ly / yscale + tex.h as f64 / 2.0).floor() as i32;
                if u < 0 || v < 0 || u >= tex.w || v >= tex.h {
                    continue;
                }
                let i = ((v * tex.w + u) * 4) as usize;
                let t = &tex.rgba[i..i + 4];
                let a = t[3] as f32 / 255.0 * alpha as f32;
                self.blend(x, y, [t[0] as f32 * r, t[1] as f32 * g, t[2] as f32 * b], a);
            }
        }
    }

    pub fn text_width(font: &FontDef, line: &str) -> i32 {
        line.chars().map(|ch| font.glyphs.get(&ch).map_or(0, |g| g.shift as i32)).sum()
    }

    /// `draw_text(x, y, s)`: `#` starts a new line, as in GameMaker 1.x.
    pub fn text(&mut self, data: &GameData, x: f64, y: f64, s: &str) {
        let font = &data.fonts[self.font];
        let page = &data.pages[font.tpag.page as usize];
        let (fx, fy) = (font.tpag.src_x as i32, font.tpag.src_y as i32);
        let color = self.color;
        let alpha = self.alpha;
        for (n, line) in split_lines(s).iter().enumerate() {
            let mut cx = match self.halign {
                HAlign::Left => x,
                HAlign::Center => x - (Self::text_width(font, line) / 2) as f64,
            }
            .floor() as i32;
            let cy = y.floor() as i32 + n as i32 * font.line_height;
            for ch in line.chars() {
                let Some(g) = font.glyphs.get(&ch) else { continue };
                if g.w > 0 && g.h > 0 {
                    self.blit(
                        page,
                        fx + g.x as i32,
                        fy + g.y as i32,
                        g.w as i32,
                        g.h as i32,
                        cx + g.offset as i32,
                        cy,
                        color,
                        alpha,
                    );
                }
                cx += g.shift as i32;
            }
        }
    }
}

/// GameMaker 1.x line breaks: `#` is a newline and `\#` a literal hash.
pub fn split_lines(s: &str) -> Vec<String> {
    let mut lines = vec![String::new()];
    let mut chars = s.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '\\' if chars.peek() == Some(&'#') => {
                chars.next();
                lines.last_mut().unwrap().push('#');
            }
            '#' | '\n' => lines.push(String::new()),
            '\r' => {}
            _ => lines.last_mut().unwrap().push(c),
        }
    }
    lines
}
