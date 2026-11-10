//! Object events, ported one-to-one from the decompiled GML.

use crate::canvas::{merge_color, Canvas, HAlign, C_AQUA, C_BLACK, C_BLUE, C_GRAY, C_PURPLE, C_RED, C_WHITE, C_YELLOW};
use crate::game::*;
use crate::gml::{floor_div, irandom_range, not, point_direction, ps, random, round, string, truthy};
use crate::particles::{PartSystem, PartType, Shape};
use crate::txt;

impl Game {
    pub(crate) fn ev_create(&mut self, i: usize) {
        let obj = self.w.insts[i].obj;
        let inst = &mut self.w.insts[i];
        match obj {
            Obj::OButton => {
                inst.w2 = 100.0;
                inst.sme = 0.0;
            }
            Obj::Listovki => {
                inst.go = 0.0;
                inst.progress = 0.0;
                inst.police = 0.0;
                inst.find = 1.0;
            }
            Obj::Razgruzka => {
                inst.go = 0.0;
                inst.progress = 0.0;
                inst.police = 0.0;
            }
            Obj::Downloader => {
                inst.go = 0.0;
                inst.progress = 0.0;
                inst.police = 0.0;
                inst.name.clear();
                inst.mb = 1.0;
                inst.site = 0;
                inst.shift = 0.0;
            }
            Obj::Oinet | Obj::Odisks => {
                inst.image_index = 0.0;
                inst.dbl = 0.0;
            }
            Obj::Oprogram => {
                inst.image_index = 0.0;
                inst.dbl = 0.0;
                let g = self.gas();
                if !truthy(g.soft_u[8]) && !truthy(g.soft_u[9]) && !truthy(g.soft_u[10]) {
                    self.w.insts[i].dead = true;
                }
            }
            Obj::Programgame => {
                inst.go = 0.0;
                inst.fin = 0.0;
                inst.bug = 0;
                inst.met = -1.0;
                inst.typ = 0;
            }
            Obj::Sym => inst.met = 0.0,
            Obj::Particlez => {
                inst.alarm[0] = 200;
                inst.a = 0.0;
                let (x, y) = (inst.x, inst.y);
                let mut sys = PartSystem::default();
                let p1 = sys.add_type(PartType {
                    shape: Shape::Line,
                    size: (0.1, 0.14, 0.05),
                    scale: (2.54, 0.2),
                    colors: [5083229, 9110908, 1590528],
                    alphas: [0.98, 0.31, 0.06],
                    speed: (1.06, 1.8, 0.01),
                    direction: (0.0, 359.0, 0.0),
                    orientation: (0.0, 0.0, 0.0, true),
                    additive: true,
                    life: (48, 82),
                });
                sys.stream(x, y, p1, 5);
                self.w.insts[i].particle1 = p1;
                self.w.parts = Some(sys);
            }
            Obj::Partfader => inst.a = 0.0,
            Obj::Partunfader => inst.a = 1.0,
            Obj::Pcc => {
                inst.m = 0.0;
                inst.alarm[0] = 1;
                inst.image_blend = C_GRAY;
            }
            Obj::Uprav => {
                inst.en = 100.0;
                inst.za = 3.0;
                inst.alarm[0] = 10;
                inst.es = 50.0;
                inst.fin = 0.0;
                inst.dfin = 0.0;
            }
            Obj::Dot | Obj::Dot2 => {
                inst.alarm[0] = 150;
                inst.drop = 0.0;
            }
            Obj::Gaminator | Obj::Unfader2 => inst.a = 1.0,
            Obj::Gas => {}
        }
    }

