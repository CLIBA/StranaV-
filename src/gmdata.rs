//! Reader for the GameMaker: Studio 1.x `data.win` archive (bytecode 13) shipped with the game.
//!
//! Only the chunks the port needs are decoded: strings, texture pages, sprites, backgrounds,
//! fonts, objects and rooms. All art and text are taken from the user's copy at runtime.

use std::collections::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub struct DataError(pub String);

impl std::fmt::Display for DataError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for DataError {}

fn err<T>(msg: impl Into<String>) -> Result<T, DataError> {
    Err(DataError(msg.into()))
}

/// A decoded texture page, straight (non-premultiplied) RGBA8.
pub struct Page {
    pub w: u32,
    pub h: u32,
    pub rgba: Vec<u8>,
}

/// Texture page item: where a frame lives on a page and how it maps into the frame box.
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct Tpag {
    pub src_x: u16,
    pub src_y: u16,
    pub src_w: u16,
    pub src_h: u16,
    pub dst_x: u16,
    pub dst_y: u16,
    pub bound_w: u16,
    pub bound_h: u16,
    pub page: u16,
}

#[derive(Serialize, Deserialize)]
pub struct SpriteDef {
    pub name: String,
    pub width: i32,
    pub height: i32,
    pub bbox_left: i32,
    pub bbox_right: i32,
    pub bbox_top: i32,
    pub bbox_bottom: i32,
    pub origin_x: i32,
    pub origin_y: i32,
    pub frames: Vec<Tpag>,
    /// Precise collision mask, `width * height` booleans, row-major.
    #[serde(with = "mask_rows")]
    pub mask: Option<Vec<bool>>,
}

