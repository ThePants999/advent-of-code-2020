use regex::Regex;
use std::str::FromStr;
use std::fmt::{self, Display};

pub fn day4(input_lines: &[String]) -> (u64, u64) {
    let passports = parse_passports(input_lines);
    for passport in passports.iter().filter(|passport| passport.all_fields_valid()) {
        println!("{}", passport);
    }
    let part1 = passports.iter().filter(|passport| passport.all_fields_present()).count() as u64;
    let part2 = passports.iter().filter(|passport| passport.all_fields_valid()).count() as u64;
    (part1,part2)
}

fn parse_passports(input_lines: &[String]) -> Vec<Passport> {
    let mut passports: Vec<Passport> = Vec::new();
    let mut passport = Passport::new();
    for line in input_lines {
        if line.is_empty() {
            passports.push(passport);
            passport = Passport::new();
        } else {
            passport.parse_line(line);
        }
    }
    passports.push(passport);
    passports
}

enum FieldState<T> {
    Valid(T),
    Invalid,
    Missing,
}

impl<T> FieldState<T> {
    fn is_present(&self) -> bool {
        !matches!(self, Self::Missing)
    }

    fn is_valid(&self) -> bool {
        matches!(self, Self::Valid(_))
    }
}

impl<T: Display> Display for FieldState<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Valid(field) => write!(f, "{}", field),
            Self::Invalid => write!(f, "**INVALID**"),
            Self::Missing => write!(f, "**MISSING**"),
        }
    }
}

enum Height {
    Centimetres(u32),
    Inches(u32),
}

impl Display for Height {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Centimetres(cm) => write!(f, "{}cm", cm),
            Self::Inches(inches) => write!(f, "{}in", inches),
        }
    }
}

struct Colour {
    r: u8,
    g: u8,
    b: u8,
}

impl Display for Colour {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "#{}", hex::encode_upper(vec![self.r, self.g, self.b]))
    }
}

#[derive(EnumString,Display)]
enum EyeColour {
    #[strum(serialize = "amb")]
    Amber,

    #[strum(serialize = "blu")]
    Blue,
    
    #[strum(serialize = "brn")]
    Brown,

    #[strum(serialize = "gry")]
    Grey,

    #[strum(serialize = "grn")]
    Green,

    #[strum(serialize = "hzl")]
    Hazel,

    #[strum(serialize = "oth")]
    Other,
}

struct Passport {
    raw: String,
    byr: FieldState<u32>,
    iyr: FieldState<u32>,
    eyr: FieldState<u32>,
    hgt: FieldState<Height>,
    hcl: FieldState<Colour>,
    ecl: FieldState<EyeColour>,
    pid: FieldState<String>,
}

impl Display for Passport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}Birth year: {}\nIssue year: {}\nExpiration year: {}\nHeight: {}\nHair colour: {}\nEye colour: {}\nPassport ID: {}\n---------------", self.raw, self.byr, self.iyr, self.eyr, self.hgt, self.hcl, self.ecl, self.pid)
    }
}

impl Passport {
    fn new() -> Self {
        Passport {
            raw: "".to_string(),
            byr: FieldState::Missing,
            iyr: FieldState::Missing,
            eyr: FieldState::Missing,
            hgt: FieldState::Missing,
            hcl: FieldState::Missing,
            ecl: FieldState::Missing,
            pid: FieldState::Missing,
        }
    }

    fn parse_line(&mut self, input: &str) {
        self.raw.push_str(input);
        self.raw.push('\n');
        lazy_static! {
            static ref RE: Regex = Regex::new(r"([a-z]{3}):([^ ]+)").unwrap();
        }
        for field in RE.captures_iter(input) {
            match &field[1] {
                "byr" => self.set_byr(&field[2]),
                "iyr" => self.set_iyr(&field[2]),
                "eyr" => self.set_eyr(&field[2]),
                "hgt" => self.set_hgt(&field[2]),
                "hcl" => self.set_hcl(&field[2]),
                "ecl" => self.set_ecl(&field[2]),
                "pid" => self.set_pid(&field[2]),
                _ => (),
            }            
        }
    }

    fn set_byr(&mut self, val: &str) {
        self.byr = FieldState::Invalid;
        let num = val.parse::<u32>();
        if let Ok(byr) = num {
            if byr >= 1920 && byr <= 2002 {
                self.byr = FieldState::Valid(byr);
            }
        }
    }

    fn set_iyr(&mut self, val: &str) {
        self.iyr = FieldState::Invalid;
        let num = val.parse::<u32>();
        if let Ok(iyr) = num {
            if iyr >= 2010 && iyr <= 2020 {
                self.iyr = FieldState::Valid(iyr)
            }
        }
    }

    fn set_eyr(&mut self, val: &str) {
        self.eyr = FieldState::Invalid;
        let num = val.parse::<u32>();
        if let Ok(eyr) = num {
            if eyr >= 2020 && eyr <= 2030 {
                self.eyr = FieldState::Valid(eyr)
            }
        }
    }

    fn set_hgt(&mut self, val: &str) {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(\d{2,3})(\D{2})$").unwrap();
        }
        self.hgt = FieldState::Invalid;
        if let Some(caps) = RE.captures(val) {
            let num = caps.get(1).unwrap().as_str().parse::<u32>().unwrap(); // regex already validated number length
            match caps.get(2).unwrap().as_str() {
                "cm" => {
                    if num >= 150 && num <= 193 { self.hgt = FieldState::Valid(Height::Centimetres(num)); }
                },
                "in" => {
                    if num >= 59 && num <= 76 { self.hgt = FieldState::Valid(Height::Inches(num)); }
                }
                _ => ()
            }
        }
    }

    fn set_hcl(&mut self, val: &str) {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^#([0-9a-f]{6})$").unwrap();
        }
        match RE.captures(val) {
            Some(caps) => {
                let bytes = hex::decode(caps.get(1).unwrap().as_str()).unwrap(); // regex already validated as hex string
                self.hcl = FieldState::Valid(Colour { r: bytes[0], g: bytes[1], b: bytes[2] });    
            },
            None => self.hcl = FieldState::Invalid,
        }
    }

    fn set_ecl(&mut self, val: &str) {
        match EyeColour::from_str(val) {
            Ok(colour) => self.ecl = FieldState::Valid(colour),
            Err(_) => self.ecl = FieldState::Invalid,
        }
    }

    fn set_pid(&mut self, val: &str) {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"^(\d{9})$").unwrap();
        }
        match RE.captures(val) {
            Some(caps) => self.pid = FieldState::Valid(caps.get(1).unwrap().as_str().to_string()),
            None => self.pid = FieldState::Invalid,
        }
    }

    fn all_fields_present(&self) -> bool {
        self.byr.is_present() &&
        self.iyr.is_present() &&
        self.eyr.is_present() &&
        self.hgt.is_present() &&
        self.hcl.is_present() &&
        self.ecl.is_present() &&
        self.pid.is_present()
    }

    fn all_fields_valid(&self) -> bool {
        self.byr.is_valid() &&
        self.iyr.is_valid() &&
        self.eyr.is_valid() &&
        self.hgt.is_valid() &&
        self.hcl.is_valid() &&
        self.ecl.is_valid() &&
        self.pid.is_valid()
    }
}