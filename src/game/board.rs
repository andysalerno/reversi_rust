pub trait Piece<T> {
    fn value(&self) -> T;
}

pub struct GameMove<T: Piece<T>> {
    piece: T,
    pos: (usize, usize),
}

pub struct BoardPos {
    x: usize,
    y: usize,
}

#[derive(Default)]
pub struct BoardState<T> {
    state: Vec<T>,
}
