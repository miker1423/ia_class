#[derive(Clone)]
enum Move {
    Up(usize),
    Down(usize),
    Left(usize),
    Right(usize)
}

fn main() {
    let mut initial = vec![1, 8, 2, 3, 5, 6, 7, 0, 4];
    println!("{:?}", &initial);
    initial.swap(0, 5);
    println!("arr: {:?}", &initial);
    let zero_index = find_zero(&initial);
    if let Some(index) = zero_index {
        println!("{}", index);
    }
}

fn find_first_moves(puzzle: &Vec<i32>) -> Option<Vec<Move>> {
    if let Some(index) = find_zero(puzzle) {
        return Some(find_moves(index));
    }
    None
}

fn find_moves(index: usize) -> Vec<Move> {
    let column = index % 3;
    let row = index / 3;

    if row == 0 {
        if column == 0 {
            return vec![Move::Up(index + 3), Move::Left(index + 1)];
        } else if column == 1 {
            return vec![Move::Right(index - 1), Move::Up(index + 3), Move::Left(index + 1)];
        }
        return vec![Move::Right(index - 1), Move::Up(index + 3)];
    } else if row == 1 {
        if column == 0 {
            return vec![Move::Down(index - 3), Move::Left(index + 1), Move::Up(index + 3)];
        } else if column == 1 {
            return vec![Move::Down(index - 3), Move::Right(index - 1), Move::Left(index + 1), Move::Up(index + 3)];
        }
        return vec![Move::Right(index - 1), Move::Up(index + 3), Move::Down(index - 3)];
    }

    if column == 0 {
        return vec![Move::Down(index - 3), Move::Left(index + 1)];
    } else if column == 1 {
        return vec![Move::Right(index - 1), Move::Down(index + 3), Move::Left(index + 1)];
    }
    return vec![Move::Right(index - 1), Move::Down(index - 3)];
}

fn find_zero(array: &Vec<i32>) -> Option<usize> {
    for i in 0..array.len() {
        if array[i] == 0 {
            return Some(i);
        }
    }
    return None;
}
