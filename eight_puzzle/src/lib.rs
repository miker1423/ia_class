#[derive(Clone,Debug)]
pub enum Move {
    Up(usize),
    Down(usize),
    Left(usize),
    Right(usize)
}

#[derive(Clone)]
pub struct Puzzle {
    state: Vec<i32>,
    last_zero_index: usize,
    step: i32
}

impl PartialEq for Puzzle {
    fn eq(&self, other: &Puzzle) -> bool {
        self.state == other.state
    }
}

impl Eq for Puzzle { }

impl Puzzle {
    pub fn new(initial_state:Vec<i32>) -> Puzzle {
        let result = Self::find_zero(&initial_state);
        let step = (initial_state.len() as f64).sqrt();
        let is_int = step.fract() == 0.0;
        if !is_int {
            panic!("Array is not a square");
        }

        if let None = result {
            return Puzzle { state: initial_state, last_zero_index: 0, step: step as i32 };
        }
        Puzzle { state: initial_state, last_zero_index: result.unwrap(), step: step as i32 }
    }

    pub fn swap(&mut self, zero_index:usize, with_index:usize){
        &mut self.state.swap(zero_index, with_index);
        self.last_zero_index = with_index;
    }
    
    pub fn find_zero(array: &Vec<i32>) -> Option<usize> {
        for i in 0..array.len() {
            if array[i] == 0 {
                return Some(i);
            }
        }
        return None;
    }

    pub fn find_first_moves(&self) -> Option<Vec<Move>> {
        if let Some(index) = Self::find_zero(&self.state) {
            return Some(Self::find_moves(self, index));
        }
        None
    }

    pub fn find_moves_puzzle(&self) -> Vec<Move> {
        Self::find_moves(self, self.last_zero_index)
    }

    pub fn find_moves(&self, index: usize) -> Vec<Move> {
        let int_index = index as i32;
        let column = int_index % 3;

        let step:i32 = self.step;

        let left = column - 1;
        let up = int_index - step;
        let down = int_index + step;
        let right = column + 1;

        let mut result:Vec<Move> = Vec::new();
        if left >= 0 {
            result.push(Move::Right(index - 1));
        } 
        if up >= 0 {
            result.push(Move::Down(index - step as usize));
        } 
        if down < step * step {
            result.push(Move::Up(index + step as usize));
        } 
        if right < step {
            result.push(Move::Left(index + 1));
        }

        return result;
    }
}



use std::collections::VecDeque;

pub fn bfs(initial_state:&Puzzle, goal:Vec<i32>) -> Option<Vec<Move>> {
    if initial_state.state == goal {
        return Some(Vec::new());
    }

    let mut frontier: VecDeque<Vec<i32>> = VecDeque::new();
    let mut path: Vec<Move> = Vec::new();
    let mut visited: Vec<Puzzle> = Vec::new();

    frontier.push_front(initial_state.state.clone());
    visited.push(initial_state.clone());

    while !frontier.is_empty() {
        let first = frontier.pop_front();
        if first.unwrap() == goal {
            break;
        }

        let nbrs = 

    }



    return None;
}