//! The game's scripts (`BAction`, `GetWork`, `_day`, …) and the `gas` controller object.

use crate::canvas::{Canvas, HAlign, C_BLACK, C_LTGRAY, C_MAROON, C_RED, C_WHITE};
use crate::game::*;
use crate::gml::{choose, floor_div, not, ps, random, round, string, truthy};
use crate::txt;

pub fn get_zp(w: i32) -> f64 {
    ((w + 1) * (w - 1) * 3 + 1) as f64
}

pub fn get_home_op(h: i32) -> f64 {
    (5 * h - 7).max(0) as f64
}

pub fn cost_com(level: i32, k: i32) -> f64 {
    (level * k * 10 - 10) as f64
}

pub fn get_drive(hard: i32) -> f64 {
    match hard {
        1 => 120.0,
        2 => 280.0,
        3 => 600.0,
        4 => 2000.0,
        5 => 40000.0,
        _ => 100.0,
    }
}

pub fn get_mb(d: i32) -> f64 {
    match d {
        60 => 10.0,
        61 => 37.0,
        62 => 1.0,
        63 => 4.0,
        64 => 12.0,
        65 => 40.0,
        66 => 68.0,
        67 => 81.0,
        68 => 5.0,
        69 => 13.0,
        70 => 297.0,
        72 => 27.0,
        73 => 16.0,
        74 => 430.0,
        75 => 1200.0,
        76 => 1765.0,
        77 => 150.0,
        78 => 2500.0,
        79 => 137.0,
        _ => 0.0,
    }
}

pub fn get_game_cost(d: i32) -> f64 {
    match d {
        72 => 17.0,
        73 => 8.0,
        74 => 30.0,
        75 => 55.0,
        76 => 40.0,
        77 => 19.0,
        78 => 100.0,
        79 => 19.0,
        _ => 0.0,
    }
}

/// Upper bounds of the hardware arrays (`_comp[0]`, `_memo[0]`, …).
const MAX_COMP: i32 = 3;
const MAX_MEMO: i32 = 3;
const MAX_HARD: i32 = 5;
const MAX_VIDEO: i32 = 3;
const MAX_MODEM: i32 = 18;
const MAX_CD: i32 = 4;
const MAX_PRINTER: i32 = 3;
const MAX_WINDOWS: i32 = 4;

/// Hardware kinds sold in the tech shop.
#[derive(Clone, Copy)]
enum Hw {
    Comp,
    Memo,
    Hard,
    Video,
    Modem,
    Cd,
    Printer,
    Windows,
}

impl Gas {
    /// `gas` Create event (the globals are reset by the caller).
    pub fn create() -> Gas {
        let mut button = vec![None; 96];
        for (d, b) in button.iter_mut().enumerate() {
            *b = txt::button_label(d as i32);
        }
        Gas {
            alarm0: 30,
            button,
            home: 1,
            watt: 0.0,
            phone: 0.0,
            rabota: 0.0,
            propiska: 0.0,
            diplom: 0.0,
            day: 10.0,
            month: 5.0,
            year: 2000.0,
            time: 1.0,
            metro_ch: 15.0,
            metro_nah: 0.0,
            tokens: 0.0,
            police_act: 0.0,
            list_first: 1.0,
            pva: 0.0,
            glass: 0.0,
            rzar: 0.0,
            rpot: 0.0,
            allow: [0.0; 10],
            inet: 0.0,
            download: 0.0,
            upload: 0.0,
            soft: 0.0,
            worm: 0.0,
            hello: 0.0,
            programs: 0.0,
            topsecret: 0.0,
            bot: 0.0,
            avt: 0.0,
            uiz: 0.0,
            poli: 0.0,
            bot_next: 10.0,
            svoy: 0.0,
            soft_u: vec![0.0; 34],
            comp: 0,
            memo: 0,
            hard: 0,
            video: 0,
            modem: 0,
            cd: 0,
            printer: 0,
            windows: 0,
            kleys: 0.0,
            kleys_max: 0.0,
            cdsbor: 0.0,
            cdmb: 200.0,
            toch: 0.0,
        }
    }
}

impl Globals {
    pub fn initial() -> Globals {
        Globals { money: 0.0, iq: 85.0, golod: 0.0, work: 1, income: 0.0 }
    }
}

impl Game {
    // ---- Name tables ------------------------------------------------------------------------

    fn table(&self, base: u32, i: i32, max: i32) -> String {
        if (1..=max).contains(&i) { self.s(base + (i - 1) as u32) } else { String::new() }
    }

    pub fn button_label(&self, d: i32) -> String {
        self.w
            .gas
            .as_ref()
            .and_then(|g| g.button.get(d as usize).copied().flatten())
            .map(|i| self.s(i))
            .unwrap_or_default()
    }

    fn hw_name(&self, hw: Hw, i: i32) -> String {
        match hw {
            Hw::Comp => self.table(txt::CPU_NAMES, i, MAX_COMP),
            Hw::Memo => self.table(txt::MEMORY_NAMES, i, MAX_MEMO),
            Hw::Hard => {
                if (1..=MAX_HARD).contains(&i) {
                    ps(self.data.s(txt::MEGABYTES), &[string(get_drive(i))])
                } else {
                    String::new()
                }
            }
            Hw::Video => self.table(txt::VIDEO_NAMES, i, MAX_VIDEO),
            Hw::Modem => self.table(txt::MODEM_NAMES, i, MAX_MODEM),
            Hw::Cd => self.table(txt::CD_NAMES, i, MAX_CD),
            Hw::Printer => self.table(txt::PRINTER_NAMES, i, MAX_PRINTER),
            Hw::Windows => self.table(txt::WINDOWS_NAMES, i, MAX_WINDOWS),
        }
    }

