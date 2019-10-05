use std::collections::hash_map::{Entry, HashMap};

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

pub fn run<S, I>(s: S, args: I) -> String 
where S: AsRef<str>,
      I: IntoIterator<Item=S> 
{
    let mut prgm = Program {
        tape: HashMap::new(),
        pointer: 0,
        jumps: Vec::new(),
        output: Vec::new(),
        commands: get_commands(s),
    };

    let joined = args.into_iter().fold(String::from(""), |acc, item| {
        if !acc.is_empty() {
            return acc + " " + &item.as_ref();
        }

        item.as_ref().to_owned()
    });
    let mut args: Vec<u8> = joined.bytes().collect();

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
                if !args.is_empty() {
                    prgm.tape.insert(prgm.pointer, args.remove(0));
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

#[cfg(test)]
mod tests {
    use super::run;

    #[test]
    fn prints_hello_world() {
        assert_eq!(&run("++++++++[>++++[>++>+++>+++>+<<<<-]>+>+>->>+[<]<-]>>.>---.+++++++..+++.>>.<-.<.+++.------.--------.>>+.>++.", vec![]), "Hello World!\n")
    }

    #[test]
    fn reads_hello_world() {
        assert_eq!(&run("+++++++++++[>,.<-]", vec!["Hello", "World"]), "Hello World");
    }
}