    /// Instance creation code from the room editor, keyed by instance id.
    pub(crate) fn creation_code(&mut self, i: usize) {
        enum Extra {
            None,
            Btn50,
            Btn100,
        }
        let id = self.w.insts[i].id;
        let (d, extra, allow): (i32, Extra, Option<usize>) = match id {
            100002 => (88, Extra::None, None),
            100003 => (89, Extra::None, None),
            100005 => {
                // The computer button on the main screen.
                let inst = &mut self.w.insts[i];
                inst.d = 53;
                inst.sprite = S_B_COMP;
                inst.sme = 25.0;
                self.ballow(i, 1);
                return;
            }
            100006 => (1, Extra::None, None),
            100007 => (2, Extra::None, None),
            100008 => (40, Extra::None, Some(0)),
            100009..=100012 => (3 + (id - 100009) as i32, Extra::Btn50, None),
            100013 => (7, Extra::None, None),
            100014 => (12, Extra::None, None),
            100015 => (13, Extra::None, None),
            100016 => (11, Extra::Btn50, None),
            100018 => (17, Extra::None, None),
            100019 => (81, Extra::None, Some(3)),
            100020 => (7, Extra::None, None),
            100021 => (8, Extra::None, None),
            100022 => (9, Extra::None, None),
            100023 => (10, Extra::Btn100, None),
            100025 => (14, Extra::None, None),
            100026 => (15, Extra::None, None),
            100027 => (16, Extra::None, None),
            100028 => (80, Extra::Btn100, Some(2)),
            100029 => (7, Extra::None, None),
            100030 => (18, Extra::None, None),
            100031 => (19, Extra::None, None),
            100032 => (20, Extra::None, None),
            100033 => (30, Extra::Btn50, None),
            100034 => (31, Extra::Btn50, None),
            100035..=100040 => (21 + (id - 100035) as i32, Extra::None, None),
            100041 => (7, Extra::None, None),
            100042 => (52, Extra::None, None),
            100043 => (41, Extra::Btn50, None),
            100044 => (45, Extra::None, None),
            100045 => (46, Extra::None, None),
            100046 => (47, Extra::None, None),
            100047 => (48, Extra::None, None),
            100048 => (50, Extra::None, None),
            100049 => (49, Extra::None, None),
            100050 => (51, Extra::None, None),
            100051 => (42, Extra::Btn50, None),
            100052 => (7, Extra::None, None),
            100056 => (57, Extra::None, None),
            100057 => (55, Extra::None, None),
            100058 => (56, Extra::None, None),
            100059 => (59, Extra::None, None),
            100060..=100064 => (60 + (id - 100060) as i32, Extra::None, None),
            100066..=100072 => (65 + (id - 100066) as i32, Extra::None, None),
            100073 => (59, Extra::None, None),
            100074..=100081 => (72 + (id - 100074) as i32, Extra::None, None),
            100082 => (59, Extra::None, None),
            100083..=100085 => (90 + (id - 100083) as i32, Extra::Btn50, None),
            100086 => (93, Extra::Btn50, Some(4)),
            100087 => (95, Extra::None, None),
            100088 => (82, Extra::Btn50, None),
            100090 => (83, Extra::None, None),
            100091 => (84, Extra::None, None),
            100092 => (85, Extra::Btn100, None),
            100093 => (86, Extra::None, None),
            100189 | 100190 | 100194 | 100200 | 100234 => {
                self.w.insts[i].m = 1.0;
                return;
            }
            _ => return,
        };
        self.w.insts[i].d = d;
        match extra {
            Extra::None => {}
            Extra::Btn50 => self.btn50(i),
            Extra::Btn100 => self.btn100(i),
        }
        if let Some(n) = allow {
            self.ballow(i, n);
        }
    }

    pub(crate) fn btn50(&mut self, i: usize) {
        let inst = &mut self.w.insts[i];
        inst.sprite = S_BUTTON2;
        inst.x -= 50.0;
        inst.w2 = 125.0;
    }

    fn btn100(&mut self, i: usize) {
        let inst = &mut self.w.insts[i];
        inst.sprite = S_BUTTON3;
        inst.x -= 100.0;
        inst.w2 = 150.0;
    }

    pub(crate) fn ballow(&mut self, i: usize, n: usize) {
        if !truthy(self.gas().allow[n]) {
            self.w.insts[i].dead = true;
        }
    }

