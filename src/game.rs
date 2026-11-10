//! A small re-implementation of the GameMaker: Studio 1.x runner loop for this game:
//! instances, alarms, mouse events, collisions, rooms, draw order and modal dialogs.

use std::collections::VecDeque;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::canvas::{Canvas, HAlign, C_BLACK, C_WHITE};
use crate::gmdata::GameData;
use crate::particles::{ParticleTextures, PartSystem};

pub const ROOM_W: i32 = 800;
pub const ROOM_H: i32 = 600;

// Asset indices; `Game::new` verifies them against the names in data.win.
pub const S_B_COMP: i32 = 1;
pub const S_BUTTON2: i32 = 2;
pub const S_BUTTON3: i32 = 3;
const SPRITE_NAMES: &[&str] =
    &["s_button", "s_b_comp", "s_button2", "s_button3", "sprite4", "s_inet", "sprite7", "s_cpp2", "s_cpp", "s_sym", "s_pcc", "s_dot"];

pub const FNT_00: usize = 1;
pub const FNT_01: usize = 2;
pub const FNT_PROG: usize = 3;
const FONT_NAMES: &[&str] = &["fnt_cap", "fnt_00", "fnt_01", "fnt_prog"];

pub const R_GAMINATOR: usize = 0;
pub const R_INTRO: usize = 1;
pub const R_GAME: usize = 2;
pub const R_RINOK: usize = 3;
pub const R_METRO: usize = 4;
pub const R_GAZETA: usize = 5;
pub const R_TEHNO: usize = 6;
pub const R_COMP: usize = 7;
pub const R_INTERNET: usize = 8;
pub const R_PASCAL: usize = 9;
pub const R_MAP: usize = 10;
pub const R_ZAG: usize = 11;
const ROOM_NAMES: &[&str] = &[
    "r_gaminator", "r_intro", "r_game", "r_rinok", "r_metro", "r_gazeta", "r_tehno", "r_comp", "r_internet", "r_pascal",
    "r_map", "r_zag",
];

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum Obj {
    #[default]
    Gas,
    OButton,
    Listovki,
    Razgruzka,
    Downloader,
    Oinet,
    Odisks,
    Oprogram,
    Programgame,
    Sym,
    Particlez,
    Partfader,
    Partunfader,
    Pcc,
    Uprav,
    Dot,
    Dot2,
    Gaminator,
    Unfader2,
}

const OBJECTS: &[(Obj, &str)] = &[
    (Obj::Gas, "gas"),
    (Obj::OButton, "o_button"),
    (Obj::Listovki, "listovki"),
    (Obj::Razgruzka, "razgruzka"),
    (Obj::Downloader, "downloader"),
    (Obj::Oinet, "oinet"),
    (Obj::Odisks, "odisks"),
    (Obj::Oprogram, "oprogram"),
    (Obj::Programgame, "programgame"),
    (Obj::Sym, "sym"),
    (Obj::Particlez, "particlez"),
    (Obj::Partfader, "partfader"),
    (Obj::Partunfader, "partunfader"),
    (Obj::Pcc, "pcc"),
    (Obj::Uprav, "uprav"),
    (Obj::Dot, "dot"),
    (Obj::Dot2, "dot2"),
    (Obj::Gaminator, "gaminator"),
    (Obj::Unfader2, "unfader2"),
];

impl Obj {
    pub fn index(self) -> usize {
        OBJECTS.iter().position(|(o, _)| *o == self).unwrap()
    }
}

/// One GameMaker instance. GML variables are dynamic; the union of everything the objects use
/// lives here, named as in the original code.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Inst {
    pub id: u32,
    pub obj: Obj,
    pub x: f64,
    pub y: f64,
    pub depth: i32,
    pub sprite: i32,
    pub image_index: f64,
    pub image_blend: u32,
    pub speed: f64,
    pub direction: f64,
    pub alarm: [i32; 12],
    pub dead: bool,
    pub mouse_over: bool,

    pub d: i32,
    pub w2: f64,
    pub sme: f64,
    pub go: f64,
    pub progress: f64,
    pub police: f64,
    pub find: f64,
    pub mb: f64,
    pub site: i32,
    pub shift: f64,
    pub name: String,
    pub soft: i32,
    pub dbl: f64,
    pub a: f64,
    pub m: f64,
    pub o: u32,
    pub drop: f64,
    pub en: f64,
    pub za: f64,
    pub es: f64,
    pub fin: f64,
    pub dfin: f64,
    pub c: String,
    pub met: f64,
    pub v: bool,
    pub typ: i32,
    pub bug: i32,
    pub particle1: usize,
    pub particle2: usize,
}