    fn hw_level(&self, hw: Hw) -> i32 {
        let g = self.gas();
        match hw {
            Hw::Comp => g.comp,
            Hw::Memo => g.memo,
            Hw::Hard => g.hard,
            Hw::Video => g.video,
            Hw::Modem => g.modem,
            Hw::Cd => g.cd,
            Hw::Printer => g.printer,
            Hw::Windows => g.windows,
        }
    }

    fn hw_level_mut(&mut self, hw: Hw) -> &mut i32 {
        let g = self.gas_mut();
        match hw {
            Hw::Comp => &mut g.comp,
            Hw::Memo => &mut g.memo,
            Hw::Hard => &mut g.hard,
            Hw::Video => &mut g.video,
            Hw::Modem => &mut g.modem,
            Hw::Cd => &mut g.cd,
            Hw::Printer => &mut g.printer,
            Hw::Windows => &mut g.windows,
        }
    }

    fn hw_max(hw: Hw) -> i32 {
        match hw {
            Hw::Comp => MAX_COMP,
            Hw::Memo => MAX_MEMO,
            Hw::Hard => MAX_HARD,
            Hw::Video => MAX_VIDEO,
            Hw::Modem => MAX_MODEM,
            Hw::Cd => MAX_CD,
            Hw::Printer => MAX_PRINTER,
            Hw::Windows => MAX_WINDOWS,
        }
    }

    fn weekday(&self, day: f64) -> String {
        let i = match (day % 7.0) as i32 {
            1 => txt::WEEKDAY_MON,
            2 => txt::WEEKDAY_TUE,
            3 => txt::WEEKDAY_WED,
            4 => txt::WEEKDAY_THU,
            5 => txt::WEEKDAY_FRI,
            6 => txt::WEEKDAY_SAT,
            _ => txt::WEEKDAY_SUN,
        };
        self.s(i)
    }

    pub fn upd_income(&mut self) {
        let g = self.gas();
        let income = get_zp(self.w.g.work) - get_home_op(g.home) + g.kleys * 3.0 + g.cdsbor * 5.0 + g.toch * 10.0;
        self.w.g.income = income;
    }

    // ---- Clock ------------------------------------------------------------------------------

    pub(crate) fn gas_alarm0(&mut self) {
        self.gas_mut().time += 1.0;
        self.hour();
        if self.gas().time > 24.0 {
            let g = self.gas_mut();
            g.time = 1.0;
            g.day += 1.0;
            self.new_day();
            let g = self.gas_mut();
            if g.day % 7.0 == 1.0 {
                self.w.g.iq += (self.w.g.work - 7).max(0) as f64;
            }
            let g = self.gas_mut();
            if g.day > 28.0 {
                g.month += 1.0;
                g.day = 1.0;
                if g.month > 12.0 {
                    g.year += 1.0;
                    g.month = 1.0;
                }
            }
        }
    }

    fn hour(&mut self) {
        let time = self.gas().time;
        if time % 6.0 == 0.0 {
            self.w.g.golod += 1.0;
            if self.w.g.golod >= 100.0 {
                if self.w.g.money >= 2.0 {
                    self.w.g.money -= 2.0;
                    self.w.g.golod -= 2.0;
                    self.msg(txt::HUNGRY_BUY_SNACK);
                } else {
                    self.msg(txt::STARVED);
                    self.game_end();
                }
            }
        }
        if self.w.g.work == 4 && time % 12.0 == 0.0 {
            self.w.g.golod = (self.w.g.golod - 2.0).max(0.0);
        }
        if self.w.room == R_INTERNET {
            self.gas_mut().inet -= 1.0;
            if self.gas().inet == 0.0 {
                self.msg(txt::INET_TIME_OVER);
                self.goto_room(R_COMP);
            }
        }
    }

    fn new_day(&mut self) {
        const WORLD: f64 = 2000000000.0;
        let income = self.w.g.income;
        let g = self.gas_mut();
        g.metro_ch = (g.metro_ch + 3.0 - (g.metro_ch / 10.0).floor()).min(15.0);
        g.police_act = (g.police_act - 1.0 - g.glass).max(0.0);
        self.w.g.money += income;
        let g = self.gas_mut();
        if g.worm > 0.0 {
            g.bot += g.worm;
            g.worm += g.worm;
            g.bot = g.bot.min(WORLD);
            g.worm = g.worm.min(WORLD);
            let (svoy, worm, bot) = (g.svoy, g.worm, g.bot);
            if (truthy(choose(&[1.0, not(svoy), 0.0])) && worm > 5.0) || bot / WORLD > 0.1 {
                if truthy(choose(&[1.0, 0.0])) || bot < 10.0 {
                    self.gas_mut().worm = (worm / 3.0).floor();
                    self.msg(txt::WORMS_FOUGHT);
                } else if truthy(choose(&[not(svoy), 1.0])) || bot / WORLD > 0.1 {
                    self.gas_mut().worm = 0.0;
                    self.msg(txt::WORMS_PATCHED);
                    self.gas_mut().poli += 1.0;
                }
            }
            let g = self.gas();
            if g.bot >= g.bot_next {
                let bot_next = g.bot_next;
                let ap = (bot_next.sqrt() - 1.0).floor();
                self.gas_mut().avt += ap;
                let t = ps(self.data.s(txt::BOTNET_AUTHORITY), &[string(bot_next), string(ap)]);
                self.show_message(t);
                self.gas_mut().bot_next *= 10.0;
            }
        }
        self.game_save();
    }

    // ---- Jobs and housing -------------------------------------------------------------------