    pub(crate) fn ev_alarm(&mut self, i: usize, a: usize) {
        let obj = self.w.insts[i].obj;
        match (obj, a) {
            (Obj::Downloader, 1) => self.shift_site(i, -777.0, 777.0, 1),
            (Obj::Downloader, 2) => {
                let shift = self.w.insts[i].shift;
                for k in self.all(Obj::OButton) {
                    self.w.insts[k].x += shift;
                }
                let inst = &mut self.w.insts[i];
                inst.shift = 0.0;
                inst.site = 0;
            }
            (Obj::Downloader, 4) => self.shift_site(i, -1646.0, 1646.0, 2),
            (Obj::Downloader, 5) => self.shift_site(i, 777.0, -777.0, 3),
            (Obj::Oinet | Obj::Odisks | Obj::Oprogram, 0) => self.w.insts[i].dbl = 0.0,
            (Obj::Programgame, 0) => self.programgame_write(i),
            (Obj::Programgame, 1) => self.programgame_run(i),
            (Obj::Programgame, 2) => {
                if self.w.insts[i].bug == -10 {
                    self.msg(txt::NO_BUGS);
                    return;
                }
                self.programgame_debug(i, 0);
            }
            (Obj::Particlez, 0) => {
                let (x, y) = (self.w.insts[i].x, self.w.insts[i].y);
                if let Some(sys) = self.w.parts.as_mut() {
                    let p2 = sys.add_type(PartType {
                        shape: Shape::Flare,
                        size: (0.1, 0.93, 0.01),
                        scale: (1.5, 1.5),
                        colors: [5324883, 12632256, 16777088],
                        alphas: [0.76, 0.74, 0.07],
                        speed: (4.63, 6.0, -0.08),
                        direction: (0.0, 359.0, -1.0),
                        orientation: (0.0, 0.0, 1.0, true),
                        additive: true,
                        life: (38, 155),
                    });
                    sys.stream(x, y, p2, 3);
                    self.w.insts[i].particle2 = p2;
                }
                self.w.insts[i].alarm[1] = 50;
            }
            (Obj::Particlez, 1) => {
                let (x, y) = (self.w.insts[i].x, self.w.insts[i].y);
                self.instance_create(x, y, Obj::Partfader);
                self.w.insts[i].alarm[2] = 150;
            }
            (Obj::Particlez, 2) => {
                let (p1, p2) = (self.w.insts[i].particle1, self.w.insts[i].particle2);
                if let Some(sys) = self.w.parts.as_mut() {
                    sys.clear_type(p1);
                    sys.clear_type(p2);
                    sys.clear();
                }
            }
            (Obj::Pcc, 0) => {
                let inst = &mut self.w.insts[i];
                if inst.m == 1.0 {
                    inst.image_blend = C_YELLOW;
                }
            }
            (Obj::Pcc, 1) => {
                let (x, y, id) = {
                    let inst = &mut self.w.insts[i];
                    inst.m = -1.0;
                    inst.image_blend = C_RED;
                    (inst.x, inst.y, inst.id)
                };
                let p = self.instance_create(x, y, Obj::Dot2);
                let d = 50.0 + random(50.0);
                let dot = &mut self.w.insts[p];
                dot.speed = 1.0;
                dot.direction = random(360.0);
                dot.alarm[0] = d as i32;
                dot.o = id;
                self.w.insts[i].alarm[1] = (d * 2.0 + 10.0) as i32;
            }
            (Obj::Uprav, 0) => self.uprav_tick(i),
            (Obj::Uprav, 1) => {
                let inst = &mut self.w.insts[i];
                let hi = (2.0 + inst.za / 5.0).floor() as i32;
                inst.en = (inst.en + irandom_range(1, hi) as f64).min(100.0);
            }
            (Obj::Uprav, 2) => {
                let inst = &mut self.w.insts[i];
                inst.dfin = not(inst.dfin);
                inst.alarm[2] = 25;
            }
            (Obj::Dot | Obj::Dot2, 0) => {
                let inst = &mut self.w.insts[i];
                inst.drop = 0.1;
                inst.alarm[1] = 3;
            }
            (Obj::Dot | Obj::Dot2, 1) => {
                let inst = &mut self.w.insts[i];
                inst.drop += 0.1;
                inst.alarm[1] = 3;
                if inst.drop >= 1.0 {
                    inst.dead = true;
                }
            }
            (Obj::Gaminator, 0) => self.goto_room(R_INTRO),
            _ => {}
        }
    }