/// The persistent `gas` controller: the player's whole state lives in it.
#[derive(Clone, Serialize, Deserialize)]
pub struct Gas {
    pub alarm0: i32,
    /// `_button[d]` captions as string-table ids (a few change during play).
    pub button: Vec<Option<u32>>,
    pub home: i32,
    pub watt: f64,
    pub phone: f64,
    pub rabota: f64,
    pub propiska: f64,
    pub diplom: f64,
    pub day: f64,
    pub month: f64,
    pub year: f64,
    pub time: f64,
    pub metro_ch: f64,
    pub metro_nah: f64,
    pub tokens: f64,
    pub police_act: f64,
    pub list_first: f64,
    pub pva: f64,
    pub glass: f64,
    pub rzar: f64,
    pub rpot: f64,
    pub allow: [f64; 10],
    pub inet: f64,
    pub download: f64,
    pub upload: f64,
    pub soft: f64,
    pub worm: f64,
    pub hello: f64,
    pub programs: f64,
    pub topsecret: f64,
    pub bot: f64,
    pub avt: f64,
    pub uiz: f64,
    pub poli: f64,
    pub bot_next: f64,
    pub svoy: f64,
    pub soft_u: Vec<f64>,
    pub comp: i32,
    pub memo: i32,
    pub hard: i32,
    pub video: i32,
    pub modem: i32,
    pub cd: i32,
    pub printer: i32,
    pub windows: i32,
    pub kleys: f64,
    pub kleys_max: f64,
    pub cdsbor: f64,
    pub cdmb: f64,
    pub toch: f64,
}

/// `global.*` variables.
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Globals {
    pub money: f64,
    pub iq: f64,
    pub golod: f64,
    pub work: i32,
    pub income: f64,
}

/// Everything `game_save` writes.
#[derive(Clone, Serialize, Deserialize)]
pub struct World {
    pub room: usize,
    pub insts: Vec<Inst>,
    pub next_id: u32,
    pub gas: Option<Gas>,
    pub g: Globals,
    pub parts: Option<PartSystem>,
    /// `with (gas) instance_destroy()` takes effect on the following room change.
    #[serde(default)]
    pub gas_destroyed: bool,
}

#[derive(Clone, Copy)]
pub enum Cont {
    /// "Start a new game?" from the title screen.
    NewGame,
    /// `programgame` debug loop paused on a mark; resumes at `resume`.
    Debug { inst: u32, resume: i32 },
}

pub enum AfterMessage {
    PoliceFine(f64),
    PosterHired(u32),
    LeafletsEarned { inst: u32, prize: f64 },
}

pub struct Modal {
    pub text: String,
    pub question: Option<Cont>,
    pub after: Option<AfterMessage>,
}

#[derive(Default, Clone, Copy)]
pub struct Input {
    pub mx: f64,
    pub my: f64,
    pub pressed: bool,
    pub released: bool,
    pub confirm: bool,
    pub cancel: bool,
}

pub struct Game {
    pub data: GameData,
    pub w: World,
    pub input: Input,
    /// The room as drawn by the last completed step.
    pub scene: Canvas,
    /// What is shown: `scene` plus any dialog on top.
    pub screen: Canvas,
    pub frame: u64,
    pub modal: VecDeque<Modal>,
    pub modal_pressed: Option<usize>,
    pending_room: Option<usize>,
    pending_load: bool,
    pub quit: bool,
    pub save_path: PathBuf,
    pub ptex: ParticleTextures,
}

impl Game {
    /// Checks that the data is the build of the game this port was written against.
    pub fn validate(data: &GameData) -> Result<(), String> {
        check_names("sprite", data.sprites.iter().map(|s| s.name.as_str()), SPRITE_NAMES)?;
        check_names("font", data.fonts.iter().map(|f| f.name.as_str()), FONT_NAMES)?;
        check_names("room", data.rooms.iter().map(|r| r.name.as_str()), ROOM_NAMES)?;
        check_names("object", data.objects.iter().map(|o| o.name.as_str()), &OBJECTS.iter().map(|o| o.1).collect::<Vec<_>>())?;
        for &(i, s) in crate::txt::PROBES {
            if data.s(i) != s {
                return Err(format!("string table mismatch at #{i}: this data.win is a different build of the game"));
            }
        }
        Ok(())
    }

