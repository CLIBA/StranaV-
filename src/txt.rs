//! Indices into the game's STRG table. The port never embeds the game's own text;
//! every message and label is read from the user's `data.win` through these ids.
//! Comments describe what each string is about.

// Hourly / daily events.
pub const HUNGRY_BUY_SNACK: u32 = 11;
pub const STARVED: u32 = 13;
pub const INET_TIME_OVER: u32 = 19;
pub const WORMS_FOUGHT: u32 = 30;
pub const WORMS_PATCHED: u32 = 31;
pub const BOTNET_AUTHORITY: u32 = 37; // "& computers ... +& authority"
pub const WEEKDAY_MON: u32 = 42;
pub const WEEKDAY_TUE: u32 = 44;
pub const WEEKDAY_WED: u32 = 45;
pub const WEEKDAY_THU: u32 = 46;
pub const WEEKDAY_FRI: u32 = 47;
pub const WEEKDAY_SAT: u32 = 48;
pub const WEEKDAY_SUN: u32 = 49;

// Metro / market actions.
pub const TOKEN_GIVEN: u32 = 55;
pub const TOKEN_PICKED_UP: u32 = 57;
pub const TOKEN_REFUSED: u32 = 58;
pub const TOKEN_REQUIRED: u32 = 61;
pub const NO_MORE_LEAFLETS: u32 = 64;
pub const WEEKENDS_ONLY: u32 = 66;
pub const MARKET_UNKNOWN: u32 = 67;
pub const GLUE_BOUGHT: u32 = 69;
pub const GLUE_OWNED: u32 = 70;
pub const PERMIT_BOUGHT: u32 = 73;
pub const PERMIT_OWNED: u32 = 74;
pub const REGISTRATION_BOUGHT: u32 = 76;
pub const REGISTRATION_OWNED: u32 = 77;
pub const DIPLOMA_BOUGHT: u32 = 79;
pub const GLASSES_BOUGHT: u32 = 80;
pub const GLASSES_OWNED: u32 = 81;

// Tech shop.
pub const NO_ELECTRICITY: u32 = 85;
pub const PHONE_BOUGHT: u32 = 87;
pub const NO_MODEM: u32 = 89;
pub const INET_CARD_BOUGHT: u32 = 90;
pub const TOO_LITTLE_POWER: u32 = 91;
pub const BOUGHT_CPU: u32 = 96;
pub const NO_COMPUTER: u32 = 97;
pub const BOUGHT_MEMORY: u32 = 100;
pub const BOUGHT_DISK: u32 = 103;
pub const BOUGHT_VIDEO: u32 = 106;
pub const BOUGHT_MODEM: u32 = 108;
pub const BOUGHT_ADSL: u32 = 109;
pub const LABEL_ADSL: u32 = 110;
pub const LABEL_ETHERNET: u32 = 112;
pub const BOUGHT_CD: u32 = 115;
pub const BOUGHT_CD_RW: u32 = 116;
pub const LABEL_CD_RW: u32 = 117;
pub const PRINTER_NEEDS_OS: u32 = 119;
pub const BOUGHT_PREFIX: u32 = 124;
pub const PRINTER_MID: u32 = 125;
pub const PRINTER_TAIL: u32 = 126;
pub const LOW_MEMORY: u32 = 127;
pub const LOW_DISK: u32 = 128;
pub const WEAK_CPU: u32 = 129;
pub const LOW_DISK_FOR_OS: u32 = 133;
pub const OS_BOUGHT_TAIL: u32 = 135;

// Computer / internet.
pub const NO_VIDEO_CARD: u32 = 136;
pub const NO_OS: u32 = 137;
pub const ALREADY_INSTALLED: u32 = 141;
pub const WEAK_VIDEO: u32 = 143;
pub const LOW_MEMORY_GAME: u32 = 144;
pub const IQ_FOR_PROGRAM_100: u32 = 145;
pub const IQ_FOR_PROGRAM_110: u32 = 146;
pub const IQ_FOR_PROGRAM_120: u32 = 147;
pub const IQ_FOR_SITE_90: u32 = 150;
pub const IQ_FOR_SITE_110: u32 = 151;
pub const NO_MONEY_FOR_GAME: u32 = 153;
pub const STALL_BOUGHT: u32 = 156;
pub const NO_MORE_STALLS: u32 = 157;
pub const NEED_BORLAND: u32 = 160;
pub const NEED_VISUAL_STUDIO: u32 = 161;
pub const NEED_EXPLOIT_INFO: u32 = 163;
pub const NEW_GAME_QUESTION: u32 = 165;
pub const NO_SAVE_FILE: u32 = 167;
pub const EXPLOIT_INFO_FRESH: u32 = 169;
pub const EXPLOIT_INFO_BOUGHT: u32 = 170;
pub const LOW_AUTHORITY: u32 = 171;
pub const INFO_ALREADY_KNOWN: u32 = 172;
pub const RARE_INFO_BOUGHT: u32 = 173;
pub const TOP_SECRET_BOUGHT: u32 = 175;
pub const NEED_TEN_PERCENT: u32 = 180;
pub const PREPARE_UPLOAD: u32 = 181;

