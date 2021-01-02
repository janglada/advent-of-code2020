use std::collections::HashSet;
use std::convert::TryInto;

#[derive(Debug, Clone, Copy)]
enum InstructionKind {
    Nop,
    Acc,
    Jmp,
}
#[derive(Debug, Clone, Copy)]
struct Instruction {
    kind: InstructionKind,
    operand: isize,
}

type Program = Vec<Instruction>;

fn parse_program(input: &str) -> Program {
    input
        .lines()
        .map(|l| {
            let mut tokens = l.split(' ');
            Instruction {
                kind: match tokens.next() {
                    Some(tok) => match tok {
                        "nop" => InstructionKind::Nop,
                        "acc" => InstructionKind::Acc,
                        "jmp" => InstructionKind::Jmp,
                        _ => panic!("unknown instruction kind {}", tok),
                    },
                    None => panic!("for line {}, expected instruction kind", l),
                },
                operand: match tokens.next() {
                    Some(tok) => tok.parse().unwrap(),
                    None => panic!("for line {}, expected operand", l),
                },
            }
        })
        .collect()
}

#[derive(Debug, Clone, Copy, Default)]
struct State {
    /// Program counter
    pc: usize,
    /// Accumulator
    acc: isize,
}

impl State {
    fn next(self, program: &Program) -> Self {
        let ins = program[self.pc];
        match ins.kind {
            InstructionKind::Nop => Self {
                pc: self.pc + 1,
                ..self
            },
            InstructionKind::Acc => Self {
                pc: self.pc + 1,
                acc: self.acc + ins.operand,
            },
            InstructionKind::Acc => Self {
                pc: self.pc + 1,
                acc: self.acc + ins.operand,
            },
            InstructionKind::Jmp => Self {
                pc: (self.pc as isize + ins.operand).try_into().unwrap(),
                ..self
            },
        }
    }
}

pub fn day_eight() {
    let program = parse_program(include_str!("../day8.txt"));
    let mut iter = itertools::iterate(State::default(), |s| s.next(&program));
    let mut set: HashSet<usize> = Default::default();
    let answer = iter.find(|state| !set.insert(state.pc)).unwrap();

    println!(
        "Before executing {} a second time, the accumulator was {}",
        answer.pc, answer.acc
    );
    //dbg!(program);
}
