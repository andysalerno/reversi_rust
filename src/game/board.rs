pub struct Piece<T> {
    body: T,
}

pub struct GameMove<T> {
    piece: Piece<T>,
    pos: (usize, usize),
}

pub struct BoardPos {
    x: usize,
    y: usize,
}

#[derive(Default)]
pub struct BoardState<T> {
    state: Vec<Piece<T>>,
}
