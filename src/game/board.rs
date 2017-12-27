#[derive(Clone)]
pub struct Piece<T> {
    body: T,
}

pub struct BoardPos {
    x: usize,
    y: usize,
}

#[derive(Default)]
pub struct BoardState<T: Clone> {
    state: Vec<Piece<T>>,
}

impl<T: Clone> Clone for BoardState<T> {
    fn clone(&self) -> Self {
        let cloned = self.state.clone();

        BoardState { state: cloned }
    }
}

impl<T: Clone> BoardState<T> {
    pub fn new() -> Self {
        BoardState::<T> {
            state: Default::default(),
        }
    }
}