    fn hired(&mut self, work: i32, head: u32, tail: u32) {
        self.w.g.work = work;
        self.upd_income();
        let t = self.s(head) + &string(get_zp(work)) + self.data.s(tail);
        self.show_message(t);
    }

    fn get_work(&mut self, d: i32) {
        let iq = self.w.g.iq;
        if !truthy(self.gas().rabota) {
            return self.msg(txt::NO_WORK_PERMIT);
        }
        if d == 18 {
            return self.hired(2, txt::HIRED_JANITOR, txt::PER_DAY_DOT);
        }
        if !truthy(self.gas().phone) {
            return self.msg(txt::NEED_PHONE);
        }
        match d {
            19 => return self.hired(3, txt::HIRED_LABORER, txt::PER_DAY_DOT),
            20 if iq < 90.0 => return self.msg(txt::IQ_FOR_WAITER),
            20 => return self.hired(4, txt::HIRED_WAITER, txt::WAITER_TAIL),
            21 if iq < 95.0 => return self.msg(txt::IQ_FOR_FITTER),
            21 => return self.hired(5, txt::HIRED_FITTER, txt::PER_DAY_DOT),
            _ => {}
        }
        if !truthy(self.gas().propiska) {
            return self.msg(txt::NEED_REGISTRATION);
        }
        match d {
            22 if iq < 96.0 => return self.msg(txt::IQ_FOR_OPERATOR),
            22 => return self.hired(6, txt::HIRED_OPERATOR, txt::PER_DAY_DOT),
            23 if iq < 100.0 => return self.msg(txt::IQ_FOR_TECHNICIAN),
            23 => return self.hired(7, txt::HIRED_TECHNICIAN, txt::PER_DAY_DOT),
            _ => {}
        }
        if !truthy(self.gas().diplom) {
            return self.msg(txt::NEED_DIPLOMA);
        }
        match d {
            24 if iq < 105.0 => self.msg(txt::IQ_FOR_PROGRAMMER),
            24 => self.hired(8, txt::HIRED_PROGRAMMER, txt::PROGRAMMER_TAIL),
            25 if iq < 115.0 => self.msg(txt::IQ_FOR_LAB_HEAD),
            25 => self.hired(9, txt::HIRED_LAB_HEAD, txt::LAB_HEAD_TAIL),
            26 if iq < 130.0 => self.msg(txt::IQ_FOR_CTO),
            26 => self.hired(10, txt::HIRED_CTO, txt::CTO_TAIL),
            _ => {}
        }
    }

    fn get_home(&mut self, d: i32) {
        if self.w.g.work < 2 {
            return self.msg(txt::RENT_NEEDS_JOB);
        }
        let (home, watt, text) = match d {
            30 => (2, 0.5, txt::RENTED_BUNK),
            31 => (3, 2.0, txt::RENTED_ROOM),
            _ => return,
        };
        if self.gas().home == home {
            return;
        }
        if self.w.g.money < get_home_op(home) * 10.0 {
            return self.msg(txt::RENT_NO_DEPOSIT);
        }
        if get_zp(self.w.g.work) < get_home_op(home) {
            return self.msg(txt::RENT_LOW_SALARY);
        }
        self.gas_mut().home = home;
        self.w.g.money -= get_home_op(home) * 10.0;
        self.upd_income();
        self.gas_mut().watt = watt;
        let t = ps(self.data.s(text), &[string(get_home_op(home))]);
        self.show_message(t);
    }

    // ---- BAction: what every button does ----------------------------------------------------

    /// Buy food or a market item: pay, and count it towards "spent at the market".
    fn buy_food(&mut self, price: f64, food: f64) {
        if self.w.g.money >= price {
            self.w.g.money -= price;
            self.w.g.golod = (self.w.g.golod - food).max(0.0);
            self.gas_mut().rpot += price;
        }
    }

    /// The shared prologue of the tech-shop items: power, then a computer.
    fn shop_requires(&mut self, computer: bool) -> bool {
        if self.gas().watt == 0.0 {
            self.msg(txt::NO_ELECTRICITY);
            return false;
        }
        if computer && self.gas().comp == 0 {
            self.msg(txt::NO_COMPUTER);
            return false;
        }
        true
    }

    /// Buy the next level of a part; returns the new level when bought.
    fn buy_hw(&mut self, hw: Hw, k: i32) -> Option<i32> {
        let lvl = self.hw_level(hw);
        if lvl >= Self::hw_max(hw) {
            return None;
        }
        let cost = cost_com(lvl + 1, k);
        if self.w.g.money < cost {
            return None;
        }
        self.w.g.money -= cost;
        *self.hw_level_mut(hw) += 1;
        Some(lvl + 1)
    }

    fn bought(&mut self, prefix: u32, hw: Hw, lvl: i32) {
        let t = self.s(prefix) + &self.hw_name(hw, lvl);
        self.show_message(t);
    }

    fn downloader_mut(&mut self) -> Option<&mut Inst> {
        let i = self.first(Obj::Downloader)?;
        Some(&mut self.w.insts[i])
    }