    /// Downloader alarms that scroll every button to show another "web site".
    fn shift_site(&mut self, i: usize, dx: f64, shift: f64, site: i32) {
        for k in self.all(Obj::OButton) {
            self.w.insts[k].x += dx;
        }
        let inst = &mut self.w.insts[i];
        inst.site = site;
        inst.shift = shift;
    }

    pub(crate) fn ev_mouse_pressed(&mut self, i: usize) {
        let obj = self.w.insts[i].obj;
        match obj {
            Obj::OButton => self.w.insts[i].image_index = 1.0,
            Obj::Oinet | Obj::Odisks | Obj::Oprogram => {
                self.w.insts[i].image_index = 1.0;
                if !truthy(self.w.insts[i].dbl) {
                    self.w.insts[i].dbl = 1.0;
                    self.w.insts[i].alarm[0] = 9;
                    return;
                }
                match obj {
                    Obj::Oinet => {
                        if self.gas().inet <= 0.0 {
                            self.msg(txt::CONNECT_FAILED);
                            self.w.insts[i].image_index = 0.0;
                        } else {
                            self.goto_room(R_INTERNET);
                        }
                    }
                    Obj::Odisks => {
                        let (cd, soft, cdmb) = (self.gas().cd, self.gas().soft, self.gas().cdmb);
                        if cd <= 0 {
                            self.msg(txt::NO_CD_DRIVE);
                        } else if cd <= 3 {
                            self.msg(txt::NO_CD_WRITER);
                        } else if soft < cdmb {
                            let t = ps(self.data.s(txt::CD_NEEDS_SOFT), &[string(cdmb)]);
                            self.show_message(t);
                        } else {
                            let g = self.gas_mut();
                            g.cdmb += 200.0;
                            g.cdsbor += 1.0;
                            self.msg(txt::CD_DISTRIBUTED);
                            self.upd_income();
                        }
                        self.w.insts[i].image_index = 0.0;
                    }
                    _ => self.goto_room(R_PASCAL),
                }
            }
            _ => {}
        }
    }

    pub(crate) fn ev_step(&mut self, i: usize) {
        match self.w.insts[i].obj {
            Obj::Listovki => self.listovki_step(i),
            Obj::Razgruzka => {
                if !truthy(self.w.insts[i].go) {
                    return;
                }
                let inst = &mut self.w.insts[i];
                inst.progress += random(0.22);
                if inst.progress >= 100.0 {
                    inst.progress = 0.0;
                    inst.go = 0.0;
                    let prize = round(random(7.0)) + 14.0;
                    self.w.g.money += prize;
                    let t = ps(self.data.s(txt::EARNED), &[string(prize)]);
                    self.show_message(t);
                    self.gas_mut().rzar += prize;
                }
            }
            Obj::Downloader => {
                if !truthy(self.w.insts[i].go) {
                    return;
                }
                let m = self.gas().modem as f64;
                let rate = m * 0.005
                    + m * m * 0.0005
                    + ((m / 5.0).floor() * (m - 3.0)) / 100.0
                    + ((m / 10.0).floor() * (m - 9.0)) / 10.0;
                let inst = &mut self.w.insts[i];
                inst.progress += random(rate);
                if inst.progress >= inst.mb {
                    inst.progress = 0.0;
                    inst.go = 0.0;
                    let (name, mb, soft) = (inst.name.clone(), inst.mb, inst.soft);
                    let t = ps(self.data.s(txt::DOWNLOADED), &[name, string(mb)]);
                    self.show_message(t);
                    self.w.g.iq += 1.0;
                    let g = self.gas_mut();
                    g.download += mb;
                    g.soft += mb;
                    g.soft_u[soft as usize] = 1.0;
                }
            }
            Obj::Sym => {
                let (mx, my) = (self.input.mx, self.input.my);
                let inst = &mut self.w.insts[i];
                inst.v = mx >= inst.x - 1.0 && mx <= inst.x + 17.0 && my >= inst.y && my <= inst.y + 25.0;
                if self.input.pressed && inst.v {
                    for k in self.all(Obj::Sym) {
                        self.w.insts[k].met = 0.0;
                    }
                    self.w.insts[i].met = 1.0;
                    let m = self.w.insts[i].m;
                    if let Some(pg) = self.first(Obj::Programgame) {
                        self.w.insts[pg].met = m;
                    }
                    self.msg(txt::MARK_SET);
                }
            }
            Obj::Gaminator => {
                let inst = &mut self.w.insts[i];
                inst.a -= 0.025;
                if inst.a <= -2.0 {
                    self.goto_room(R_INTRO);
                }
            }
            Obj::Unfader2 => {
                let inst = &mut self.w.insts[i];
                inst.a -= 0.03;
                if inst.a <= 0.0 {
                    inst.dead = true;
                }
            }
            Obj::Uprav => {
                let inst = &mut self.w.insts[i];
                if inst.en < 100.0 && inst.alarm[1] == -1 {
                    inst.alarm[1] = irandom_range(3, 15);
                }
            }
            _ => {}
        }
    }