    pub fn new(data: GameData, save_path: PathBuf) -> Result<Game, String> {
        Self::validate(&data)?;
        let mut g = Game {
            w: World { room: R_GAMINATOR, insts: Vec::new(), next_id: data.last_instance_id + 1, gas: None, g: Globals::default(), parts: None, gas_destroyed: false },
            data,
            input: Input::default(),
            scene: Canvas::new(ROOM_W, ROOM_H),
            screen: Canvas::new(ROOM_W, ROOM_H),
            frame: 0,
            modal: VecDeque::new(),
            modal_pressed: None,
            pending_room: None,
            pending_load: false,
            quit: false,
            save_path,
            ptex: ParticleTextures::new(),
        };
        g.enter_room(R_GAMINATOR);
        g.draw();
        g.screen = g.scene.clone();
        Ok(g)
    }

    pub fn s(&self, idx: u32) -> String {
        self.data.s(idx).to_string()
    }

    // ---- GML built-ins ------------------------------------------------------------------

    pub fn show_message(&mut self, text: String) {
        self.modal.push_back(Modal { text, question: None, after: None });
    }

    pub(crate) fn show_message_then(&mut self, text: String, after: AfterMessage) {
        self.modal.push_back(Modal { text, question: None, after: Some(after) });
    }

    pub fn msg(&mut self, idx: u32) {
        let t = self.s(idx);
        self.show_message(t);
    }

    pub fn show_question(&mut self, text: String, cont: Cont) {
        self.modal.push_back(Modal { text, question: Some(cont), after: None });
    }

    pub fn goto_room(&mut self, room: usize) {
        self.pending_room = Some(room);
    }

    pub fn game_end(&mut self) {
        self.quit = true;
    }

    pub fn game_save(&self) {
        let save = || -> Result<(), Box<dyn std::error::Error>> {
            if let Some(dir) = self.save_path.parent() {
                std::fs::create_dir_all(dir)?;
            }
            std::fs::write(&self.save_path, serde_json::to_vec(&self.w)?)?;
            Ok(())
        };
        if let Err(e) = save() {
            eprintln!("game_save: {e}");
        }
    }

    pub fn save_exists(&self) -> bool {
        self.save_path.is_file()
    }

    pub fn game_load(&mut self) {
        self.pending_load = true;
    }

    pub fn io_clear(&mut self) {
        self.input.pressed = false;
        self.input.released = false;
    }

    pub fn gas(&self) -> &Gas {
        self.w.gas.as_ref().expect("gas instance")
    }

    pub fn gas_mut(&mut self) -> &mut Gas {
        self.w.gas.as_mut().expect("gas instance")
    }

    /// Index of the first live instance of `obj` (what `obj.var` refers to in GML).
    pub fn first(&self, obj: Obj) -> Option<usize> {
        self.w.insts.iter().position(|i| i.obj == obj && !i.dead)
    }

    pub fn find_id(&self, id: u32) -> Option<usize> {
        self.w.insts.iter().position(|i| i.id == id && !i.dead)
    }

    pub fn all(&self, obj: Obj) -> Vec<usize> {
        (0..self.w.insts.len()).filter(|&i| self.w.insts[i].obj == obj && !self.w.insts[i].dead).collect()
    }

    pub fn instance_create(&mut self, x: f64, y: f64, obj: Obj) -> usize {
        let id = self.w.next_id;
        self.w.next_id += 1;
        self.spawn(x, y, obj, id)
    }

    fn spawn(&mut self, x: f64, y: f64, obj: Obj, id: u32) -> usize {
        let def = &self.data.objects[obj.index()];
        self.w.insts.push(Inst {
            id,
            obj,
            x,
            y,
            depth: def.depth,
            sprite: def.sprite,
            image_blend: C_WHITE,
            alarm: [-1; 12],
            ..Default::default()
        });
        let i = self.w.insts.len() - 1;
        self.ev_create(i);
        i
    }

