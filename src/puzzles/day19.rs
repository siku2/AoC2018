use std::collections::HashMap;
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


fn run_process(registry: &mut Registry, ip: usize, program: &Vec<Operation>) {
    let opcodes = get_opcodes();

    while let Some(operation) = program.get(registry[ip]) {
        let (opcode, a, b, c) = operation;
        let op = &opcodes[opcode];
        op.perform(registry, *a, *b, *c);

        registry[ip] += 1;
    }
}


pub fn solve_first<T: BufRead>(input: T) -> RegisterType {
    let (ip, program) = get_input(input)
        .expect("Invalid input?");

    let mut registry: Registry = vec![0; 6];
    run_process(&mut registry, ip, &program);
    registry[0]
}

fn divisor_sum(num: u64) -> u64 {
    let mut result = 0;

    for i in 2..=(num as f64).sqrt() as u64 {
        if num % i == 0 {
            if i.pow(2) == num {
                result += i;
            } else {
                result += i + num / i;
            }
        }
    }

    result + 1 + num
}

pub fn solve_second<T: BufRead>(input: T) -> u64 {
    let (ip, mut program) = get_input(input)
        .expect("Invalid input?");

    let mut registry: Registry = vec![0; 6];
    registry[0] = 1;

    // Manipulate the program such that the last line doesn't enter the loop,
    // but instead returns immediately so we may retrieve the value for the c register!
    let program_len = program.len();
    *program.last_mut().unwrap() = ("seti".to_string(), program_len + 1, 0, ip);

    run_process(&mut registry, ip, &program);

    // No idea if every input actually does it like this, but this is all I can do right now.
    // See the reverse-engineering process at the bottom of this file!
    divisor_sum(registry[2] as u64)
}

pub fn solve<T: BufRead>(problem: u8, input: T) -> Result<String, String> {
    match problem {
        1 => Result::Ok(solve_first(input).to_string()),
        2 => Result::Ok(solve_second(input).to_string()),
        _ => Result::Err("This problem only has 2 parts!".to_string())
    }
}

// raw input but substituted with arithmetic operators and letters:
/*
d += 16
e = 1
f = 1
b = e * f
b = b == c
d += b
d += 1
a += e
f += 1
b = f > c
d += b
d = 2
e += 1
b = e > c
d += b
d = 1
d *= d
c += 2
c *= c
c *= d
c *= 11
b += 5
b *= d
b += 8
c += b
d += a
d = 0
b = d
b *= d
b += d
b *= d
b *= 14
b *= d
c += b
a = 0
d = 0
*/

// use goto statements
/*
00: GOTO 17
01: e = 1
02: f = 1
03: b = e * f
04: b = b == c
05: if b == c: GOTO 7 else: GOTO 6
06: GOTO 8
07: a += e
08: f += 1
09: b = f > c
10: if f > c: GOTO 12 else: GOTO 11
11: GOTO 3
12: e += 1
13: b = e > c
14: if e > c: GOTO 16 else: GOTO 15
15: GOTO 2
16: EXIT! (GOTO 16 * 16 = 196)
17: c += 2
18: c *= c
19: c *= 19
20: c *= 11
21: b += 5
22: b *= 22
23: b += 8
24: c += b
25: if a: GOTO 27 else: GOTO 26
26: GOTO 1
27: b = 27
28: b *= 28
29: b += 29
30: b *= 30
31: b *= 14
32: b *= 32
33: c += b
34: a = 0
35: GOTO 1
*/

// basic code translation
/*
a = 1;
b = 0;
c = 0;
e = 0;
f = 0;

c = (c + 2)^2 * 19 * 11
b = (b + 5) * 22 + 8
c += b
if a:
    b = (27 * 28 + 29) * 30 * 14 * 32
    c += b
    a = 0

e = 1
# find all divisors of c and sum them up
while True:
    f = 1

    # find multiples of e that divide c
    while True:
        b = e * f

        # if e divides c
        if b == c:
            a += e

        f += 1
        if f > c:
            break

    e += 1
    if e > c:
        break

return a
*/

// simplified code
/*
def div_sum(num: int) -> int:
    result = 0
    for i in range(2, int(math.sqrt(num)) + 1):
        if num % i == 0:
            if i**2 == num:
                result += i
            else:
                result += i + num // i
    return result + 1 + num

# determine c by running code until loop starts
return div_sum(c)
*/