    fn listovki_step(&mut self, i: usize) {
        if !truthy(self.w.insts[i].go) {
            return;
        }
        let (pva, police_act, time) = (self.gas().pva, self.gas().police_act, self.gas().time);
        let inst = &mut self.w.insts[i];
        if truthy(inst.find) {
            inst.progress += random(0.5);
        } else {
            inst.progress += random(0.4 + pva * 0.4);
        }
        if truthy(police_act) {
            inst.police += random(police_act / 6.0 + time.sin() / 6.0);
            if inst.police >= 100.0 {
                inst.progress = 0.0;
                inst.go = 0.0;
                inst.police = 0.0;
                let money = self.w.g.money;
                let lose = (money / 2.0).ceil().clamp(5.0, 100.0).min(money);
                let t = ps(self.data.s(txt::CAUGHT_BY_POLICE), &[string(lose)]);
                self.show_message(t);
                self.w.g.money -= lose;
            }
        }
        let inst = &mut self.w.insts[i];
        if inst.progress >= 100.0 {
            inst.progress = 0.0;
            inst.go = 0.0;
            if truthy(inst.find) {
                self.gas_mut().kleys += 1.0;
                self.msg(txt::POSTER_HIRED);
                self.gas_mut().police_act += 10.0;
                self.upd_income();
            } else {
                let prize = if truthy(self.gas().list_first) {
                    self.gas_mut().list_first = 0.0;
                    5.0
                } else {
                    irandom_range(3, 8) as f64
                };
                self.w.g.money += prize;
                let t = ps(self.data.s(txt::EARNED), &[string(prize)]);
                self.show_message(t);
                self.gas_mut().police_act += (prize / 4.0).floor() + 1.0;
            }
            self.w.insts[i].police = 0.0;
        }
    }

    pub(crate) fn ev_collision_pcc(&mut self, i: usize, p: usize) {
        if self.w.insts[i].obj == Obj::Dot {
            if !truthy(self.w.insts[p].m) {
                self.w.insts[i].dead = true;
                let pcc = &mut self.w.insts[p];
                pcc.m = 1.0;
                pcc.image_blend = C_YELLOW;
                pcc.alarm[1] = -1;
            }
        } else if self.w.insts[p].m != -1.0 {
            self.w.insts[i].dead = true;
            let pcc = &mut self.w.insts[p];
            pcc.m = -1.0;
            pcc.image_blend = C_RED;
        }
    }

    // ---- programgame: the "write a program" mini-game -------------------------------------

    fn programgame_write(&mut self, i: usize) {
        let (x, y, go, typ) = {
            let pg = &mut self.w.insts[i];
            pg.a = 19.0;
            (pg.x, pg.y, pg.go, pg.typ as f64)
        };
        let c = irandom_range(36, 126);
        let p = self.instance_create(x + 19.0 * (go % 25.0), y + 27.0 * floor_div(go, 25.0), Obj::Sym);
        let s = &mut self.w.insts[p];
        s.c = char::from(c as u8).to_string();
        s.d = c;
        s.m = go;
        let work = self.w.g.work;
        let pg = &mut self.w.insts[i];
        pg.go += 1.0;
        if pg.go < 35.0 + 50.0 * typ + random(14.0 + 11.0 * typ) {
            pg.alarm[0] = 4;
        } else {
            pg.fin = 1.0;
            pg.bug = irandom_range(2, pg.go as i32 - 3);
            if work == 10 {
                pg.bug = -10;
            }
        }
    }