    // ---- Rooms ----------------------------------------------------------------------------

    pub(crate) fn enter_room(&mut self, room: usize) {
        // Only `gas` is persistent, and it lives outside the instance list.
        self.w.insts.clear();
        self.w.room = room;
        if std::mem::take(&mut self.w.gas_destroyed) {
            self.w.gas = None;
        }
        let placed: Vec<(i32, i32, usize, u32)> =
            self.data.rooms[room].instances.iter().map(|i| (i.x, i.y, i.obj, i.id)).collect();
        for (x, y, obj, id) in placed {
            let obj = OBJECTS[obj].0;
            if obj == Obj::Gas {
                self.w.gas = Some(Gas::create());
                self.w.g = Globals::initial();
                continue;
            }
            let i = self.spawn(x as f64, y as f64, obj, id);
            self.creation_code(i);
        }
    }

    fn apply_pending(&mut self) {
        if std::mem::take(&mut self.pending_load) {
            match std::fs::read(&self.save_path).map_err(|e| e.to_string()).and_then(|b| {
                serde_json::from_slice::<World>(&b).map_err(|e| e.to_string())
            }) {
                Ok(w) => {
                    self.w = w;
                    self.pending_room = None;
                }
                Err(e) => eprintln!("game_load: {e}"),
            }
        }
        if let Some(r) = self.pending_room.take() {
            self.enter_room(r);
        }
    }

    // ---- Main loop ------------------------------------------------------------------------

    /// One game step at room_speed (30 per second).
    pub fn tick(&mut self) {
        if !self.modal.is_empty() {
            self.tick_modal();
            self.input = Input { mx: self.input.mx, my: self.input.my, ..Default::default() };
            return;
        }
        self.frame += 1;

        // Alarms, alarm-number major like the runner.
        for a in 0..12 {
            if a == 0 {
                if let Some(gas) = self.w.gas.as_mut() {
                    if gas.alarm0 > 0 {
                        gas.alarm0 -= 1;
                        if gas.alarm0 == 0 {
                            gas.alarm0 = -1;
                            self.gas_alarm0();
                        }
                    }
                }
            }
            let mut i = 0;
            while i < self.w.insts.len() {
                let inst = &mut self.w.insts[i];
                if !inst.dead && inst.alarm[a] > 0 {
                    inst.alarm[a] -= 1;
                    if inst.alarm[a] == 0 {
                        inst.alarm[a] = -1;
                        self.ev_alarm(i, a);
                    }
                }
                i += 1;
            }
        }

        self.mouse_events();

        let mut i = 0;
        while i < self.w.insts.len() {
            if !self.w.insts[i].dead {
                self.ev_step(i);
            }
            i += 1;
        }

        for inst in self.w.insts.iter_mut().filter(|i| !i.dead && i.speed != 0.0) {
            let r = inst.direction.to_radians();
            inst.x += inst.speed * r.cos();
            inst.y -= inst.speed * r.sin();
        }

        self.collisions();

        // End step.
        if let Some(gas) = self.w.gas.as_mut() {
            if gas.alarm0 == -1 {
                gas.alarm0 = 30;
            }
        }

        if let Some(p) = self.w.parts.as_mut() {
            p.update();
        }

        // A dialog raised during this step freezes the picture of the previous step,
        // exactly like the blocking message box did.
        if self.modal.is_empty() {
            self.draw();
            self.screen.px.copy_from_slice(&self.scene.px);
        }

        self.w.insts.retain(|i| !i.dead);
        if self.modal.is_empty() {
            self.apply_pending();
        } else {
            self.render_modal();
        }
        self.input = Input { mx: self.input.mx, my: self.input.my, ..Default::default() };
    }

    fn mouse_over(&self, i: usize) -> bool {
        let inst = &self.w.insts[i];
        if inst.sprite < 0 {
            return false;
        }
        let spr = &self.data.sprites[inst.sprite as usize];
        let lx = (self.input.mx - (inst.x - spr.origin_x as f64)).floor() as i32;
        let ly = (self.input.my - (inst.y - spr.origin_y as f64)).floor() as i32;
        spr.solid_at(lx, ly)
    }