    pub fn baction(&mut self, d: i32) {
        let money = self.w.g.money;
        match d {
            1 => self.goto_room(R_METRO),
            2 => self.goto_room(R_RINOK),
            7 => self.goto_room(R_GAME),
            3 => self.buy_food(2.0, 2.0),
            4 => self.buy_food(8.0, 10.0),
            5 => self.buy_food(11.0, 15.0),
            6 => self.buy_food(16.0, 22.0),
            8 => {
                if money >= 3.0 {
                    self.w.g.money -= 3.0;
                    self.gas_mut().tokens += 1.0;
                }
            }
            9 => {
                let g = self.gas_mut();
                if random(100.0) < g.metro_ch {
                    g.tokens += 1.0;
                    g.metro_ch = (g.metro_ch - 3.0).max(0.0);
                    self.msg(txt::TOKEN_GIVEN);
                } else if g.metro_nah > 100.0 {
                    g.tokens += 1.0;
                    g.metro_nah = 0.0;
                    self.msg(txt::TOKEN_PICKED_UP);
                } else {
                    g.metro_nah += g.metro_ch;
                    g.metro_ch = (g.metro_ch - 1.0).max(0.0);
                    self.msg(txt::TOKEN_REFUSED);
                }
            }
            10 | 80 => {
                let Some(l) = self.first(Obj::Listovki) else { return };
                if truthy(self.w.insts[l].go) {
                    return;
                }
                if d == 10 {
                    if truthy(self.gas().tokens) {
                        self.gas_mut().tokens -= 1.0;
                        self.w.insts[l].go = 1.0;
                        self.w.insts[l].find = 0.0;
                    } else {
                        self.msg(txt::TOKEN_REQUIRED);
                    }
                } else if self.gas().kleys < self.gas().kleys_max {
                    self.w.insts[l].go = 1.0;
                    self.w.insts[l].find = 1.0;
                } else {
                    self.msg(txt::NO_MORE_LEAFLETS);
                }
            }
            11 => {
                let Some(r) = self.first(Obj::Razgruzka) else { return };
                if truthy(self.w.insts[r].go) {
                    return;
                }
                if self.gas().rpot >= 25.0 {
                    let wd = self.gas().day % 7.0;
                    if wd == 0.0 || wd == 6.0 {
                        self.w.insts[r].go = 1.0;
                    } else {
                        self.msg(txt::WEEKENDS_ONLY);
                    }
                } else {
                    self.msg(txt::MARKET_UNKNOWN);
                }
            }
            13 => {
                if self.gas().pva == 0.0 {
                    if money >= 12.0 {
                        self.w.g.money -= 12.0;
                        let g = self.gas_mut();
                        g.pva = 1.0;
                        g.rpot += 12.0;
                        self.msg(txt::GLUE_BOUGHT);
                    }
                } else {
                    self.msg(txt::GLUE_OWNED);
                }
            }
            14 => {
                if self.gas().rabota == 0.0 {
                    if money >= 50.0 {
                        self.w.g.money -= 50.0;
                        let g = self.gas_mut();
                        g.rabota = 1.0;
                        g.allow[0] = 1.0;
                        self.msg(txt::PERMIT_BOUGHT);
                    }
                } else {
                    self.msg(txt::PERMIT_OWNED);
                }
            }
            15 => {
                if self.gas().propiska == 0.0 {
                    if money >= 500.0 {
                        self.w.g.money -= 500.0;
                        let g = self.gas_mut();
                        g.propiska = 1.0;
                        g.allow[3] = 1.0;
                        self.msg(txt::REGISTRATION_BOUGHT);
                    }
                } else {
                    self.msg(txt::REGISTRATION_OWNED);
                }
            }
            16 => {
                if self.gas().diplom == 0.0 {
                    if money >= 2000.0 {
                        self.w.g.money -= 2000.0;
                        self.gas_mut().diplom = 1.0;
                        self.msg(txt::DIPLOMA_BOUGHT);
                    }
                } else {
                    // The original shows the registration message here too.
                    self.msg(txt::REGISTRATION_OWNED);
                }
            }
            17 => {
                if self.gas().glass == 0.0 {
                    if money >= 10.0 {
                        self.w.g.money -= 10.0;
                        let g = self.gas_mut();
                        g.glass = 1.0;
                        g.rpot += 10.0;
                        self.msg(txt::GLASSES_BOUGHT);
                    }
                } else {
                    self.msg(txt::GLASSES_OWNED);
                }
            }
            12 => {
                if money >= 1.0 {
                    self.w.g.money -= 1.0;
                    self.goto_room(R_GAZETA);
                }
            }
            18..=29 => self.get_work(d),
            30..=39 => self.get_home(d),
            40 => self.goto_room(R_TEHNO),
            41 => {
                if !self.shop_requires(false) {
                    return;
                }
                if self.gas().phone == 0.0 && money >= 34.0 {
                    self.w.g.money -= 34.0;
                    self.gas_mut().phone = 1.0;
                    self.msg(txt::PHONE_BOUGHT);
                }
            }
            42 => {
                if self.gas().modem == 0 {
                    return self.msg(txt::NO_MODEM);
                }
                if money >= 30.0 {
                    self.w.g.money -= 30.0;
                    self.gas_mut().inet += 50.0;
                    self.msg(txt::INET_CARD_BOUGHT);
                }
            }
            45 => {
                if !self.shop_requires(false) {
                    return;
                }
                if self.gas().watt < 1.0 {
                    return self.msg(txt::TOO_LITTLE_POWER);
                }
                if let Some(l) = self.buy_hw(Hw::Comp, 5) {
                    self.bought(txt::BOUGHT_CPU, Hw::Comp, l);
                    self.gas_mut().allow[1] = 1.0;
                }
            }
            46 => {
                if self.shop_requires(true) {
                    if let Some(l) = self.buy_hw(Hw::Memo, 3) {
                        self.bought(txt::BOUGHT_MEMORY, Hw::Memo, l);
                    }
                }
            }
            47 => {
                if self.shop_requires(true) {
                    if let Some(l) = self.buy_hw(Hw::Hard, 2) {
                        self.bought(txt::BOUGHT_DISK, Hw::Hard, l);
                    }
                }
            }
            48 => {
                if self.shop_requires(true) {
                    if let Some(l) = self.buy_hw(Hw::Video, 4) {
                        self.bought(txt::BOUGHT_VIDEO, Hw::Video, l);
                    }
                }
            }
            49 => {
                if self.shop_requires(true) {
                    if let Some(l) = self.buy_hw(Hw::Modem, 3) {
                        self.bought(if l <= 6 { txt::BOUGHT_MODEM } else { txt::BOUGHT_ADSL }, Hw::Modem, l);
                        if l == 6 {
                            self.gas_mut().button[49] = Some(txt::LABEL_ADSL);
                        }
                        if l == 13 {
                            self.gas_mut().button[49] = Some(txt::LABEL_ETHERNET);
                        }
                    }
                }
            }
            50 => {
                if self.shop_requires(true) {
                    if let Some(l) = self.buy_hw(Hw::Cd, 2) {
                        if l <= 3 {
                            self.bought(txt::BOUGHT_CD, Hw::Cd, l);
                        } else {
                            self.msg(txt::BOUGHT_CD_RW);
                        }
                        if l == 3 {
                            self.gas_mut().button[50] = Some(txt::LABEL_CD_RW);
                        }
                    }
                }
            }
            51 => {
                if !self.shop_requires(true) {
                    return;
                }
                if self.gas().windows == 0 {
                    return self.msg(txt::PRINTER_NEEDS_OS);
                }
                if let Some(l) = self.buy_hw(Hw::Printer, 8) {
                    let addk = 1.0 + (l * l * 2) as f64;
                    let t = self.s(txt::BOUGHT_PREFIX)
                        + &self.hw_name(Hw::Printer, l)
                        + self.data.s(txt::PRINTER_MID)
                        + &string(addk)
                        + self.data.s(txt::PRINTER_TAIL);
                    self.show_message(t);
                    let g = self.gas_mut();
                    g.allow[2] = 1.0;
                    g.kleys_max += addk;
                }
            }
            52 => {
                if !self.shop_requires(true) {
                    return;
                }
                let g = self.gas();
                let (memo, hard, comp, windows, soft) = (g.memo, g.hard, g.comp, g.windows, g.soft);
                if memo == 0 {
                    return self.msg(txt::LOW_MEMORY);
                }
                if hard == 0 {
                    return self.msg(txt::LOW_DISK);
                }
                if windows == 3 && comp < 3 {
                    return self.msg(txt::WEAK_CPU);
                }
                if windows == 3 && memo < 3 {
                    return self.msg(txt::LOW_MEMORY);
                }
                let scost = (55 * (windows + 2) * (windows + 1)) as f64;
                if soft + scost > get_drive(hard) {
                    let t = ps(self.data.s(txt::LOW_DISK_FOR_OS), &[string(scost)]);
                    return self.show_message(t);
                }
                if let Some(l) = self.buy_hw(Hw::Windows, 2) {
                    self.gas_mut().soft += scost;
                    let t = self.s(txt::BOUGHT_PREFIX) + &self.hw_name(Hw::Windows, l) + self.data.s(txt::OS_BOUGHT_TAIL);
                    self.show_message(t);
                    self.w.g.iq += 1.0;
                }
            }
            53 => {
                if self.gas().video == 0 {
                    return self.msg(txt::NO_VIDEO_CARD);
                }
                if self.gas().windows == 0 {
                    return self.msg(txt::NO_OS);
                }
                self.goto_room(R_COMP);
            }
            57 | 95 => self.goto_room(R_COMP),
            55 => {
                if let Some(dl) = self.downloader_mut() {
                    dl.alarm[1] = 3;
                }
            }
            59 => {
                self.io_clear();
                if let Some(dl) = self.downloader_mut() {
                    dl.alarm[2] = 3;
                }
            }
            60..=70 | 72..=79 => {
                let Some(dl) = self.first(Obj::Downloader) else { return };
                if truthy(self.w.insts[dl].go) {
                    return;
                }
                let g = self.gas();
                let (soft, hard, video, memo) = (g.soft, g.hard, g.video, g.memo);
                if truthy(g.soft_u[(d - 60) as usize]) {
                    return self.msg(txt::ALREADY_INSTALLED);
                }
                if soft + get_mb(d) > get_drive(hard) {
                    return self.msg(txt::LOW_DISK);
                }
                if d <= 70 {
                    if (65..=67).contains(&d) && video == 1 {
                        return self.msg(txt::WEAK_VIDEO);
                    }
                    if (65..=67).contains(&d) && memo == 1 {
                        return self.msg(txt::LOW_MEMORY_GAME);
                    }
                    let iq = self.w.g.iq;
                    if d == 68 && iq < 100.0 {
                        return self.msg(txt::IQ_FOR_PROGRAM_100);
                    }
                    if d == 69 && iq < 110.0 {
                        return self.msg(txt::IQ_FOR_PROGRAM_110);
                    }
                    if d == 70 && iq < 120.0 {
                        return self.msg(txt::IQ_FOR_PROGRAM_120);
                    }
                } else {
                    if money < get_game_cost(d) {
                        return self.msg(txt::NO_MONEY_FOR_GAME);
                    }
                    self.w.g.money -= get_game_cost(d);
                }
                let name = self.button_label(d);
                let inst = &mut self.w.insts[dl];
                inst.go = 1.0;
                inst.mb = get_mb(d);
                inst.name = name;
                inst.soft = d - 60;
            }
            71 => {
                if self.w.g.iq < 90.0 {
                    return self.msg(txt::IQ_FOR_SITE_90);
                }
                if let Some(dl) = self.downloader_mut() {
                    dl.alarm[4] = 3;
                }
            }
            56 => {
                if self.w.g.iq < 110.0 {
                    return self.msg(txt::IQ_FOR_SITE_110);
                }
                if let Some(dl) = self.downloader_mut() {
                    dl.alarm[5] = 3;
                }
            }
            81 => {
                if money >= 200.0 {
                    if self.gas().toch < 10.0 {
                        self.gas_mut().toch += 1.0;
                        self.upd_income();
                        self.w.g.money -= 200.0;
                        self.msg(txt::STALL_BOUGHT);
                    } else {
                        self.msg(txt::NO_MORE_STALLS);
                    }
                }
            }
            82..=86 => self.program_button(d),
            88 => {
                if !self.save_exists() {
                    self.goto_room(R_GAME);
                } else {
                    let t = self.s(txt::NEW_GAME_QUESTION);
                    self.show_question(t, Cont::NewGame);
                }
            }
            89 => {
                if !self.save_exists() {
                    self.msg(txt::NO_SAVE_FILE);
                } else {
                    self.game_load();
                }
            }
            90 => {
                if self.gas().poli < self.gas().uiz {
                    return self.msg(txt::EXPLOIT_INFO_FRESH);
                }
                if money >= 500.0 {
                    self.w.g.money -= 500.0;
                    self.gas_mut().uiz += 1.0;
                    self.msg(txt::EXPLOIT_INFO_BOUGHT);
                }
            }
            91 => {
                if self.gas().avt <= 10.0 {
                    return self.msg(txt::LOW_AUTHORITY);
                }
                if truthy(self.gas().svoy) {
                    return self.msg(txt::INFO_ALREADY_KNOWN);
                }
                if money >= 2000.0 {
                    self.w.g.money -= 2000.0;
                    self.gas_mut().svoy = 1.0;
                    self.msg(txt::RARE_INFO_BOUGHT);
                }
            }
            92 => {
                if self.gas().avt <= 100.0 {
                    return self.msg(txt::LOW_AUTHORITY);
                }
                if truthy(self.gas().topsecret) {
                    return self.msg(txt::INFO_ALREADY_KNOWN);
                }
                if money >= 5000.0 {
                    self.w.g.money -= 5000.0;
                    let g = self.gas_mut();
                    g.topsecret = 1.0;
                    g.allow[4] = 1.0;
                    self.msg(txt::TOP_SECRET_BOUGHT);
                    let p = self.instance_create(401.0, 464.0, Obj::OButton);
                    self.w.insts[p].d = 93;
                    self.btn50(p);
                    self.ballow(p, 4);
                }
            }
            93 => {
                if self.gas().bot / 2000000000.0 < 0.1 {
                    return self.msg(txt::NEED_TEN_PERCENT);
                }
                self.msg(txt::PREPARE_UPLOAD);
                self.w.gas_destroyed = true;
                self.goto_room(R_ZAG);
                for b in self.all(Obj::OButton) {
                    self.w.insts[b].dead = true;
                }
            }
            _ => {}
        }
    }

