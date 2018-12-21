use std::collections::HashMap;
use std::collections::HashSet;
use std::io::BufRead;

use utils;

type RegisterType = usize;
type Registry = Vec<RegisterType>;
type Operation = (String, RegisterType, RegisterType, RegisterType);
type OpMap = HashMap<String, Op>;

macro_rules! hashmap {
    ($( $key: expr => $val: expr ),*) => {{
         let mut map = ::std::collections::HashMap::new();
         $( map.insert($key, $val); )*
         map
    }}
}

struct Op {
    op: Box<Fn(&Registry, RegisterType, RegisterType) -> RegisterType>
}

impl Op {
    fn new(op: Box<Fn(&Registry, RegisterType, RegisterType) -> RegisterType>) -> Op {
        Op { op }
    }
    fn perform(&self, registry: &mut Registry, a: RegisterType, b: RegisterType, c: usize) {
        registry[c] = (self.op)(registry, a, b);
    }
}

fn get_opcodes() -> OpMap {
    hashmap![
        "addr".to_string() => Op::new(Box::new(|register: &Registry, a, b| register[a as usize] + register[b as usize])),
        "addi".to_string() => Op::new(Box::new(|register: &Registry, a, b| register[a as usize] + b)),
        "mulr".to_string() => Op::new(Box::new(|register: &Registry, a, b| register[a as usize] * register[b as usize])),
        "muli".to_string() => Op::new(Box::new(|register: &Registry, a, b| register[a as usize] * b)),
        "banr".to_string() => Op::new(Box::new(|register: &Registry, a, b| register[a as usize] & register[b as usize])),
        "bani".to_string() => Op::new(Box::new(|register: &Registry, a, b| register[a as usize] & b)),
        "borr".to_string() => Op::new(Box::new(|register: &Registry, a, b| register[a as usize] | register[b as usize])),
        "bori".to_string() => Op::new(Box::new(|register: &Registry, a, b| register[a as usize] | b)),
        "setr".to_string() => Op::new(Box::new(|register: &Registry, a, _| register[a as usize])),
        "seti".to_string() => Op::new(Box::new(|_, a, _| a)),
        "gtir".to_string() => Op::new(Box::new(|register: &Registry, a, b| (a > register[b as usize]) as RegisterType)),
        "gtri".to_string() => Op::new(Box::new(|register: &Registry, a, b| (register[a as usize] > b) as RegisterType)),
        "gtrr".to_string() => Op::new(Box::new(|register: &Registry, a, b| (register[a as usize] > register[b as usize]) as RegisterType)),
        "eqir".to_string() => Op::new(Box::new(|register: &Registry, a, b| (a == register[b as usize]) as RegisterType)),
        "eqri".to_string() => Op::new(Box::new(|register: &Registry, a, b| (register[a as usize] == b) as RegisterType)),
        "eqrr".to_string() => Op::new(Box::new(|register: &Registry, a, b| (register[a as usize] == register[b as usize]) as RegisterType))
    ]
}


fn parse_instruction(instruction: &str) -> Operation {
    let mut it = instruction.split_whitespace();

    let parse_next = |it: &mut Iterator<Item=&str>| it.next().unwrap().parse().unwrap();

    (it.next().unwrap().to_string(), parse_next(&mut it), parse_next(&mut it), parse_next(&mut it))
}

const COMP_REG: usize = 5;

fn get_input<T: BufRead>(input: T) -> Option<(usize, Vec<Operation>)> {
    let mut lines = utils::get_lines(input);

    let ip_line = lines.remove(0);
    let ip = ip_line.split_whitespace()
        .last()?
        .parse().ok()?;

    let mut program = Vec::new();

    for line in lines.iter() {
        program.push(parse_instruction(line.as_str()));
    }

    Some((ip, program))
}


fn run_process(registry: &mut Registry, ip: usize, program: &Vec<Operation>, find_last: bool) -> Option<usize> {
    let opcodes = get_opcodes();
    let mut seen = HashSet::new();
    let mut last_seen: usize = 0;

    while let Some(operation) = program.get(registry[ip]) {
        let (opcode, a, b, c) = operation;
        let op = &opcodes[opcode];
        op.perform(registry, *a, *b, *c);
        registry[ip] += 1;

        // eqrr comparison on line 28 which checks whether reg[0] == reg[COMP_REG] (5 for my input)
        if registry[ip] == 28 {
            if !find_last || seen.contains(&registry[COMP_REG]) {
                return Some(last_seen);
            } else {
                last_seen = registry[COMP_REG];
                seen.insert(last_seen);
            }
        }
    }

    None
}


pub fn solve_first<T: BufRead>(input: T) -> usize {
    let (ip, program) = get_input(input)
        .expect("Invalid input?");

    let mut registry: Registry = vec![0; 6];
    run_process(&mut registry, ip, &program, false);

    registry[COMP_REG]
}

pub fn solve_second<T: BufRead>(input: T) -> usize {
    let (ip, program) = get_input(input)
        .expect("Invalid input?");

    let mut registry: Registry = vec![0; 6];
    run_process(&mut registry, ip, &program, true)
        .unwrap()
}

pub fn solve<T: BufRead>(problem: u8, input: T) -> Result<String, String> {
    match problem {
        1 => Result::Ok(solve_first(input).to_string()),
        2 => Result::Ok(solve_second(input).to_string()),
        _ => Result::Err("This problem only has 2 parts!".to_string())
    }
}