    fn programgame_run(&mut self, i: usize) {
        if !truthy(self.w.insts[i].fin) {
            self.msg(txt::NOT_WRITTEN);
            return;
        }
        if truthy(self.w.insts[i].bug as f64) {
            self.msg(txt::BUG_CRASH);
            return;
        }
        self.msg(txt::PROGRAM_RUNS);
        for k in self.all(Obj::Sym) {
            self.w.insts[k].dead = true;
        }
        let typ = {
            let pg = &mut self.w.insts[i];
            pg.met = -1.0;
            pg.bug = -1;
            pg.go = 0.0;
            pg.fin = 0.0;
            pg.typ
        };
        if typ == 0 {
            let hello = self.gas().hello;
            let piq = if hello <= 1.0 { 3.0 } else { 1.0 };
            if irandom_range(0, hello as i32) <= 1 {
                let t = self.s(txt::HELLO_IQ) + &string(piq);
                self.show_message(t);
                self.w.g.iq += piq;
                self.gas_mut().hello += 1.0;
            } else {
                self.msg(txt::HELLO_NO_IQ);
            }
        }
        if typ == 1 {
            let programs = self.gas().programs;
            let prize = (irandom_range(15, self.w.g.iq as i32) as f64 * (1.0 + programs / 5.0)).ceil();
            self.w.g.money += prize;
            let piq = if programs <= 1.0 { 3.0 } else { 1.0 };
            if irandom_range(0, programs as i32) <= 1 {
                let t = ps(self.data.s(txt::SOLD_IQ), &[string(prize), string(piq)]);
                self.show_message(t);
                self.w.g.iq += piq;
                self.gas_mut().programs += 1.0;
            } else {
                let t = ps(self.data.s(txt::SOLD_NO_IQ), &[string(prize)]);
                self.show_message(t);
            }
            let dsoft = irandom_range(16, 64) as f64;
            self.gas_mut().soft += dsoft;
            let t = ps(self.data.s(txt::PROGRAM_DISK), &[string(dsoft)]);
            self.show_message(t);
        }
        if typ == 2 {
            self.msg(txt::WORM_WRITTEN);
            let bot = self.gas().bot;
            if bot >= 10.0 {
                let kop = (bot / 10.0).floor() * 5.0;
                self.gas_mut().worm += kop;
                let t = self.s(txt::WORM_SPREAD) + &string(kop) + self.data.s(txt::WORM_COPIES);
                self.show_message(t);
            }
            self.gas_mut().worm += 1.0;
        }
    }

    /// The debug loop of Alarm 2. A mark that is not the bug asks whether to go on, which in
    /// GML blocked inside the loop; here the loop resumes from the dialog's answer.
    pub(crate) fn programgame_debug(&mut self, i: usize, from: i32) {
        let go = self.w.insts[i].go as i32;
        for k in from..=go {
            let (bug, met) = (self.w.insts[i].bug, self.w.insts[i].met as i32);
            if k == bug && k == met {
                self.msg(txt::BUG_FIXED);
                self.w.insts[i].bug = -1;
            } else if k == met {
                let t = self.s(txt::MARK_NOT_BUG);
                let id = self.w.insts[i].id;
                self.show_question(t, Cont::Debug { inst: id, resume: k + 1 });
                return;
            } else if k == bug {
                self.msg(txt::ERROR);
                return;
            }
        }
    }

    // ---- uprav: the botnet finale ------------------------------------------------------------

    fn uprav_tick(&mut self, i: usize) {
        self.w.insts[i].alarm[0] = 15;
        let mut tmp = 0.0;
        let pccs = self.all(Obj::Pcc);
        let tmp2 = pccs.len() as f64;
        for p in pccs {
            let (es, za) = (self.w.insts[i].es, self.w.insts[i].za);
            let pcc = &mut self.w.insts[p];
            if truthy(pcc.m) {
                tmp += 1.0;
            } else if !truthy(es) && pcc.m == 0.0 && random(250.0) < za {
                pcc.alarm[1] = 3;
                self.w.insts[i].es += random(100.0 / (1.0 + za));
            }
        }
        let u = &mut self.w.insts[i];
        if truthy(u.es) {
            u.es = (u.es - 1.0).max(0.0);
        }
        u.za = (100.0 * tmp / tmp2).floor();
        if u.za >= 100.0 && !truthy(u.fin) {
            u.fin = 1.0;
            u.alarm[2] = 60;
        }
        if tmp == 0.0 {
            self.msg(txt::MIND_DESTROYED);
            self.game_end();
        }
    }

