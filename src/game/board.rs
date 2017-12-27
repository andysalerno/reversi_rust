#[derive(Clone)]
pub struct Piece<T> {
    body: T,
}

pub struct BoardPos {
    x: usize,
    y: usize,
}

#[derive(Default, Clone)]
pub struct BoardState<T> {
    state: Vec<Piece<T>>,
}

impl<T> BoardState<T> {
    pub fn new() -> Self {
        BoardState::<T> {
            state: Default::default(),
        }
    }
}