    fn mouse_events(&mut self) {
        let n = self.w.insts.len();
        let over: Vec<bool> = (0..n).map(|i| self.mouse_over(i)).collect();
        // Left pressed (4).
        for i in 0..n {
            if !self.w.insts[i].dead && self.input.pressed && over[i] {
                self.ev_mouse_pressed(i);
            }
        }
        // Left released (7).
        for i in 0..n {
            if !self.w.insts[i].dead && self.input.released && over[i] && self.w.insts[i].obj == Obj::OButton {
                let d = self.w.insts[i].d;
                self.baction(d);
                self.w.insts[i].image_index = 0.0;
            }
        }
        // Mouse leave (11).
        for i in 0..n {
            let inst = &mut self.w.insts[i];
            if !inst.dead && inst.obj == Obj::OButton && inst.mouse_over && !over[i] {
                inst.image_index = 0.0;
            }
            inst.mouse_over = over[i];
        }
        // Global left pressed (53).
        for i in 0..n {
            if !self.w.insts[i].dead && self.input.pressed {
                match self.w.insts[i].obj {
                    Obj::Gaminator => self.goto_room(R_INTRO),
                    Obj::Uprav => self.uprav_fire(i),
                    _ => {}
                }
            }
        }
        // Global left released (56).
        for i in 0..n {
            let inst = &self.w.insts[i];
            if !inst.dead && self.input.released && matches!(inst.obj, Obj::Oinet | Obj::Odisks | Obj::Oprogram) {
                if crate::gml::point_distance(inst.x, inst.y, self.input.mx, self.input.my) > 100.0 {
                    self.w.insts[i].image_index = 0.0;
                }
            }
        }
    }

    fn bbox(&self, i: usize) -> Option<(i32, i32, i32, i32, i32, i32)> {
        let inst = &self.w.insts[i];
        let spr = self.data.sprites.get(usize::try_from(inst.sprite).ok()?)?;
        let ox = (inst.x - spr.origin_x as f64).round() as i32;
        let oy = (inst.y - spr.origin_y as f64).round() as i32;
        Some((ox + spr.bbox_left, oy + spr.bbox_top, ox + spr.bbox_right, oy + spr.bbox_bottom, ox, oy))
    }

    /// Precise (mask) collision test between two instances.
    fn collide(&self, a: usize, b: usize) -> bool {
        let (Some(ba), Some(bb)) = (self.bbox(a), self.bbox(b)) else { return false };
        let (l, t) = (ba.0.max(bb.0), ba.1.max(bb.1));
        let (r, btm) = (ba.2.min(bb.2), ba.3.min(bb.3));
        if l > r || t > btm {
            return false;
        }
        let sa = &self.data.sprites[self.w.insts[a].sprite as usize];
        let sb = &self.data.sprites[self.w.insts[b].sprite as usize];
        (t..=btm).any(|y| (l..=r).any(|x| sa.solid_at(x - ba.4, y - ba.5) && sb.solid_at(x - bb.4, y - bb.5)))
    }

    fn collisions(&mut self) {
        let pccs = self.all(Obj::Pcc);
        let n = self.w.insts.len();
        for i in 0..n {
            let obj = self.w.insts[i].obj;
            if !matches!(obj, Obj::Dot | Obj::Dot2) {
                continue;
            }
            for &p in &pccs {
                if self.w.insts[i].dead {
                    break;
                }
                if !self.w.insts[p].dead && self.collide(i, p) {
                    self.ev_collision_pcc(i, p);
                }
            }
        }
    }

    // ---- Drawing --------------------------------------------------------------------------

    pub fn draw(&mut self) {
        let mut cv = std::mem::replace(&mut self.scene, Canvas::new(0, 0));
        let room = &self.data.rooms[self.w.room];
        if room.draw_color {
            cv.clear(room.color);
        }
        self.draw_backgrounds(&mut cv, false);

        // Depth-sorted draw list; `gas` first among equals, the particle system at depth 0.
        enum D {
            Gas,
            Inst(usize),
            Parts,
        }
        let mut list: Vec<(i32, D)> = Vec::new();
        if self.w.gas.is_some() {
            list.push((0, D::Gas));
        }
        for (i, inst) in self.w.insts.iter().enumerate() {
            if !inst.dead {
                list.push((inst.depth, D::Inst(i)));
            }
        }
        if self.w.parts.is_some() {
            list.push((0, D::Parts));
        }
        list.sort_by(|a, b| b.0.cmp(&a.0));
        for (_, d) in list {
            match d {
                D::Gas => self.gas_draw(&mut cv),
                D::Inst(i) => {
                    if !self.w.insts[i].dead {
                        self.ev_draw(i, &mut cv)
                    }
                }
                D::Parts => {
                    if let Some(p) = &self.w.parts {
                        p.draw(&mut cv, &self.ptex);
                    }
                }
            }
        }
        self.draw_backgrounds(&mut cv, true);
        self.scene = cv;
    }

