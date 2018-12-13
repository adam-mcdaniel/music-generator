#![allow(dead_code)]


use song::*;
use self::Number::*;
use self::Accidental::*;
use self::Tone::*;

#[derive(Debug, Copy, Clone)]
pub enum Number {
    I,
    II,
    III,
    IV,
    V,
    VI,
    VII
}

#[derive(Debug, Copy, Clone)]
pub enum Tone {
    Major,
    Minor
}

#[derive(Debug, Copy, Clone)]
pub enum Accidental {
    Natural,
    Sharp,
    Flat
}

#[derive(Debug, Copy, Clone)]
pub struct Chord {
    number: Number,
    tone: Tone,
    accidental: Accidental
}


impl Chord {
    pub fn new(number: Number, tone: Tone, accidental: Accidental) -> Chord {
        Self {
            number,
            tone,
            accidental
        }
    }

    pub fn show(&self, key: Key) {
        println!("{:?}{}{}",
            to_key(self.number, key),
            match self.accidental {
                Sharp => "#",
                Flat => "b",
                Natural => ""
            },
            match self.tone {
                Major => "",
                Minor => "m",
            }
        )
    }
}

// impl std::fmt::Display for Chord {
//     fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
//         write!(f, "{}{:?} {:?}",
//             match self.accidental {
//                 Sharp => "#",
//                 Flat => "b",
//                 Natural => ""
//             },
//             self.number,
//             self.tone
//         )
//     }
// }

pub fn to_number(n: i32) -> Number {
    match n % 7 {
        0 => I,
        1 => II,
        2 => III,
        3 => IV,
        4 => V,
        5 => VI,
        6 => VII,
        _ => I
    }
}

pub fn to_int(n: Number) -> i32 {
    match n {
        I => 0,
        II => 1,
        III => 2,
        IV => 3,
        V => 4,
        VI => 5,
        VII => 6,
    }
}