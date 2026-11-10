//! Converts the player's copy of the original game into the port's own asset folder, and loads it.
//!
//! `strana extract StranaV.exe` unpacks the cabinet inside the installer, parses `data.win`
//! and writes plain files: one PNG per sprite frame, background and font, plus `game.json`
//! with strings, glyph metrics, collision masks, objects and rooms. The game itself only
//! ever reads that folder.

use std::io::{Cursor, Read};
use std::path::{Path, PathBuf};

use crate::gmdata::{GameData, Page, Tpag};

pub const MANIFEST: &str = "game.json";

type Error = Box<dyn std::error::Error>;

/// Returns the bytes of `data.win`, taken either from the file itself or from the installer.
pub fn read_data_win(path: &Path) -> Result<Vec<u8>, Error> {
    let bytes = std::fs::read(path).map_err(|e| format!("{}: {e}", path.display()))?;
    if bytes.starts_with(b"FORM") {
        return Ok(bytes);
    }
    if !bytes.starts_with(b"MZ") {
        return Err(format!("{}: neither data.win nor StranaV.exe", path.display()).into());
    }
    // IExpress installer: a Microsoft cabinet stored inside the executable.
    let mut at = 0;
    while let Some(off) = find(&bytes[at..], b"MSCF").map(|o| o + at) {
        at = off + 1;
        let h = &bytes[off..];
        // A real CFHEADER has reserved fields of zero and version 1.3.
        if h.len() < 36 || h[4..8] != [0; 4] || h[24] != 3 || h[25] != 1 {
            continue;
        }
        let mut cab = cab::Cabinet::new(Cursor::new(&bytes[off..]))?;
        let mut out = Vec::new();
        cab.read_file("data.win")?.read_to_end(&mut out)?;
        return Ok(out);
    }
    Err(format!("{}: no cabinet with data.win inside", path.display()).into())
}

fn find(hay: &[u8], needle: &[u8]) -> Option<usize> {
    hay.windows(needle.len()).position(|w| w == needle)
}

fn crop(page: &Page, t: &Tpag, w: u32, h: u32) -> Page {
    let mut rgba = vec![0; (w * h * 4) as usize];
    for y in 0..t.src_h as u32 {
        for x in 0..t.src_w as u32 {
            let (dx, dy) = (t.dst_x as u32 + x, t.dst_y as u32 + y);
            if dx >= w || dy >= h {
                continue;
            }
            let s = (((t.src_y as u32 + y) * page.w + t.src_x as u32 + x) * 4) as usize;
            let d = ((dy * w + dx) * 4) as usize;
            rgba[d..d + 4].copy_from_slice(&page.rgba[s..s + 4]);
        }
    }
    Page { w, h, rgba }
}

fn write_png(path: &Path, p: &Page) -> Result<(), Error> {
    if let Some(dir) = path.parent() {
        std::fs::create_dir_all(dir)?;
    }
    let file = std::fs::File::create(path)?;
    let mut enc = png::Encoder::new(std::io::BufWriter::new(file), p.w, p.h);
    enc.set_color(png::ColorType::Rgba);
    enc.set_depth(png::BitDepth::Eight);
    enc.write_header()?.write_image_data(&p.rgba)?;
    Ok(())
}


/// Splits the texture atlases into one image per resource and writes the asset folder.
pub fn extract(src: &Path, dst: &Path) -> Result<(), Error> {
    let mut data = GameData::parse(&read_data_win(src)?)?;
    crate::game::Game::validate(&data)?;
    let atlases = std::mem::take(&mut data.pages);
    let mut images = Vec::new();
    let mut pages = Vec::new();
    let mut add = |file: String, img: Page, t: &mut Tpag| -> Result<(), Error> {
        write_png(&dst.join(&file), &img)?;
        *t = Tpag {
            src_x: 0,
            src_y: 0,
            src_w: img.w as u16,
            src_h: img.h as u16,
            dst_x: 0,
            dst_y: 0,
            bound_w: img.w as u16,
            bound_h: img.h as u16,
            page: pages.len() as u16,
        };
        images.push(file);
        pages.push(img);
        Ok(())
    };
    for s in &mut data.sprites {
        let (w, h) = (s.width as u32, s.height as u32);
        for (i, t) in s.frames.iter_mut().enumerate() {
            let img = crop(&atlases[t.page as usize], t, w, h);
            add(format!("sprites/{}_{i}.png", s.name), img, t)?;
        }
    }
    for b in &mut data.backgrounds {
        let t = &mut b.tpag;
        let img = crop(&atlases[t.page as usize], t, t.bound_w as u32, t.bound_h as u32);
        add(format!("backgrounds/{}.png", b.name), img, t)?;
    }
    for f in &mut data.fonts {
        let t = &mut f.tpag;
        let img = crop(&atlases[t.page as usize], t, t.bound_w as u32, t.bound_h as u32);
        add(format!("fonts/{}.png", f.name), img, t)?;
    }
    data.images = images;
    std::fs::create_dir_all(dst)?;
    std::fs::write(dst.join(MANIFEST), serde_json::to_vec_pretty(&data)?)?;
    Ok(())
}

mod embedded {
    include!(concat!(env!("OUT_DIR"), "/embedded_assets.rs"));
}

/// Were assets compiled into this executable?
pub fn has_embedded() -> bool {
    !embedded::FILES.is_empty()
}

fn load_with(read: impl Fn(&str) -> Result<Vec<u8>, Error>) -> Result<GameData, Error> {
    let mut data: GameData = serde_json::from_slice(&read(MANIFEST)?)?;
    data.pages = data
        .images
        .iter()
        .map(|f| Ok(crate::gmdata::decode_png(&read(f)?)?))
        .collect::<Result<_, Error>>()?;
    Ok(data)
}

pub fn load(dir: &Path) -> Result<GameData, Error> {
    load_with(|name| {
        let path = dir.join(name);
        std::fs::read(&path).map_err(|e| format!("{}: {e}", path.display()).into())
    })
}

pub fn load_embedded() -> Result<GameData, Error> {
    load_with(|name| {
        embedded::FILES
            .iter()
            .find(|(n, _)| *n == name)
            .map(|(_, bytes)| bytes.to_vec())
            .ok_or_else(|| format!("embedded asset {name} is missing").into())
    })
}

/// Where the original game might be lying around, for the first-run extraction.
pub fn original_candidates() -> Vec<PathBuf> {
    let mut v = vec![
        PathBuf::from("StranaV.exe"),
        PathBuf::from("data.win"),
        PathBuf::from("out/cab/data.win"),
    ];
    if let Some(dir) = std::env::current_exe().ok().as_deref().and_then(Path::parent) {
        v.push(dir.join("StranaV.exe"));
        v.push(dir.join("data.win"));
    }
    v
}
