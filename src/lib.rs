use std::collections::hash_map::{Entry, HashMap};
use std::io::{self, Read};

#[derive(Clone, Debug)]
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

impl Command {
    fn from_char(ch: char) -> Option<Self> {
        match ch {
            '>' => Some(Self::MovePointerRight),
            '<' => Some(Self::MovePointerLeft),
            '+' => Some(Self::Increment),
            '-' => Some(Self::Decrement),
            '.' => Some(Self::Output),
            ',' => Some(Self::Input),
            '[' => Some(Self::JumpStart),
            ']' => Some(Self::JumpEnd),
            _ => None,
        }
    }
}

#[derive(Debug)]
struct Program {
    tape: HashMap<isize, u8>,
    pointer: isize,
    jumps: Vec<usize>,
    output: Vec<char>,
    commands: Vec<Command>,
}

fn get_commands<S: AsRef<str>>(s: S) -> Vec<Command> {
    s.as_ref().chars().filter_map(Command::from_char).collect()
}

pub fn run<S: AsRef<str>>(s: S) -> String {
    let mut prgm = Program {
        tape: HashMap::new(),
        pointer: 0,
        jumps: Vec::new(),
        output: Vec::new(),
        commands: get_commands(s),
    };

    let mut pc = 0;
    while pc < prgm.commands.len() {
        match prgm.commands[pc] {
            Command::MovePointerRight => {
                prgm.pointer += 1;
            }

            Command::MovePointerLeft => {
                prgm.pointer -= 1;
            }

            Command::Increment => *prgm.tape.entry(prgm.pointer).or_insert(0) += 1,

            Command::Decrement => match prgm.tape.entry(prgm.pointer) {
                Entry::Occupied(mut entry) => {
                    if *entry.get() != 0 {
                        *entry.get_mut() -= 1;
                    }
                }

                Entry::Vacant(entry) => {
                    entry.insert(0);
                }
            },

            Command::Output => prgm
                .output
                .push(*prgm.tape.get(&prgm.pointer).unwrap_or(&0) as char),

            Command::Input => {
                let byte = io::stdin().bytes().nth(0).expect("An error occurred reading stdin.");
                if byte.is_ok() {
                    prgm.tape.insert(prgm.pointer, byte.unwrap());
                }
            }

            Command::JumpStart => {
                prgm.jumps.push(pc);
            }

            Command::JumpEnd => match prgm.tape.entry(prgm.pointer) {
                Entry::Occupied(entry) => {
                    if *entry.get() != 0 {
                        if !prgm.jumps.is_empty() {
                            pc = *prgm.jumps.last().unwrap();
                        }
                    } else {
                        prgm.jumps.pop();
                    }
                }

                _ => {}
            },
        }

        pc += 1;
    }

    prgm.output.iter().collect()
}