    fn program_button(&mut self, d: i32) {
        let Some(pg) = self.first(Obj::Programgame) else { return };
        let (go, fin) = (truthy(self.w.insts[pg].go), truthy(self.w.insts[pg].fin));
        match d {
            82 => {
                if !go {
                    self.w.insts[pg].alarm[0] = 3;
                    self.w.insts[pg].typ = 0;
                }
            }
            83 => {
                if go {
                    self.w.insts[pg].alarm[1] = 3;
                }
            }
            84 => {
                if go && fin {
                    self.w.insts[pg].alarm[2] = 3;
                }
            }
            85 => {
                if !truthy(self.gas().soft_u[9]) {
                    let t = self.s(txt::NEED_BORLAND);
                    return self.show_message(t);
                }
                if !go {
                    self.w.insts[pg].alarm[0] = 3;
                    self.w.insts[pg].typ = 1;
                }
            }
            86 => {
                if !truthy(self.gas().soft_u[10]) {
                    return self.msg(txt::NEED_VISUAL_STUDIO);
                }
                if self.gas().uiz <= self.gas().poli {
                    return self.msg(txt::NEED_EXPLOIT_INFO);
                }
                if !go {
                    self.w.insts[pg].alarm[0] = 3;
                    self.w.insts[pg].typ = 2;
                }
            }
            _ => {}
        }
    }

