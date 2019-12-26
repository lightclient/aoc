use common::load_input;
use std::collections::HashMap;
use std::hash::Hash;
use vm::{Int, Status, Vm};

#[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq)]
struct Position {
    x: i64,
    y: i64,
}

struct Game {
    vm: Vm,
    display: HashMap<Position, Sprite>,
    score: u64,
    ball: Position,
    paddle: Position,
}

impl Game {
    pub fn new(code: &[Int]) -> Self {
        let mut s = Self {
            vm: Vm::new(code),
            display: HashMap::new(),
            score: 0,
            ball: Position::default(),
            paddle: Position::default(),
        };

        s.next_frame();
        s.vm.reset();
        s.vm.set(0, 2);
        s
    }

    pub fn next_frame(&mut self) -> bool {
        let mut out = vec![];

        loop {
            match self.vm.step() {
                Ok(Status::AwaitingInput) => return true,
                Ok(Status::Output(o)) => out.push(o),
                Ok(Status::Halted) => return false,
                Ok(_) => continue,
                Err(e) => panic!("{:?}", e),
            }

            if out.len() == 3 {
                let p = Position {
                    x: out[0],
                    y: out[1],
                };

                if p == (Position { x: -1, y: 0 }) {
                    self.score = out[2] as u64;
                } else {
                    let s = Sprite::from_raw(out[2]);

                    match s {
                        Sprite::Ball => self.ball = p,
                        Sprite::Paddle => self.paddle = p,
                        _ => (),
                    }

                    self.display.insert(p, s);
                }

                out.clear();
            }
        }
    }

    pub fn simulate(&mut self) -> u64 {
        while self.next_frame() {
            self.vm.insert_input((self.ball.x - self.paddle.x).signum())
        }

        self.score
    }
}

#[derive(Eq, PartialEq)]
enum Sprite {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
}

impl Sprite {
    pub fn from_raw(n: Int) -> Self {
        match n {
            0 => Self::Empty,
            1 => Self::Wall,
            2 => Self::Block,
            3 => Self::Paddle,
            4 => Self::Ball,
            n => panic!("unknown tile {}", n),
        }
    }
}

fn main() {
    let code: Vec<Int> = load_input!(",", Int);
    let mut game = Game::new(&code);

    game.next_frame();

    let blocks: usize = game
        .display
        .iter()
        .filter(|(_, v)| **v == Sprite::Block)
        .collect::<HashMap<&Position, &Sprite>>()
        .len();

    println!("Block tiles: {}", blocks);
    println!("Final score: {}", game.simulate());
}
