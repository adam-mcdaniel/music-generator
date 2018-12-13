#![allow(dead_code)]

extern crate rand;

use song::rand::seq::SliceRandom;
use chord::*;
use chord::Number::*;
use chord::Tone::*;
use chord::Accidental::*;

use song::Key::*;

#[derive(Debug, Copy, Clone)]
pub enum Key {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
}

pub fn key_to_int(key: Key) -> i32 {
    match key {
        A => 0,
        B => 1,
        C => 2,
        D => 3,
        E => 4,
        F => 5,
        G => 6,
    }
}

pub fn int_to_key(n: i32) -> Key {
    match n % 7 {
        0 => A,
        1 => B,
        2 => C,
        3 => D,
        4 => E,
        5 => F,
        6 => G,
        _ => A
    }
}

pub fn to_key(n: Number, key: Key) -> Key {
    int_to_key(to_int(n) + key_to_int(key))
}


pub type Progression = Vec<Chord>;

#[derive(Debug)]
pub struct Song {
    key: Key,
    progression: Progression,
}


impl Song {
    pub fn new(key: Key, choruses: i32, verses: i32) -> Self {
        return Self {
            key,
            progression: Song::generate_progression(choruses, verses),
        }
    }

    pub fn generate_verse() -> Progression {
        vec![
            vec![
                Chord::new(I, Major, Natural),
                Chord::new(IV, Major, Natural),
                Chord::new(I, Major, Natural),
                Chord::new(IV, Major, Natural),
                Chord::new(I, Major, Natural),
                Chord::new(II, Minor, Natural),
                Chord::new(V, Major, Natural),
                Chord::new(III, Minor, Sharp),
                Chord::new(V, Major, Natural),
                ]
        ].choose(&mut rand::thread_rng()).unwrap().clone()
    }

    pub fn generate_chorus() -> Progression {
        vec![
            vec![
                Chord::new(I, Major, Natural),
                Chord::new(IV, Major, Flat),
                Chord::new(VI, Minor, Natural),
                Chord::new(II, Minor, Natural),
                Chord::new(VI, Minor, Natural),
                Chord::new(V, Major, Natural),
                Chord::new(I, Major, Natural),
                ]
        ].choose(&mut rand::thread_rng()).unwrap().clone()
    }

    pub fn generate_progression(choruses: i32, verses: i32) -> Progression {
        let mut choruses = choruses.clone();
        let mut progression: Progression = vec![];

        let verse = Song::generate_verse();
        let chorus = Song::generate_chorus();

        for _ in 0..verses {
            progression.extend(verse.clone());

            if choruses > 0 {
                progression.extend(chorus.clone());
            }
            choruses -= 1;
        }

        for _ in 0..choruses {
            progression.extend(chorus.clone());
        }

        progression
    }
}


impl std::fmt::Display for Song {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        // write!(f, "(value a: {}, value b: {})", self.a, self.b)

        println!("Key: {:?}", self.key);
        for chord in self.progression.clone() {
            chord.show(self.key);
        }

        write!(f, "")
    }
}