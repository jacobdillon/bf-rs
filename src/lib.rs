use std::collections::hash_map::{Entry, HashMap};

#[derive(Clone, Debug)]
pub enum Command {
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
    pub fn from_char(ch: char) -> Option<Self> {
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

fn get_commands(s: &str) -> Vec<Command> {
    s.chars().filter_map(Command::from_char).collect()
}

pub fn run(s: &str) -> String {
    let mut prgm = Program {
        tape: HashMap::new(),
        pointer: 0,
        jumps: Vec::new(),
        output: Vec::new(),
        commands: get_commands(s),
    };

    let mut i = 0;
    while i < prgm.commands.len() {
        match prgm.commands[i] {
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

            Command::Input => {}

            Command::JumpStart => {
                prgm.jumps.push(i);
            }

            Command::JumpEnd => match prgm.tape.entry(prgm.pointer) {
                Entry::Occupied(entry) => {
                    if *entry.get() != 0 {
                        if !prgm.jumps.is_empty() {
                            i = *prgm.jumps.last().unwrap();
                        }
                    } else {
                        prgm.jumps.pop();
                    }
                }

                _ => {}
            },
        }

        i += 1;
    }

    prgm.output.iter().collect()
}