    pub(crate) fn uprav_fire(&mut self, i: usize) {
        let en = self.w.insts[i].en;
        if en < 25.0 {
            return;
        }
        let (mx, my) = (self.input.mx, self.input.my);
        for p in self.all(Obj::Pcc) {
            let (x, y, m, id) = {
                let pcc = &self.w.insts[p];
                (pcc.x, pcc.y, pcc.m, pcc.id)
            };
            if m != 1.0 {
                continue;
            }
            let d = self.instance_create(x, y, Obj::Dot);
            let dot = &mut self.w.insts[d];
            dot.o = id;
            dot.direction = point_direction(x, y, mx, my);
            dot.speed = 1.0;
            dot.alarm[0] = en as i32;
        }
        self.w.insts[i].en = 0.0;
    }

    // ---- Draw events ------------------------------------------------------------------------

    pub(crate) fn ev_draw(&mut self, i: usize, cv: &mut Canvas) {
        let inst = self.w.insts[i].clone();
        let (x, y) = (inst.x, inst.y);
        match inst.obj {
            Obj::OButton => {
                self.draw_sprite(cv, inst.sprite, inst.image_index, x, y, inst.image_blend, 1.0);
                cv.font = FNT_01;
                cv.color = C_BLACK;
                cv.halign = HAlign::Center;
                let s = self.doptext(inst.d);
                let label = self.button_label(inst.d);
                if s.is_empty() {
                    cv.text(&self.data, x + inst.w2 + inst.sme, y + 13.0, &label);
                } else {
                    cv.text(&self.data, x + inst.w2, y + 5.0, &label);
                    cv.color = crate::canvas::C_MAROON;
                    cv.font = FNT_00;
                    cv.text(&self.data, x + inst.w2, y + 28.0, &s);
                }
                cv.halign = HAlign::Left;
            }
            Obj::Listovki => {
                if !truthy(inst.go) {
                    return;
                }
                cv.font = FNT_01;
                let (ww, hh) = (200.0, 40.0);
                let mut hha = 0.0;
                cv.color = C_BLACK;
                if self.gas().police_act > 0.0 {
                    hha = 15.0;
                    cv.color = C_RED;
                    cv.rect(x, y, x + round(ww * inst.police / 100.0), y + hha, false);
                }
                cv.color = C_BLUE;
                cv.rect(x, y + hha, x + round(ww * inst.progress / 100.0), y + hh, false);
                cv.color = C_BLACK;
                cv.rect(x, y, x + ww, y + hh, true);
                cv.halign = HAlign::Center;
                let f = if truthy(inst.find) { txt::HIRING_PCT } else { txt::POSTING_PCT };
                let t = ps(self.data.s(f), &[string(inst.progress.floor())]);
                cv.text(&self.data, x + ww / 2.0, y + hh / 2.0, &t);
                cv.halign = HAlign::Left;
            }
            Obj::Razgruzka => {
                if !truthy(inst.go) {
                    return;
                }
                cv.font = FNT_01;
                let (ww, hh) = (358.0, 40.0);
                cv.color = C_BLUE;
                cv.rect(x, y, x + round(ww * inst.progress / 100.0), y + hh, false);
                cv.color = C_WHITE;
                cv.rect(x, y, x + ww, y + hh, true);
                cv.halign = HAlign::Center;
                let t = ps(self.data.s(txt::UNLOADING_PCT), &[string(inst.progress.floor())]);
                cv.text(&self.data, x + ww / 2.0, y + hh / 2.0 - 5.0, &t);
                cv.halign = HAlign::Left;
            }
            Obj::Downloader => {
                if !truthy(inst.go) {
                    return;
                }
                cv.font = FNT_01;
                let (ww, hh) = (751.0, 32.0);
                cv.color = C_BLUE;
                cv.rect(x, y, x + round(ww * inst.progress / inst.mb), y + hh, false);
                cv.color = C_BLACK;
                cv.halign = HAlign::Center;
                let pct = (100.0 * inst.progress / inst.mb).floor();
                let t = ps(self.data.s(txt::DOWNLOADING), &[inst.name.clone(), string(inst.mb), string(pct)]);
                cv.text(&self.data, x + ww / 2.0, y + hh / 2.0 - 5.0, &t);
                cv.halign = HAlign::Left;
            }
            Obj::Sym => {
                cv.font = FNT_PROG;
                if inst.v {
                    cv.color = C_BLUE;
                    cv.rect(x - 1.0, y, x + 17.0, y + 28.0, false);
                }
                cv.color = C_AQUA;
                if inst.v {
                    cv.color = C_YELLOW;
                }
                if truthy(inst.met) {
                    cv.color = C_RED;
                }
                cv.text(&self.data, x, y, &inst.c);
            }
            Obj::Partfader => {
                cv.color = merge_color(C_WHITE, C_PURPLE, 0.5);
                cv.additive = true;
                cv.alpha = inst.a.min(1.0);
                cv.rect(-1.0, -1.0, 801.0, 601.0, false);
                cv.additive = false;
                let a = inst.a + 0.02;
                self.w.insts[i].a = a;
                if a >= 1.0 {
                    cv.color = C_WHITE;
                    cv.alpha = (a - 1.0).min(1.0);
                    cv.rect(-1.0, -1.0, 801.0, 601.0, false);
                    if a >= 3.5 {
                        self.goto_room(R_MAP);
                    }
                }
                cv.alpha = 1.0;
            }
            Obj::Partunfader => {
                cv.color = C_WHITE;
                cv.alpha = inst.a.max(0.0);
                cv.rect(-1.0, -1.0, 801.0, 601.0, false);
                cv.alpha = 1.0;
                let a = inst.a - 0.01 / inst.a.max(0.1);
                self.w.insts[i].a = a;
                if a < 0.025 {
                    self.w.insts[i].dead = true;
                }
            }
            Obj::Uprav => {
                cv.color = C_YELLOW;
                cv.font = FNT_01;
                cv.halign = HAlign::Center;
                let t = ps(self.data.s(txt::ENERGY_PCT), &[string(inst.en)]);
                cv.text(&self.data, 400.0, y, &t);
                let t = ps(self.data.s(txt::NETWORKS_CAPTURED), &[string(inst.za)]);
                cv.text(&self.data, 400.0, y + 41.0, &t);
                if truthy(inst.dfin) {
                    cv.text(&self.data, 400.0, 20.0, self.data.s(txt::THE_END));
                }
                cv.halign = HAlign::Left;
            }
            Obj::Dot | Obj::Dot2 => {
                if inst.drop != 0.0 {
                    cv.alpha = 1.0 - inst.drop;
                }
                cv.color = if inst.obj == Obj::Dot { C_YELLOW } else { C_RED };
                if let Some(o) = self.find_id(inst.o) {
                    let (ox, oy) = (self.w.insts[o].x, self.w.insts[o].y);
                    cv.line(x, y, ox, oy);
                }
                cv.alpha = 1.0;
            }
            Obj::Gaminator => {
                cv.color = C_BLACK;
                let aa = if inst.a > 0.0 {
                    inst.a
                } else if inst.a < -1.0 {
                    inst.a.abs() - 1.0
                } else {
                    0.0
                };
                cv.alpha = aa;
                cv.rect(-1.0, -1.0, 801.0, 601.0, false);
                cv.alpha = 1.0;
            }
            Obj::Unfader2 => {
                cv.color = C_BLACK;
                cv.alpha = inst.a.abs();
                cv.rect(-1.0, -1.0, 801.0, 601.0, false);
                cv.alpha = 1.0;
            }
            // No draw event: the runner draws the sprite, if any.
            Obj::Oinet | Obj::Odisks | Obj::Oprogram | Obj::Pcc | Obj::Programgame | Obj::Particlez => {
                self.draw_sprite(cv, inst.sprite, inst.image_index, x, y, inst.image_blend, 1.0);
            }
            Obj::Gas => {}
        }
    }
}
