use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::collections::HashMap;
use std::collections::HashSet;
use std::io::BufRead;

use utils;

trait Add<RHS = Self> {
    type Output;
    fn add(self, rhs: RHS) -> Self::Output;
}

impl Add for (i32, i32) {
    type Output = (i32, i32);

    fn add(self, rhs: (i32, i32)) -> Self::Output {
        (self.0 + rhs.0, self.1 + rhs.1)
    }
}

#[derive(Copy, Clone)]
enum Facing {
    Left,
    Right,
    Up,
    Down,
}

impl Facing {
    fn parse(direction: char) -> Option<Facing> {
        match direction {
            '<' => Some(Facing::Left),
            '>' => Some(Facing::Right),
            '^' => Some(Facing::Up),
            'v' => Some(Facing::Down),
            _ => None,
        }
    }

    fn direction(&self) -> (i32, i32) {
        match self {
            Facing::Left => (-1, 0),
            Facing::Right => (1, 0),
            Facing::Up => (0, -1),
            Facing::Down => (0, 1),
        }
    }

    fn corner_choices(&self) -> [Facing; 2] {
        match self {
            Facing::Left => [Facing::Down, Facing::Up],
            Facing::Right => [Facing::Up, Facing::Down],
            Facing::Up => [Facing::Right, Facing::Left],
            Facing::Down => [Facing::Left, Facing::Right],
        }
    }

    fn corner_choice(&self, up: bool) -> Facing {
        self.corner_choices()[!up as usize]
    }
}

enum RelativeFacing {
    Straight,
    Left,
    Right,
}

impl RelativeFacing {
    fn get_absolute(&self, direction: Facing) -> Facing {
        match self {
            RelativeFacing::Straight => direction,
            RelativeFacing::Left => match direction {
                Facing::Left => Facing::Down,
                Facing::Right => Facing::Up,
                Facing::Up => Facing::Left,
                Facing::Down => Facing::Right,
            },
            RelativeFacing::Right => match direction {
                Facing::Left => Facing::Up,
                Facing::Right => Facing::Down,
                Facing::Up => Facing::Right,
                Facing::Down => Facing::Left,
            }
        }
    }
}

struct Cart {
    position: (i32, i32),
    facing: Facing,
    intersection_choice: RelativeFacing,
}

impl Cart {
    fn new(position: (i32, i32), facing: Facing) -> Cart {
        Cart { position, facing, intersection_choice: RelativeFacing::Left }
    }

    fn next_intersection_choice(&mut self) {
        self.intersection_choice = match &self.intersection_choice {
            RelativeFacing::Left => RelativeFacing::Straight,
            RelativeFacing::Straight => RelativeFacing::Right,
            RelativeFacing::Right => RelativeFacing::Left
        }
    }

    fn intersection_facing(&mut self) -> Facing {
        let facing = self.intersection_choice.get_absolute(self.facing);
        self.next_intersection_choice();

        facing
    }

    fn next_position(&self) -> (i32, i32) {
        return self.position.add(self.facing.direction());
    }

    fn move_to(&mut self, position: (i32, i32), facing: Facing) {
        self.position = position;
        self.facing = facing;
    }
}

#[derive(Debug)]
enum Rail {
    Straight,
    CornerUp,
    CornerDown,
    Intersection,
}


fn sort_carts(carts: &mut Vec<Cart>) {
    carts.sort_unstable_by(|x, y| {
        let (x1, y1) = x.position;
        let (x2, y2) = y.position;

        let y_cmp = y1.cmp(&y2);

        if let Ordering::Equal = y_cmp {
            x1.cmp(&x2)
        } else {
            y_cmp
        }
    });
}

fn move_cart(grid: &HashMap<(i32, i32), Rail>, cart: &mut Cart) {
    let next_pos = cart.next_position();
    let rail = grid.get(&next_pos)
        .expect("this cart isn't even on rails lol");

    let facing = match rail {
        Rail::Straight => cart.facing,
        Rail::CornerUp => cart.facing.corner_choice(true),
        Rail::CornerDown => cart.facing.corner_choice(false),
        Rail::Intersection => cart.intersection_facing()
    };

    cart.move_to(next_pos, facing);
}

fn simulate_until_crash(grid: &HashMap<(i32, i32), Rail>, carts: &mut Vec<Cart>) -> (i32, i32) {
    loop {
        sort_carts(carts);

        let mut cart_positions: HashSet<(i32, i32)> = HashSet::new();
        for cart in carts.iter_mut() {
            move_cart(grid, cart);

            if !cart_positions.insert(cart.position) {
                return cart.position;
            }
        }
    }
}

fn simulate_until_one_left(grid: &HashMap<(i32, i32), Rail>, carts: &mut Vec<Cart>) -> (i32, i32) {
    while carts.len() > 1 {
        sort_carts(carts);

        let mut cart_positions: HashMap<(i32, i32), usize> = HashMap::new();
        let mut to_remove: BTreeSet<usize> = BTreeSet::new();

        for (i, cart) in carts.iter_mut().enumerate() {
            let other_index = cart_positions.remove(&cart.position);
            move_cart(grid, cart);

            let other_index = other_index.or(cart_positions.remove(&cart.position));

            if let Some(other_index) = other_index {
                to_remove.insert(other_index);
                to_remove.insert(i);
            } else {
                cart_positions.insert(cart.position, i);
            }
        }

        for &i in to_remove.iter().rev() {
            carts.remove(i);
        }
    }

    carts.first().expect("even amount of carts, rip").position
}

fn parse_input<T: BufRead>(input: T) -> (HashMap<(i32, i32), Rail>, Vec<Cart>) {
    let mut grid = HashMap::new();
    let mut carts = Vec::new();
    let lines = utils::get_lines(input);

    let straight_rails = vec!['-', '|'];
    let corner_up_rail = '/';
    let corner_down_rail = '\\';
    let intersection_rail = '+';

    for (y, line) in lines.iter().enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c.is_whitespace() {
                continue;
            }

            let pos = (x as i32, y as i32);
            let rail: Rail;

            if straight_rails.contains(&c) {
                rail = Rail::Straight;
            } else if c == corner_up_rail {
                rail = Rail::CornerUp;
            } else if c == corner_down_rail {
                rail = Rail::CornerDown;
            } else if c == intersection_rail {
                rail = Rail::Intersection;
            } else {
                rail = Rail::Straight;
                carts.push(Cart::new(pos, Facing::parse(c)
                    .expect(format!("couldn't parse facing {}", c).as_str())));
            }

            grid.insert(pos, rail);
        }
    }

    (grid, carts)
}

pub fn solve_first<T: BufRead>(input: T) -> (i32, i32) {
    let (grid, mut carts) = parse_input(input);
    simulate_until_crash(&grid, &mut carts)
}

pub fn solve_second<T: BufRead>(input: T) -> (i32, i32) {
    let (grid, mut carts) = parse_input(input);
    simulate_until_one_left(&grid, &mut carts)
}

pub fn solve<T: BufRead>(problem: u8, input: T) -> Result<String, String> {
    match problem {
        1 => Result::Ok(format!("{:?}", solve_first(input))),
        2 => Result::Ok(format!("{:?}", solve_second(input))),
        _ => Result::Err("This problem only has 2 parts!".to_string())
    }
}