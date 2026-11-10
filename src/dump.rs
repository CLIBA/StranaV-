//! Developer aid: `STRANA_DUMP_ROOMS=dir strana` renders every room headlessly to PNG files,
//! with a well-equipped player, so the port can be compared with the original screen by screen.

use std::path::Path;

use crate::game::*;

fn write_png(path: &Path, px: &[u8]) {
    let file = std::fs::File::create(path).expect("create png");
    let mut enc = png::Encoder::new(std::io::BufWriter::new(file), ROOM_W as u32, ROOM_H as u32);
    enc.set_color(png::ColorType::Rgba);
    enc.set_depth(png::BitDepth::Eight);
    enc.write_header().and_then(|mut w| w.write_image_data(px)).expect("write png");
}

pub fn dump_rooms(mut g: Game, dir: &Path) {
    std::fs::create_dir_all(dir).expect("create dump dir");
    // Let the splash screen play out into the title screen.
    for n in 0..120 {
        g.tick();
        if n == 20 || n == 119 {
            write_png(&dir.join(format!("00_start_{n:03}.png")), &g.screen.px);
        }
    }
    g.save_path = dir.join("save.json");
    let rooms = [
        (R_INTRO, "intro"),
        (R_GAME, "game"),
        (R_RINOK, "rinok"),
        (R_METRO, "metro"),
        (R_GAZETA, "gazeta"),
        (R_TEHNO, "tehno"),
        (R_COMP, "comp"),
        (R_INTERNET, "internet"),
        (R_PASCAL, "pascal"),
        (R_ZAG, "zag"),
        (R_MAP, "map"),
    ];
    for (k, &(room, name)) in rooms.iter().enumerate() {
        if room == R_GAME {
            let gas = g.w.gas.as_mut().expect("gas exists after the title screen");
            for a in gas.allow.iter_mut() {
                *a = 1.0;
            }
            (gas.comp, gas.memo, gas.hard, gas.video, gas.modem, gas.cd, gas.printer, gas.windows) = (3, 3, 4, 3, 7, 4, 2, 2);
            (gas.watt, gas.phone, gas.rabota, gas.inet, gas.kleys, gas.kleys_max, gas.toch, gas.bot, gas.worm) =
                (2.0, 1.0, 1.0, 50.0, 2.0, 9.0, 3.0, 1234.0, 40.0);
            gas.soft_u[8] = 1.0;
            g.w.g.money = 1234.0;
            g.w.g.golod = 42.0;
            g.w.g.work = 6;
        }
        g.modal.clear();
        g.enter_room(room);
        let frames = if room == R_ZAG { 260 } else { 40 };
        for n in 0..frames {
            g.modal.clear();
            g.tick();
            if room == R_ZAG && n % 65 == 64 {
                write_png(&dir.join(format!("{:02}_{name}_{n}.png", k + 1)), &g.screen.px);
            }
        }
        if room == R_INTERNET {
            // Visit the download site too.
            g.baction(55);
            for _ in 0..5 {
                g.tick();
            }
            g.baction(60);
            for _ in 0..30 {
                g.tick();
            }
            write_png(&dir.join(format!("{:02}_{name}_tracker.png", k + 1)), &g.screen.px);
        }
        if room == R_PASCAL {
            g.baction(82);
            for _ in 0..200 {
                g.modal.clear();
                g.tick();
            }
        }
        if room == R_RINOK {
            g.baction(4);
            g.tick();
        }
        write_png(&dir.join(format!("{:02}_{name}.png", k + 1)), &g.screen.px);
    }
    // A message box over the main screen.
    g.w.parts = None;
    g.enter_room(R_GAME);
    g.tick();
    g.baction(9);
    g.tick();
    write_png(&dir.join("20_message.png"), &g.screen.px);
}