// Button sub-captions (doptext).
pub const PRICE_2: u32 = 183;
pub const PRICE_8: u32 = 184;
pub const PRICE_11: u32 = 185;
pub const PRICE_16: u32 = 186;
pub const PRICE_TOKEN: u32 = 187;
pub const CHANCE_PCT: u32 = 188;
pub const POLICE_ATTENTION: u32 = 189;
pub const EMPTY: u32 = 190;
pub const PRICE_1: u32 = 191;
pub const PRICE_12: u32 = 192;
pub const PRICE_50: u32 = 193;
pub const PRICE_500: u32 = 194;
pub const PRICE_2000: u32 = 195;
pub const PRICE_10: u32 = 196;
pub const SALARY_PER_DAY: u32 = 198;
pub const RENT_PRICE: u32 = 200;
pub const PRICE_PHONE: u32 = 201;
pub const PRICE_INET_CARD: u32 = 202;
pub const COMMA_FOR: u32 = 203;
pub const RUBLES_SUFFIX: u32 = 204;
pub const MEGABYTES: u32 = 205;
pub const MEGABYTES_AND_PRICE: u32 = 206;
pub const PRICE_STALL: u32 = 207;
pub const PRICE_EXPLOIT_INFO: u32 = 208;
pub const PRICE_RARE_INFO: u32 = 209;
pub const PRICE_TOP_SECRET: u32 = 210;

// Jobs.
pub const NO_WORK_PERMIT: u32 = 214;
pub const HIRED_JANITOR: u32 = 215;
pub const PER_DAY_DOT: u32 = 216;
pub const NEED_PHONE: u32 = 217;
pub const HIRED_LABORER: u32 = 218;
pub const IQ_FOR_WAITER: u32 = 219;
pub const HIRED_WAITER: u32 = 220;
pub const WAITER_TAIL: u32 = 221;
pub const IQ_FOR_FITTER: u32 = 222;
pub const HIRED_FITTER: u32 = 223;
pub const NEED_REGISTRATION: u32 = 224;
pub const IQ_FOR_OPERATOR: u32 = 225;
pub const HIRED_OPERATOR: u32 = 226;
pub const IQ_FOR_TECHNICIAN: u32 = 227;
pub const HIRED_TECHNICIAN: u32 = 228;
pub const NEED_DIPLOMA: u32 = 229;
pub const IQ_FOR_PROGRAMMER: u32 = 230;
pub const HIRED_PROGRAMMER: u32 = 231;
pub const PROGRAMMER_TAIL: u32 = 232;
pub const IQ_FOR_LAB_HEAD: u32 = 233;
pub const HIRED_LAB_HEAD: u32 = 234;
pub const LAB_HEAD_TAIL: u32 = 235;
pub const IQ_FOR_CTO: u32 = 236;
pub const HIRED_CTO: u32 = 237;
pub const CTO_TAIL: u32 = 238;

// Housing.
pub const RENT_NEEDS_JOB: u32 = 239;
pub const RENT_NO_DEPOSIT: u32 = 241;
pub const RENT_LOW_SALARY: u32 = 242;
pub const RENTED_BUNK: u32 = 243;
pub const RENTED_ROOM: u32 = 244;

// Name tables (first entry of each run).
pub const WORK_NAMES: u32 = 248; // _work[1..=10]
pub const HOME_NAMES: u32 = 336; // _home[1..=3]
pub const CPU_NAMES: u32 = 347; // _comp[1..=3]
pub const MEMORY_NAMES: u32 = 350; // _memo[1..=3]
pub const VIDEO_NAMES: u32 = 353; // _video[1..=3]
pub const MODEM_NAMES: u32 = 356; // _modem[1..=18]
pub const CD_NAMES: u32 = 374; // _cd[1..=4]
pub const PRINTER_NAMES: u32 = 378; // _printer[1..=3]
pub const WINDOWS_NAMES: u32 = 381; // _windows[1..=4]