    fn draw_backgrounds(&self, cv: &mut Canvas, foreground: bool) {
        let room = &self.data.rooms[self.w.room];
        for b in room.backgrounds.iter().filter(|b| b.foreground == foreground) {
            let t = self.data.backgrounds[b.bg].tpag;
            let page = &self.data.pages[t.page as usize];
            if b.stretch {
                cv.blit_stretched(page, &t, 0, 0, room.width, room.height);
                continue;
            }
            let (w, h) = (t.src_w as i32, t.src_h as i32);
            let xs: Vec<i32> = if b.tile_x { (-1..=room.width / w + 1).map(|k| b.x.rem_euclid(w) + k * w).collect() } else { vec![b.x] };
            let ys: Vec<i32> = if b.tile_y { (-1..=room.height / h + 1).map(|k| b.y.rem_euclid(h) + k * h).collect() } else { vec![b.y] };
            for &y in &ys {
                for &x in &xs {
                    cv.blit(page, t.src_x as i32, t.src_y as i32, w, h, x + t.dst_x as i32, y + t.dst_y as i32, C_WHITE, 1.0);
                }
            }
        }
    }

    pub fn draw_sprite(&self, cv: &mut Canvas, sprite: i32, frame: f64, x: f64, y: f64, blend: u32, alpha: f64) {
        let Some(spr) = usize::try_from(sprite).ok().and_then(|s| self.data.sprites.get(s)) else { return };
        if spr.frames.is_empty() {
            return;
        }
        let f = (frame.floor() as i64).rem_euclid(spr.frames.len() as i64) as usize;
        cv.draw_tpag(&self.data, &spr.frames[f], x - spr.origin_x as f64, y - spr.origin_y as f64, blend, alpha);
    }

    // ---- Message boxes ----------------------------------------------------------------------

    fn modal_layout(&self, m: &Modal) -> ModalLayout {
        let font = &self.data.fonts[FNT_00];
        let mut lines = Vec::new();
        for para in crate::canvas::split_lines(&m.text) {
            lines.extend(wrap(font, &para, 440));
        }
        let text_w = lines.iter().map(|l| Canvas::text_width(font, l)).max().unwrap_or(0);
        let nbtn = if m.question.is_some() { 2 } else { 1 };
        let w = (text_w + 48).max(nbtn * 90 + 40).max(220);
        let h = 30 + 18 + lines.len() as i32 * font.line_height + 52;
        let x = (ROOM_W - w) / 2;
        let y = (ROOM_H - h) / 2;
        let by = y + h - 38;
        let total = nbtn * 80 + (nbtn - 1) * 10;
        let bx0 = x + (w - total) / 2;
        let buttons = (0..nbtn).map(|k| (bx0 + k * 90, by, 80, 26)).collect();
        ModalLayout { x, y, w, h, lines, buttons }
    }