    /// `doptext(d)`: the small caption under a button.
    pub fn doptext(&self, d: i32) -> String {
        let Some(g) = self.w.gas.as_ref() else { return String::new() };
        let p = |idx: u32, args: &[f64]| ps(self.data.s(idx), &args.iter().map(|&a| string(a)).collect::<Vec<_>>());
        let next = |hw: Hw, k: i32| {
            let lvl = self.hw_level(hw);
            if lvl == Self::hw_max(hw) {
                String::new()
            } else {
                self.hw_name(hw, lvl + 1) + self.data.s(txt::COMMA_FOR) + &string(cost_com(lvl + 1, k)) + self.data.s(txt::RUBLES_SUFFIX)
            }
        };
        match d {
            3 => self.s(txt::PRICE_2),
            4 => self.s(txt::PRICE_8),
            5 => self.s(txt::PRICE_11),
            6 => self.s(txt::PRICE_16),
            8 => self.s(txt::PRICE_TOKEN),
            9 => p(txt::CHANCE_PCT, &[g.metro_ch]),
            10 | 80 => {
                if g.police_act > 0.0 {
                    p(txt::POLICE_ATTENTION, &[g.police_act])
                } else {
                    self.s(txt::EMPTY)
                }
            }
            12 => self.s(txt::PRICE_1),
            13 => self.s(txt::PRICE_12),
            14 => self.s(txt::PRICE_50),
            15 => self.s(txt::PRICE_500),
            16 => self.s(txt::PRICE_2000),
            17 => self.s(txt::PRICE_10),
            18..=26 => p(txt::SALARY_PER_DAY, &[get_zp(d - 16)]),
            30 | 31 => {
                let op = get_home_op(d - 28);
                p(txt::RENT_PRICE, &[op, 10.0 * op])
            }
            41 => self.s(txt::PRICE_PHONE),
            42 => self.s(txt::PRICE_INET_CARD),
            45 => next(Hw::Comp, 5),
            46 => next(Hw::Memo, 3),
            47 => next(Hw::Hard, 2),
            48 => next(Hw::Video, 4),
            49 => next(Hw::Modem, 3),
            50 => next(Hw::Cd, 2),
            51 => next(Hw::Printer, 8),
            52 => next(Hw::Windows, 2),
            60..=70 => p(txt::MEGABYTES, &[get_mb(d)]),
            72..=79 => p(txt::MEGABYTES_AND_PRICE, &[get_mb(d), get_game_cost(d)]),
            81 => self.s(txt::PRICE_STALL),
            90 => self.s(txt::PRICE_EXPLOIT_INFO),
            91 => self.s(txt::PRICE_RARE_INFO),
            92 => self.s(txt::PRICE_TOP_SECRET),
            _ => String::new(),
        }
    }