impl SpriteDef {
    /// Is the sprite-local pixel solid for collision purposes?
    pub fn solid_at(&self, lx: i32, ly: i32) -> bool {
        if lx < self.bbox_left || lx > self.bbox_right || ly < self.bbox_top || ly > self.bbox_bottom {
            return false;
        }
        match &self.mask {
            Some(m) => m[(ly * self.width + lx) as usize],
            None => true,
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct BackgroundDef {
    pub name: String,
    pub tpag: Tpag,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Glyph {
    pub x: u16,
    pub y: u16,
    pub w: u16,
    pub h: u16,
    pub shift: i16,
    pub offset: i16,
}

#[derive(Serialize, Deserialize)]
pub struct FontDef {
    pub name: String,
    pub tpag: Tpag,
    pub glyphs: HashMap<char, Glyph>,
    pub line_height: i32,
}

#[derive(Serialize, Deserialize)]
pub struct ObjectDef {
    pub name: String,
    pub sprite: i32,
    pub depth: i32,
}

#[derive(Serialize, Deserialize)]
pub struct RoomBackground {
    pub foreground: bool,
    pub bg: usize,
    pub x: i32,
    pub y: i32,
    pub tile_x: bool,
    pub tile_y: bool,
    pub stretch: bool,
}

#[derive(Serialize, Deserialize)]
pub struct RoomInstance {
    pub x: i32,
    pub y: i32,
    pub obj: usize,
    pub id: u32,
}

#[derive(Serialize, Deserialize)]
pub struct RoomDef {
    pub name: String,
    pub width: i32,
    pub height: i32,
    /// GameMaker colour, 0x00BBGGRR.
    pub color: u32,
    pub draw_color: bool,
    pub backgrounds: Vec<RoomBackground>,
    pub instances: Vec<RoomInstance>,
}

#[derive(Serialize, Deserialize)]
pub struct GameData {
    pub display_name: String,
    pub strings: Vec<String>,
    /// Image files backing `pages`, relative to the assets directory (empty for data.win).
    #[serde(default)]
    pub images: Vec<String>,
    #[serde(skip)]
    pub pages: Vec<Page>,
    pub sprites: Vec<SpriteDef>,
    pub backgrounds: Vec<BackgroundDef>,
    pub fonts: Vec<FontDef>,
    pub objects: Vec<ObjectDef>,
    pub rooms: Vec<RoomDef>,
    pub last_instance_id: u32,
}

struct Reader<'a> {
    d: &'a [u8],
}

impl<'a> Reader<'a> {
    fn u8(&self, o: usize) -> Result<u8, DataError> {
        self.d.get(o).copied().ok_or_else(|| DataError(format!("read past end at {o:#x}")))
    }
    fn u16(&self, o: usize) -> Result<u16, DataError> {
        Ok(u16::from_le_bytes([self.u8(o)?, self.u8(o + 1)?]))
    }
    fn i16(&self, o: usize) -> Result<i16, DataError> {
        Ok(self.u16(o)? as i16)
    }
    fn u32(&self, o: usize) -> Result<u32, DataError> {
        let b = self.d.get(o..o + 4).ok_or_else(|| DataError(format!("read past end at {o:#x}")))?;
        Ok(u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
    }
    fn i32(&self, o: usize) -> Result<i32, DataError> {
        Ok(self.u32(o)? as i32)
    }
    fn ptr(&self, o: usize) -> Result<usize, DataError> {
        Ok(self.u32(o)? as usize)
    }
    /// GameMaker strings are referenced by a pointer to their bytes; the length precedes them.
    fn string_at(&self, p: usize) -> Result<String, DataError> {
        if p < 4 {
            return Ok(String::new());
        }
        let len = self.u32(p - 4)? as usize;
        let b = self.d.get(p..p + len).ok_or_else(|| DataError("bad string".into()))?;
        Ok(String::from_utf8_lossy(b).into_owned())
    }
    fn string(&self, o: usize) -> Result<String, DataError> {
        self.string_at(self.ptr(o)?)
    }
    fn ptr_list(&self, o: usize) -> Result<Vec<usize>, DataError> {
        let n = self.u32(o)? as usize;
        (0..n).map(|i| self.ptr(o + 4 + 4 * i)).collect()
    }
    fn tpag(&self, o: usize) -> Result<Tpag, DataError> {
        Ok(Tpag {
            src_x: self.u16(o)?,
            src_y: self.u16(o + 2)?,
            src_w: self.u16(o + 4)?,
            src_h: self.u16(o + 6)?,
            dst_x: self.u16(o + 8)?,
            dst_y: self.u16(o + 10)?,
            bound_w: self.u16(o + 16)?,
            bound_h: self.u16(o + 18)?,
            page: self.u16(o + 20)?,
        })
    }
}

pub fn decode_png(bytes: &[u8]) -> Result<Page, DataError> {
    let mut dec = png::Decoder::new(std::io::Cursor::new(bytes));
    dec.set_transformations(png::Transformations::EXPAND | png::Transformations::STRIP_16);
    let mut reader = dec.read_info().map_err(|e| DataError(format!("png: {e}")))?;
    let size = reader.output_buffer_size().ok_or_else(|| DataError("png: image too large".into()))?;
    let mut buf = vec![0; size];
    let info = reader.next_frame(&mut buf).map_err(|e| DataError(format!("png: {e}")))?;
    buf.truncate(info.buffer_size());
    let (w, h) = (info.width, info.height);
    let n = (w * h) as usize;
    let rgba = match info.color_type {
        png::ColorType::Rgba => buf,
        png::ColorType::Rgb => buf.chunks_exact(3).flat_map(|p| [p[0], p[1], p[2], 255]).collect(),
        png::ColorType::GrayscaleAlpha => buf.chunks_exact(2).flat_map(|p| [p[0], p[0], p[0], p[1]]).collect(),
        png::ColorType::Grayscale => buf.iter().flat_map(|&g| [g, g, g, 255]).collect(),
        png::ColorType::Indexed => return err("png: unexpanded palette"),
    };
    if rgba.len() != n * 4 {
        return err("png: unexpected buffer size");
    }
    Ok(Page { w, h, rgba })
}

impl GameData {
    pub fn parse(d: &[u8]) -> Result<GameData, DataError> {
        let r = Reader { d };
        if d.get(0..4) != Some(b"FORM") {
            return err("not a GameMaker data.win (no FORM header)");
        }
        let mut chunks: HashMap<&str, (usize, usize)> = HashMap::new();
        let mut p = 8;
        while p + 8 <= d.len() {
            let name = std::str::from_utf8(&d[p..p + 4]).map_err(|_| DataError("bad chunk name".into()))?;
            let size = r.u32(p + 4)? as usize;
            chunks.insert(name, (p + 8, size));
            p += 8 + size;
        }
        let chunk = |n: &str| chunks.get(n).copied().ok_or_else(|| DataError(format!("missing chunk {n}")));

        let (gen8, _) = chunk("GEN8")?;
        let bytecode = r.u8(gen8 + 1)?;
        if bytecode > 16 {
            return err(format!("bytecode version {bytecode} is not GameMaker: Studio 1.x"));
        }
        let last_instance_id = r.u32(gen8 + 12)?;
        let display_name = r.string(gen8 + 100).unwrap_or_default();

        let (strg, _) = chunk("STRG")?;
        let strings = r
            .ptr_list(strg)?
            .into_iter()
            .map(|e| r.string_at(e + 4))
            .collect::<Result<Vec<_>, _>>()?;

        let (txtr, txtr_size) = chunk("TXTR")?;
        let entries = r.ptr_list(txtr)?;
        let offsets = entries.iter().map(|&e| r.ptr(e + 4)).collect::<Result<Vec<_>, _>>()?;
        let mut pages = Vec::new();
        for (i, &start) in offsets.iter().enumerate() {
            let end = offsets.get(i + 1).copied().unwrap_or(txtr + txtr_size);
            let bytes = d.get(start..end).ok_or_else(|| DataError("bad texture offset".into()))?;
            pages.push(decode_png(bytes)?);
        }

        let (sprt, _) = chunk("SPRT")?;
        let mut sprites = Vec::new();
        for e in r.ptr_list(sprt)? {
            let width = r.i32(e + 4)?;
            let height = r.i32(e + 8)?;
            let tex_list = e + 4 * 14;
            let frames = r.ptr_list(tex_list)?.into_iter().map(|t| r.tpag(t)).collect::<Result<Vec<_>, _>>()?;
            let masks_at = tex_list + 4 + 4 * frames.len();
            let mask = if r.u32(masks_at)? > 0 {
                let stride = ((width + 7) / 8) as usize;
                let base = masks_at + 4;
                let mut m = vec![false; (width * height) as usize];
                for y in 0..height as usize {
                    for x in 0..width as usize {
                        let byte = r.u8(base + y * stride + x / 8)?;
                        m[y * width as usize + x] = byte & (0x80 >> (x % 8)) != 0;
                    }
                }
                Some(m)
            } else {
                None
            };
            sprites.push(SpriteDef {
                name: r.string(e)?,
                width,
                height,
                bbox_left: r.i32(e + 12)?,
                bbox_right: r.i32(e + 16)?,
                bbox_bottom: r.i32(e + 20)?,
                bbox_top: r.i32(e + 24)?,
                origin_x: r.i32(e + 48)?,
                origin_y: r.i32(e + 52)?,
                frames,
                mask,
            });
        }

        let (bgnd, _) = chunk("BGND")?;
        let backgrounds = r
            .ptr_list(bgnd)?
            .into_iter()
            .map(|e| Ok(BackgroundDef { name: r.string(e)?, tpag: r.tpag(r.ptr(e + 16)?)? }))
            .collect::<Result<Vec<_>, DataError>>()?;

        let (font, _) = chunk("FONT")?;
        let mut fonts = Vec::new();
        for e in r.ptr_list(font)? {
            let mut glyphs = HashMap::new();
            for g in r.ptr_list(e + 40)? {
                let Some(ch) = char::from_u32(r.u16(g)? as u32) else { continue };
                glyphs.insert(
                    ch,
                    Glyph {
                        x: r.u16(g + 2)?,
                        y: r.u16(g + 4)?,
                        w: r.u16(g + 6)?,
                        h: r.u16(g + 8)?,
                        shift: r.i16(g + 10)?,
                        offset: r.i16(g + 12)?,
                    },
                );
            }
            let line_height = glyphs
                .get(&'M')
                .map(|g| g.h as i32)
                .unwrap_or_else(|| glyphs.values().map(|g| g.h as i32).max().unwrap_or(16));
            fonts.push(FontDef { name: r.string(e)?, tpag: r.tpag(r.ptr(e + 28)?)?, glyphs, line_height });
        }

        let (objt, _) = chunk("OBJT")?;
        let objects = r
            .ptr_list(objt)?
            .into_iter()
            .map(|e| Ok(ObjectDef { name: r.string(e)?, sprite: r.i32(e + 4)?, depth: r.i32(e + 16)? }))
            .collect::<Result<Vec<_>, DataError>>()?;

        let (room, _) = chunk("ROOM")?;
        let mut rooms = Vec::new();
        for e in r.ptr_list(room)? {
            let mut bgs = Vec::new();
            for b in r.ptr_list(r.ptr(e + 40)?)? {
                if r.u32(b)? == 0 {
                    continue;
                }
                bgs.push(RoomBackground {
                    foreground: r.u32(b + 4)? != 0,
                    bg: r.u32(b + 8)? as usize,
                    x: r.i32(b + 12)?,
                    y: r.i32(b + 16)?,
                    tile_x: r.u32(b + 20)? != 0,
                    tile_y: r.u32(b + 24)? != 0,
                    stretch: r.u32(b + 36)? != 0,
                });
            }
            let instances = r
                .ptr_list(r.ptr(e + 48)?)?
                .into_iter()
                .map(|i| {
                    Ok(RoomInstance { x: r.i32(i)?, y: r.i32(i + 4)?, obj: r.u32(i + 8)? as usize, id: r.u32(i + 12)? })
                })
                .collect::<Result<Vec<_>, DataError>>()?;
            rooms.push(RoomDef {
                name: r.string(e)?,
                width: r.i32(e + 8)?,
                height: r.i32(e + 12)?,
                color: r.u32(e + 24)?,
                draw_color: r.u32(e + 28)? != 0,
                backgrounds: bgs,
                instances,
            });
        }

        Ok(GameData { display_name, strings, images: Vec::new(), pages, sprites, backgrounds, fonts, objects, rooms, last_instance_id })
    }

    /// Text from the game's string table.
    pub fn s(&self, idx: u32) -> &str {
        self.strings.get(idx as usize).map(String::as_str).unwrap_or("")
    }
}

/// Collision masks are stored as rows of `#` (solid) and `.` (empty) in game.json.
mod mask_rows {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S: Serializer>(mask: &Option<Vec<bool>>, s: S) -> Result<S::Ok, S::Error> {
        let Some(m) = mask else { return Option::<Vec<String>>::None.serialize(s) };
        // The row width is not known here, so the whole mask is one string split by the reader.
        let flat: String = m.iter().map(|&b| if b { '#' } else { '.' }).collect();
        Some(flat).serialize(s)
    }

    pub fn deserialize<'de, D: Deserializer<'de>>(d: D) -> Result<Option<Vec<bool>>, D::Error> {
        let flat = Option::<String>::deserialize(d)?;
        Ok(flat.map(|f| f.chars().map(|c| c == '#').collect()))
    }
}