    fn tick_modal(&mut self) {
        let Some(m) = self.modal.front() else { return };
        let lay = self.modal_layout(m);
        let hit = |inp: &Input| {
            lay.buttons.iter().position(|&(bx, by, bw, bh)| {
                inp.mx >= bx as f64 && inp.mx < (bx + bw) as f64 && inp.my >= by as f64 && inp.my < (by + bh) as f64
            })
        };
        let mut answer = None;
        if self.input.pressed {
            self.modal_pressed = hit(&self.input);
        }
        if self.input.released {
            if let Some(p) = self.modal_pressed.take() {
                if hit(&self.input) == Some(p) {
                    answer = Some(p == 0);
                }
            }
        }
        if self.input.confirm {
            answer = Some(true);
        } else if self.input.cancel {
            answer = Some(m.question.is_none());
        }
        if let Some(yes) = answer {
            let m = self.modal.pop_front().unwrap();
            self.modal_pressed = None;
            if let Some(cont) = m.question {
                self.resume(cont, yes);
            }
            if let Some(after) = m.after {
                match after {
                    AfterMessage::PoliceFine(lose) => self.w.g.money -= lose,
                    AfterMessage::PosterHired(inst) => {
                        self.gas_mut().police_act += 10.0;
                        self.upd_income();
                        if let Some(i) = self.find_id(inst) {
                            self.w.insts[i].police = 0.0;
                        }
                    }
                    AfterMessage::LeafletsEarned { inst, prize } => {
                        self.gas_mut().police_act += (prize / 4.0).floor() + 1.0;
                        if let Some(i) = self.find_id(inst) {
                            self.w.insts[i].police = 0.0;
                        }
                    }
                }
            }
        }
        if self.modal.is_empty() {
            self.apply_pending();
            self.screen.px.copy_from_slice(&self.scene.px);
        } else {
            self.render_modal();
        }
    }

    fn resume(&mut self, cont: Cont, yes: bool) {
        match cont {
            Cont::NewGame => {
                if yes {
                    self.goto_room(R_GAME);
                }
            }
            Cont::Debug { inst, resume } => {
                if yes {
                    if let Some(i) = self.find_id(inst) {
                        self.programgame_debug(i, resume);
                    }
                }
            }
        }
    }

    /// Classic Windows message box over the frozen frame.
    fn render_modal(&mut self) {
        self.screen.px.copy_from_slice(&self.scene.px);
        let Some(m) = self.modal.front() else { return };
        let lay = self.modal_layout(m);
        let cv = &mut self.screen;
        let (x, y, w, h) = (lay.x as f64, lay.y as f64, lay.w as f64, lay.h as f64);
        let saved = (cv.color, cv.alpha, cv.additive, cv.font, cv.halign);
        cv.alpha = 1.0;
        cv.additive = false;
        bevel(cv, x, y, w, h, false);
        cv.color = 0x800000; // title bar, navy
        cv.rect(x + 3.0, y + 3.0, x + w - 4.0, y + 22.0, false);
        cv.font = FNT_00;
        cv.halign = HAlign::Left;
        cv.color = C_WHITE;
        cv.text(&self.data, x + 8.0, y + 4.0, &self.data.display_name);
        cv.color = C_BLACK;
        for (k, line) in lay.lines.iter().enumerate() {
            cv.text(&self.data, x + 24.0, y + 40.0 + (k as i32 * self.data.fonts[FNT_00].line_height) as f64, line);
        }
        let labels: &[&str] = if m.question.is_some() { &["Да", "Нет"] } else { &["OK"] };
        for (k, &(bx, by, bw, bh)) in lay.buttons.iter().enumerate() {
            let down = self.modal_pressed == Some(k);
            bevel(cv, bx as f64, by as f64, bw as f64, bh as f64, down);
            cv.color = C_BLACK;
            cv.halign = HAlign::Center;
            let off = if down { 1.0 } else { 0.0 };
            cv.text(&self.data, (bx + bw / 2) as f64 + off, by as f64 + 4.0 + off, labels[k]);
        }
        (cv.color, cv.alpha, cv.additive, cv.font, cv.halign) = saved;
    }
}

struct ModalLayout {
    x: i32,
    y: i32,
    w: i32,
    h: i32,
    lines: Vec<String>,
    buttons: Vec<(i32, i32, i32, i32)>,
}

fn bevel(cv: &mut Canvas, x: f64, y: f64, w: f64, h: f64, sunken: bool) {
    let (hi, lo) = if sunken { (0x808080, C_WHITE) } else { (C_WHITE, 0x808080) };
    cv.color = 0xc0c0c0;
    cv.rect(x, y, x + w - 1.0, y + h - 1.0, false);
    cv.color = hi;
    cv.line(x, y, x + w - 1.0, y);
    cv.line(x, y, x, y + h - 1.0);
    cv.color = lo;
    cv.line(x + 1.0, y + h - 2.0, x + w - 2.0, y + h - 2.0);
    cv.line(x + w - 2.0, y + 1.0, x + w - 2.0, y + h - 2.0);
    cv.color = C_BLACK;
    cv.line(x, y + h - 1.0, x + w - 1.0, y + h - 1.0);
    cv.line(x + w - 1.0, y, x + w - 1.0, y + h - 1.0);
}