// HUD.
pub const TITLE: u32 = 393;
pub const AUTHOR: u32 = 395;
pub const HUD_STATUS: u32 = 402;
pub const HUD_CAPITAL: u32 = 405;
pub const HUD_WORK: u32 = 406;
pub const HUD_INCOME: u32 = 407;
pub const HUD_IQ: u32 = 408;
pub const HUD_HUNGER: u32 = 410;
pub const HUD_DOCUMENTS: u32 = 411;
pub const HUD_NO_PERMIT: u32 = 412;
pub const HUD_NO_REGISTRATION: u32 = 413;
pub const HUD_DATE: u32 = 415;
pub const HUD_DATE_FORMAT: u32 = 416;
pub const HUD_METRO: u32 = 258;
pub const HUD_TOKENS: u32 = 418;
pub const HUD_MARKET: u32 = 259;
pub const HUD_EARNED: u32 = 419;
pub const HUD_SPENT: u32 = 420;
pub const HUD_MY_COMPUTER: u32 = 421;
pub const HUD_CPU: u32 = 422;
pub const HUD_MEMORY: u32 = 423;
pub const HUD_DISK: u32 = 424;
pub const HUD_VIDEO: u32 = 425;
pub const HUD_MODEM: u32 = 426;
pub const HUD_CD: u32 = 427;
pub const HUD_PRINTER: u32 = 428;
pub const HUD_HOUSING: u32 = 429;
pub const HUD_TYPE: u32 = 430;
pub const HUD_NO_POWER: u32 = 431;
pub const HUD_POWER: u32 = 432;
pub const HUD_HAS_PHONE: u32 = 433;
pub const HUD_COMPUTER: u32 = 434;
pub const HUD_INTERNET: u32 = 435;
pub const HUD_HOURS_LEFT: u32 = 436;
pub const HUD_SPEED: u32 = 437;
pub const HUD_SITE_TRACKER: u32 = 439;
pub const HUD_SITE_HACKERS: u32 = 440;
pub const HUD_AUTHORITY: u32 = 441;
pub const HUD_BUSINESS: u32 = 442;
pub const HUD_POSTERS: u32 = 443;
pub const HUD_CD_COLLECTIONS: u32 = 444;
pub const HUD_STALLS: u32 = 445;
pub const HUD_BOTNET: u32 = 446;
pub const HUD_WORMS: u32 = 447;
pub const HUD_INFECTED: u32 = 448;
pub const HUD_WORLD_PCS: u32 = 449;
pub const HUD_INFECTED_PCT: u32 = 450;

// Mini-games.
pub const CAUGHT_BY_POLICE: u32 = 466;
pub const POSTER_HIRED: u32 = 467;
pub const EARNED: u32 = 470;
pub const HIRING_PCT: u32 = 472;
pub const POSTING_PCT: u32 = 473;
pub const UNLOADING_PCT: u32 = 474;
pub const DOWNLOADED: u32 = 476;
pub const DOWNLOADING: u32 = 477;
pub const CONNECT_FAILED: u32 = 482;
pub const NO_CD_DRIVE: u32 = 483;
pub const NO_CD_WRITER: u32 = 484;
pub const CD_NEEDS_SOFT: u32 = 485;
pub const CD_DISTRIBUTED: u32 = 486;
pub const NO_BUGS: u32 = 489;
pub const BUG_FIXED: u32 = 490;
pub const MARK_NOT_BUG: u32 = 491;
pub const ERROR: u32 = 492;
pub const NOT_WRITTEN: u32 = 493;
pub const BUG_CRASH: u32 = 494;
pub const PROGRAM_RUNS: u32 = 495;
pub const HELLO_IQ: u32 = 497;
pub const HELLO_NO_IQ: u32 = 498;
pub const SOLD_IQ: u32 = 499;
pub const SOLD_NO_IQ: u32 = 500;
pub const PROGRAM_DISK: u32 = 502;
pub const WORM_WRITTEN: u32 = 503;
pub const WORM_SPREAD: u32 = 505;
pub const WORM_COPIES: u32 = 506;
pub const MARK_SET: u32 = 515;
pub const MIND_DESTROYED: u32 = 554;
pub const ENERGY_PCT: u32 = 556;
pub const NETWORKS_CAPTURED: u32 = 557;
pub const THE_END: u32 = 558;

/// `gas._button[d]` caption for an action id, as initialised in `gas` Create.
pub fn button_label(d: i32) -> Option<u32> {
    let i = match d {
        1..=26 => 257 + d,
        30 => 284,
        31 => 285,
        40 => 286,
        41 => 287,
        42 => 288,
        45..=52 => 244 + d,
        53 => 289,
        55..=57 => 242 + d,
        59 => 300,
        60..=86 => 241 + d,
        88 => 328,
        89 => 329,
        90..=93 => 240 + d,
        95 => 334,
        _ => return None,
    };
    Some(i as u32)
}

/// Sanity check that the loaded string table is the one this port was written against.
pub const PROBES: &[(u32, &str)] = &[
    (9, "Golod"),
    (10, "Money"),
    (50, "IQ"),
    (155, "UpdIncome"),
    (240, "home"),
    (247, "_work"),
    (335, "_home"),
    (346, "programs"),
    (385, "cdmb"),
    (394, "draw_text"),
    (417, "weekday"),
    (465, "lose"),
    (468, "prize"),
    (501, "dsoft"),
    (504, "kop"),
    (559, "drop"),
    (564, "StranaV"),
];
