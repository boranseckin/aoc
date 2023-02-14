use std::{fmt, collections::VecDeque};

const DISPLAY_MASK: u64 = 0b1111111111111111111111111111111111111111;

#[derive(Debug, Copy, Clone)]
enum Instruction {
    Noop,
    Addx(i32),
}

impl Instruction {
    fn get_cycles(&self) -> u8 {
        match self {
            Self::Noop => 1,
            Self::Addx(_) => 2,
        }
    }
}

struct StateMachine {
    instructions: VecDeque<Instruction>,
    current: Option<Instruction>,
    cycles: u32,
    cycles_left: u8,
    reg_x: i32,
    crt: Vec<u64>,
}

impl fmt::Debug for StateMachine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "cycles: {}\t reg_x: {}\t current={:?}\t ({} instructions left)",
            self.cycles,
            self.reg_x,
            self.current,
            self.instructions.len()
        )?;

        for line in &self.crt {
            for i in 0..40 {
                let c = if line & Self::cycle_mask(i) > 0 { '#' } else { '.' };
                write!(f, "{c}")?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl StateMachine {
    fn new() -> Self {
        let input: VecDeque<Instruction> = include_str!("./prod.txt")
            .lines()
            .map(|line| {
                let mut parts = line.split_whitespace();
                let opcode = parts.next().unwrap().to_string();

                match opcode.as_str() {
                    "noop" => Instruction::Noop,
                    "addx" => Instruction::Addx(parts.next().unwrap().parse().unwrap()),
                    _ => panic!("AOC lied to me."), 
                }
            })
            .collect();

        let mut instance = StateMachine {
            instructions: input,
            current: None,
            cycles: 0,
            cycles_left: 0,
            reg_x: 1,
            crt: vec![],
        };

        instance.load();
        instance
    }

    fn load(&mut self) {
        let instruction = self.instructions.pop_front().map(|i| (i, i.get_cycles()));

        if let Some(instruction) = instruction {
            self.current = Some(instruction.0);
            self.cycles_left = instruction.1;
        } else {
            self.current = None;
            self.cycles_left = 0;
        }
    }

    fn step(&mut self) {
        self.cycles += 1;

        if self.current.is_none() {
            return;
        }

        if self.cycles_left > 1 {
            self.cycles_left -= 1;
            return;
        }

        let instruction = self.current.unwrap();

        match instruction {
            Instruction::Noop => {},
            Instruction::Addx(x) => self.reg_x += x,
        };

        self.load();
    }

    // Inspiration: https://fasterthanli.me/series/advent-of-code-2022/part-10
    fn cycle_mask(cycle: u32) -> u64 {
        (0b10000000000000000000000000000000000000000 >> ((cycle % 40) + 1)) & DISPLAY_MASK 
    }

    fn sprite_pos(pos: u32) -> u64 {
        0b11100000000000000000000000000000000000000_u64.overflowing_shr(pos).0 & DISPLAY_MASK
    }

    fn draw(&mut self) {
        let line = (self.cycles / 40) as usize;

        if line >= self.crt.len() {
            self.crt.push(0);
        }

        let line = self.crt.get_mut(line).unwrap();
        let cycle_mask = Self::cycle_mask(self.cycles);
        let sprite = Self::sprite_pos(self.reg_x as u32);

        *line |= cycle_mask & sprite;
    }
}

fn main() {
    let mut machine = StateMachine::new();

    loop {
        machine.draw();

        println!("{:?}", machine);

        machine.step();
        if machine.current.is_none() {
            break;
        }
    }
}