fn wrap(font: &crate::gmdata::FontDef, para: &str, max: i32) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    for word in para.split(' ') {
        let cand = if cur.is_empty() { word.to_string() } else { format!("{cur} {word}") };
        if Canvas::text_width(font, &cand) > max && !cur.is_empty() {
            out.push(std::mem::take(&mut cur));
            cur = word.to_string();
        } else {
            cur = cand;
        }
    }
    out.push(cur);
    out
}

fn check_names<'a>(kind: &str, got: impl Iterator<Item = &'a str>, want: &[&str]) -> Result<(), String> {
    let got: Vec<&str> = got.collect();
    if got.len() < want.len() || got.iter().zip(want).any(|(g, w)| g != w) {
        return Err(format!("unexpected {kind} list in data.win ({got:?}); is this «Страна возможностей»?"));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn police_fine_is_charged_after_message() {
        let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("assets");
        let data = crate::assets::load(&dir).expect("extracted game assets");
        let save = std::env::temp_dir().join(format!("strana-police-test-{}.json", std::process::id()));
        let mut g = Game::new(data, save).unwrap();
        g.w.gas = Some(Gas::create());
        g.w.g = Globals::initial();
        g.w.g.money = 50.0;
        g.gas_mut().police_act = 1.0;
        let i = g.instance_create(0.0, 0.0, Obj::Listovki);
        g.w.insts[i].go = 1.0;
        g.w.insts[i].police = 101.0;

        g.ev_step(i);
        assert_eq!(g.w.g.money, 50.0);
        assert_eq!(g.modal.len(), 1);
        g.input.confirm = true;
        g.tick();
        assert_eq!(g.w.g.money, 25.0);
        assert!(g.modal.is_empty());
    }

    #[test]
    fn leaflet_completion_continues_after_message() {
        let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("assets");
        let data = crate::assets::load(&dir).expect("extracted game assets");
        let save = std::env::temp_dir().join(format!("strana-leaflet-test-{}.json", std::process::id()));
        let mut g = Game::new(data, save).unwrap();
        g.w.gas = Some(Gas::create());
        g.w.g = Globals::initial();
        g.gas_mut().police_act = 0.0;
        let i = g.instance_create(0.0, 0.0, Obj::Listovki);
        g.w.insts[i].go = 1.0;
        g.w.insts[i].find = 1.0;
        g.w.insts[i].progress = 101.0;
        g.w.insts[i].police = 8.0;
        let income = g.w.g.income;

        g.ev_step(i);
        assert_eq!(g.gas().kleys, 1.0);
        assert_eq!(g.gas().police_act, 0.0);
        assert_eq!(g.w.g.income, income);
        assert_eq!(g.w.insts[i].police, 8.0);
        g.input.confirm = true;
        g.tick();
        assert_eq!(g.gas().police_act, 10.0);
        assert_eq!(g.w.g.income, crate::scripts::get_zp(g.w.g.work) - crate::scripts::get_home_op(g.gas().home) + 3.0);
        assert_eq!(g.w.insts[i].police, 0.0);
    }

    /// The original clock keeps running on the title screen and can overwrite a previous save.
    #[test]
    fn title_screen_clock_autosaves() {
        let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("assets");
        let data = crate::assets::load(&dir).expect("extracted game assets");
        let tmp = std::env::temp_dir().join(format!("strana-title-clock-test-{}", std::process::id()));
        std::fs::create_dir_all(&tmp).unwrap();
        let save = tmp.join("save.json");
        std::fs::write(&save, b"previous save").unwrap();
        let mut g = Game::new(data, save.clone()).unwrap();

        for _ in 0..30 * 60 {
            if g.w.room == R_INTRO {
                break;
            }
            g.tick();
        }
        assert_eq!(g.w.room, R_INTRO);
        let start = (g.gas().time, g.gas().day);
        for _ in 0..30 * 30 {
            g.tick();
        }
        assert!(g.gas().day > start.1);
        let saved: World = serde_json::from_slice(&std::fs::read(&save).unwrap()).unwrap();
        assert_eq!(saved.room, R_INTRO);
        assert!(saved.gas.unwrap().day > start.1);
        let _ = std::fs::remove_dir_all(&tmp);
    }
}