    // ---- gas Draw: the HUD panels -----------------------------------------------------------

    /// The panel frame every HUD box shares; returns the text origin.
    fn panel(&self, cv: &mut Canvas, xx: f64, yy: f64, ww: f64, hh: f64, title: u32) -> (f64, f64) {
        cv.font = FNT_00;
        cv.color = C_LTGRAY;
        cv.rect(xx, yy, xx + ww, yy + hh, false);
        cv.color = C_BLACK;
        cv.rect(xx, yy, xx + ww, yy + hh, true);
        cv.rect(xx, yy, xx + ww, yy + 25.0, false);
        cv.color = C_WHITE;
        cv.halign = HAlign::Center;
        cv.text(&self.data, xx + ww / 2.0, yy + 5.0, self.data.s(title));
        cv.halign = HAlign::Left;
        cv.font = FNT_00;
        cv.color = C_BLACK;
        (xx + 5.0, yy + 10.0)
    }

    pub(crate) fn gas_draw(&self, cv: &mut Canvas) {
        let Some(g) = self.w.gas.as_ref() else { return };
        let gl = &self.w.g;
        let room = self.w.room;
        let d = &self.data;
        let p = |idx: u32, args: &[f64]| ps(d.s(idx), &args.iter().map(|&a| string(a)).collect::<Vec<_>>());

        if room == R_INTRO {
            cv.color = C_BLACK;
            cv.font = FNT_01;
            cv.text(d, 20.0, 15.0, d.s(txt::TITLE));
            cv.font = FNT_00;
            cv.text(d, 686.0, 575.0, d.s(txt::AUTHOR));
            return;
        }

        // Status.
        let ww = 200.0;
        let (xx, yy) = self.panel(cv, 5.0, 5.0, ww, 140.0, txt::HUD_STATUS);
        let row = |ch: f64| yy + 20.0 * ch;
        cv.text(d, xx, row(1.0), &p(txt::HUD_CAPITAL, &[gl.money]));
        cv.text(d, xx, row(2.0), &(self.s(txt::HUD_WORK) + &self.table(txt::WORK_NAMES, gl.work, 10)));
        cv.text(d, xx, row(3.0), &p(txt::HUD_INCOME, &[gl.income]));
        cv.text(d, xx, row(4.0), &p(txt::HUD_IQ, &[gl.iq]));
        cv.color = C_RED;
        cv.rect(xx, row(5.0), xx + round((ww - xx) * gl.golod / 100.0), row(5.0) + 20.0, false);
        cv.color = C_BLACK;
        cv.halign = HAlign::Center;
        cv.text(d, xx + ww / 2.0 - 5.0, row(5.0) + 2.0, d.s(txt::HUD_HUNGER));
        cv.halign = HAlign::Left;
        cv.rect(xx, row(5.0), ww, row(5.0) + 20.0, true);

        if room == R_GAME {
            let (xx, yy) = self.panel(cv, 5.0, 160.0, 230.0, 100.0, txt::HUD_DOCUMENTS);
            cv.color = C_RED;
            let mut ch = 1.0;
            if !truthy(g.rabota) {
                cv.text(d, xx, yy + 20.0 * ch, d.s(txt::HUD_NO_PERMIT));
                ch += 1.0;
            }
            if !truthy(g.propiska) {
                cv.text(d, xx, yy + 20.0 * ch, d.s(txt::HUD_NO_REGISTRATION));
            }
        }

        // Date (with the weekday at the market).
        let drwd = room == R_RINOK;
        let hh = 50.0 + if drwd { 20.0 } else { 0.0 };
        let (xx, yy) = self.panel(cv, 630.0, 5.0, 157.0, hh, txt::HUD_DATE);
        let date = p(txt::HUD_DATE_FORMAT, &[g.day, g.month, g.year, floor_div(g.time, 10.0), g.time % 10.0]);
        cv.text(d, xx, yy + 20.0, &date);
        if drwd {
            let wd = g.day % 7.0;
            if wd == 6.0 || wd == 0.0 {
                cv.color = C_MAROON;
            }
            cv.text(d, xx + 5.0, yy + 40.0, &self.weekday(g.day));
        }

        if room == R_METRO {
            let (xx, yy) = self.panel(cv, 630.0, 70.0, 155.0, 50.0, txt::HUD_METRO);
            cv.text(d, xx, yy + 20.0, &(self.s(txt::HUD_TOKENS) + &string(g.tokens)));
        }

        if room == R_RINOK {
            let (xx, yy) = self.panel(cv, 630.0, 90.0, 157.0, 70.0, txt::HUD_MARKET);
            cv.text(d, xx, yy + 20.0, &(self.s(txt::HUD_EARNED) + &string(g.rzar)));
            cv.text(d, xx, yy + 40.0, &(self.s(txt::HUD_SPENT) + &string(g.rpot)));
        }

        if room == R_COMP {
            let (xx, yy) = self.panel(cv, 600.0, 70.0, 187.0, 180.0, txt::HUD_MY_COMPUTER);
            let mut rows = vec![
                self.s(txt::HUD_CPU) + &self.hw_name(Hw::Comp, g.comp),
                self.s(txt::HUD_MEMORY) + &self.hw_name(Hw::Memo, g.memo),
                ps(d.s(txt::HUD_DISK), &[string(g.soft), self.hw_name(Hw::Hard, g.hard)]),
                self.s(txt::HUD_VIDEO) + &self.hw_name(Hw::Video, g.video),
            ];
            if g.modem != 0 {
                rows.push(self.s(txt::HUD_MODEM) + &self.hw_name(Hw::Modem, g.modem));
            }
            if g.cd != 0 {
                rows.push(self.s(txt::HUD_CD) + &self.hw_name(Hw::Cd, g.cd));
            }
            if g.printer != 0 {
                rows.push(self.s(txt::HUD_PRINTER) + &self.hw_name(Hw::Printer, g.printer));
            }
            for (k, r) in rows.iter().enumerate() {
                cv.text(d, xx, yy + 20.0 * (k + 1) as f64, r);
            }
        }

        if room == R_GAME {
            let (xx, yy) = self.panel(cv, 5.0, 275.0, 230.0, 130.0, txt::HUD_HOUSING);
            let mut ch = 1.0;
            let mut line = |cv: &mut Canvas, s: &str| {
                cv.text(d, xx, yy + 20.0 * ch, s);
                ch += 1.0;
            };
            line(cv, &(self.s(txt::HUD_TYPE) + &self.table(txt::HOME_NAMES, g.home, 3)));
            if g.watt == 0.0 {
                cv.color = C_RED;
                line(cv, d.s(txt::HUD_NO_POWER));
            } else {
                line(cv, &p(txt::HUD_POWER, &[g.watt]));
            }
            if truthy(g.phone) {
                line(cv, d.s(txt::HUD_HAS_PHONE));
            }
            if g.comp != 0 {
                line(cv, &(self.s(txt::HUD_COMPUTER) + &self.hw_name(Hw::Comp, g.comp)));
            }
            if g.printer != 0 {
                line(cv, &(self.s(txt::HUD_PRINTER) + &self.hw_name(Hw::Printer, g.printer)));
            }
        }

        if room == R_INTERNET {
            let (xx, yy) = self.panel(cv, 5.0, 160.0, 230.0, 110.0, txt::HUD_INTERNET);
            cv.text(d, xx, yy + 20.0, &p(txt::HUD_HOURS_LEFT, &[g.inet]));
            cv.text(d, xx, yy + 40.0, &(self.s(txt::HUD_SPEED) + &self.hw_name(Hw::Modem, g.modem)));
            let site = self.first(Obj::Downloader).map_or(0, |i| self.w.insts[i].site);
            if site == 1 {
                cv.text(d, xx, yy + 60.0, d.s(txt::HUD_SITE_TRACKER));
            } else if site == 3 {
                cv.text(d, xx, yy + 60.0, d.s(txt::HUD_SITE_HACKERS));
                cv.text(d, xx, yy + 80.0, &(self.s(txt::HUD_AUTHORITY) + &string(g.avt)));
            }
        }

        if room == R_GAME && (g.kleys > 0.0 || g.cdsbor > 0.0 || g.toch > 0.0) {
            let (xx, yy) = self.panel(cv, 5.0, 415.0, 230.0, 120.0, txt::HUD_BUSINESS);
            let mut ch = 1.0;
            if truthy(g.kleys) {
                cv.text(d, xx, yy + 20.0 * ch, &p(txt::HUD_POSTERS, &[g.kleys, g.kleys_max]));
                ch += 1.0;
            }
            if truthy(g.cdsbor) {
                cv.text(d, xx, yy + 20.0 * ch, &p(txt::HUD_CD_COLLECTIONS, &[g.cdsbor]));
                ch += 1.0;
            }
            if truthy(g.toch) {
                cv.text(d, xx, yy + 20.0 * ch, &p(txt::HUD_STALLS, &[g.toch]));
            }
        }

        if (room == R_GAME || room == R_PASCAL) && (g.worm > 0.0 || g.bot > 0.0) {
            let (xx, yy) = self.panel(cv, 225.0, 5.0, 300.0, 110.0, txt::HUD_BOTNET);
            cv.text(d, xx, yy + 20.0, &p(txt::HUD_WORMS, &[g.worm]));
            cv.text(d, xx, yy + 40.0, &p(txt::HUD_INFECTED, &[g.bot]));
            cv.text(d, xx, yy + 60.0, d.s(txt::HUD_WORLD_PCS));
            cv.text(d, xx, yy + 80.0, &p(txt::HUD_INFECTED_PCT, &[100.0 * g.bot / 2000000000.0]));
        }
    }
}
