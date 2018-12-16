use std::collections::HashMap;
use std::collections::HashSet;
use std::io::BufRead;
use std::iter::FromIterator;

use regex::Regex;

use utils;

type Register = Vec<u16>;
type Operation = (u16, u16, u16, u16);
type Sample = (Register, Operation, Register);
type OpMap = HashMap<&'static str, Op>;

macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}

struct Op {
    op: Box<Fn(&Register, u16, u16) -> u16>
}

impl Op {
    fn new(op: Box<Fn(&Register, u16, u16) -> u16>) -> Op {
        Op { op }
    }
    fn perform(&self, register: &mut Register, a: u16, b: u16, c: usize) {
        register[c] = (self.op)(register, a, b);
    }
}

fn parse_sample(sample: &str) -> Option<Sample> {
    let number_extractor = Regex::new(r#"\[(\d+), *(\d+), *(\d+), *(\d+)]"#).unwrap();

    if let [before, raw_op, after] = sample.split("\n").collect::<Vec<&str>>()[..] {
        let before: Register = number_extractor
            .captures(before)?
            .iter()
            .skip(1)
            .map(|n| n.unwrap().as_str().parse().unwrap())
            .collect();

        let mut it = raw_op.split_whitespace().map(|n| n.parse().unwrap());
        let mut op: Operation = (it.next().unwrap(), it.next().unwrap(), it.next().unwrap(), it.next().unwrap());

        let after: Register = number_extractor
            .captures(after)?
            .iter()
            .skip(1)
            .map(|n| n.unwrap().as_str().parse().unwrap())
            .collect::<Vec<u16>>();

        Some((before, op, after))
    } else {
        None
    }
}

fn parse_instruction(instruction: &str) -> Operation {
    let mut it = instruction.split_whitespace().map(|n| n.parse().unwrap());
    (it.next().unwrap(), it.next().unwrap(), it.next().unwrap(), it.next().unwrap())
}


fn get_input<T: BufRead>(input: T) -> Option<(Vec<Sample>, Vec<Operation>)> {
    let text = utils::get_lines_until_exit(input).join("\n");

    let mut samples = Vec::new();
    let mut program = Vec::new();

    if let [raw_samples, raw_program] = text.split("\n\n\n\n").collect::<Vec<&str>>()[..] {
        for sample in raw_samples.split("\n\n") {
            samples.push(parse_sample(sample).expect("Couldn't parse."));
        }

        for instruction in raw_program.split("\n") {
            program.push(parse_instruction(instruction));
        }

        Some((samples, program))
    } else {
        None
    }
}

fn get_opcodes() -> OpMap {
    hashmap![
        "addr" => Op::new(Box::new(|register: &Register, a, b| register[a as usize] + register[b as usize])),
        "addi" => Op::new(Box::new(|register: &Register, a, b| register[a as usize] + b)),
        "mulr" => Op::new(Box::new(|register: &Register, a, b| register[a as usize] * register[b as usize])),
        "muli" => Op::new(Box::new(|register: &Register, a, b| register[a as usize] * b)),
        "banr" => Op::new(Box::new(|register: &Register, a, b| register[a as usize] & register[b as usize])),
        "bani" => Op::new(Box::new(|register: &Register, a, b| register[a as usize] & b)),
        "borr" => Op::new(Box::new(|register: &Register, a, b| register[a as usize] | register[b as usize])),
        "bori" => Op::new(Box::new(|register: &Register, a, b| register[a as usize] | b)),
        "setr" => Op::new(Box::new(|register: &Register, a, _| register[a as usize])),
        "seti" => Op::new(Box::new(|_, a, _| a)),
        "gtir" => Op::new(Box::new(|register: &Register, a, b| (a > register[b as usize]) as u16)),
        "gtri" => Op::new(Box::new(|register: &Register, a, b| (register[a as usize] > b) as u16)),
        "gtrr" => Op::new(Box::new(|register: &Register, a, b| (register[a as usize] > register[b as usize]) as u16)),
        "eqir" => Op::new(Box::new(|register: &Register, a, b| (a == register[b as usize]) as u16)),
        "eqri" => Op::new(Box::new(|register: &Register, a, b| (register[a as usize] == b) as u16)),
        "eqrr" => Op::new(Box::new(|register: &Register, a, b| (register[a as usize] == register[b as usize]) as u16))
    ]
}

fn filter_impossible_opcodes(sample: &Sample, possible_ops: &mut HashSet<&str>, opcode_map: &OpMap) {
    let (before, operation, after) = sample;
    let mut invalid = HashSet::new();

    for &possible_op in possible_ops.iter() {
        let op = opcode_map.get(possible_op).unwrap();

        let mut register = before.clone();
        op.perform(&mut register, operation.1, operation.2, operation.3 as usize);

        if &register != after {
            invalid.insert(possible_op);
        }
    }

    invalid.iter().for_each(|&op_code| { possible_ops.remove(op_code); });
}


pub fn solve_first<T: BufRead>(input: T) -> usize {
    let opcodes = get_opcodes();
    let (samples, _) = get_input(input).expect("Invalid input?");

    let mut more_than_three = 0;

    for sample in samples {
        let mut possible_ops = opcodes.keys().map(|&s| s).collect();
        filter_impossible_opcodes(&sample, &mut possible_ops, &opcodes);

        if possible_ops.len() >= 3 {
            more_than_three += 1;
        }
    }

    more_than_three
}

pub fn solve_second<T: BufRead>(input: T) -> u16 {
    let opcodes = get_opcodes();
    let mut opcode_map: HashMap<u16, HashSet<&str>> = HashMap::new();
    for i in 0..16 {
        opcode_map.insert(i, opcodes.keys().map(|&s| s).collect());
    }

    let (samples, program) = get_input(input).expect("Invalid input?");

    for sample in samples {
        let opcode = (sample.1).0;
        let mut possible_ops = opcode_map.get_mut(&opcode).unwrap();

        filter_impossible_opcodes(&sample, &mut possible_ops, &opcodes);
    }

    let mut ambiguous = true;
    while ambiguous {
        let reserved: HashSet<&str> = opcode_map.values().filter_map(|v| if v.len() == 1 { Some(*v.iter().next().unwrap()) } else { None }).collect();
        if reserved.is_empty() {
            panic!("Input ambiguous!");
        }

        ambiguous = false;
        for (_, value) in opcode_map.iter_mut() {
            if value.len() > 1 {
                ambiguous = true;

                for &res in reserved.iter() {
                    value.remove(res);
                }
            }
        }
    }

    let opcode_map: HashMap<u16, &str> = HashMap::from_iter(opcode_map.iter().map(|(&k, v)| (k, *v.iter().next().expect("there's no opcode rip"))));
    let mut register: Register = vec![0; 4];

    for instruction in program {
        let op = opcodes.get(*opcode_map.get(&instruction.0).unwrap()).unwrap();
        op.perform(&mut register, instruction.1, instruction.2, instruction.3 as usize);
    }

    register[0]
}

pub fn solve<T: BufRead>(problem: u8, input: T) -> Result<String, String> {
    match problem {
        1 => Result::Ok(solve_first(input).to_string()),
        2 => Result::Ok(solve_second(input).to_string()),
        _ => Result::Err("This problem only has 2 parts!".to_string())
    }
}