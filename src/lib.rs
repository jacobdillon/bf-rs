#![feature(plugin)]
#![plugin(phf_macros)]

extern crate phf;

use std::collections::hash_map::{HashMap, Entry};

#[derive(Clone,Debug)]
enum Command {
    MovePointerRight,
    MovePointerLeft,
    Increment,
    Decrement,
    Output,
    Input,
    JumpStart,
    JumpEnd,
}

static CHARACTER_MAP: phf::Map<char, Command> = phf_map! {
    '>' => Command::MovePointerRight,
    '<' => Command::MovePointerLeft,
    '+' => Command::Increment,
    '-' => Command::Decrement,
    '.' => Command::Output,
    ',' => Command::Input,
    '[' => Command::JumpStart,
    ']' => Command::JumpEnd,
};

#[derive(Debug)]
struct Program {
    tape: HashMap<isize, u8>,
    pointer: isize,
    jumps: Vec<usize>,
    output: Vec<char>,
    commands: Vec<Command>,
}

fn get_commands(s: &str) -> Vec<Command> {
    s.chars()
        .filter(|c| CHARACTER_MAP.contains_key(c))
        .map(|c| CHARACTER_MAP.get(&c).unwrap())
        .cloned()
        .collect()
}

pub fn run(s: &str) -> String {
    let mut prgm = Program {
        tape: HashMap::new(),
        pointer: 0,
        jumps: Vec::new(),
        output: Vec::new(),
        commands: get_commands(s)
    };

    let mut i = 0;
    while i < prgm.commands.len() {
        match prgm.commands[i] {
            Command::MovePointerRight => {
                prgm.pointer += 1;
            },

            Command::MovePointerLeft => {
                prgm.pointer -= 1;
            },

            Command::Increment => {
                *prgm.tape.entry(prgm.pointer).or_insert(0) += 1
            },

            Command::Decrement => {
                match prgm.tape.entry(prgm.pointer) {
                    Entry::Occupied(mut entry) => {
                        if *entry.get() != 0 {
                            *entry.get_mut() -= 1;
                        }
                    },

                    Entry::Vacant(entry) => {
                        entry.insert(0);
                    },
                }
            },

            Command::Output => {
                prgm.output.push(*prgm.tape.get(&prgm.pointer).unwrap_or(&0) as char)
            },

            Command::Input => {

            },

            Command::JumpStart => {
                prgm.jumps.push(i);
            },

            Command::JumpEnd => {
                match prgm.tape.entry(prgm.pointer) {
                    Entry::Occupied(entry) => {
                        if *entry.get() != 0 {
                            if !prgm.jumps.is_empty() {
                                i = *prgm.jumps.last().unwrap();
                            }
                        } else {
                            prgm.jumps.pop();
                        }
                    },

                    _ => {},
                }
            },
        }

        i += 1;
    }

    prgm.output.iter().collect()
}