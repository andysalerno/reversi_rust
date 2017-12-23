
pub trait GameAgent<T> 
where T: Piece<T> {
    fn pick_move<'a>(&self, board: &BoardState<T>, choices: &'a [GameMove<T>]) -> &'a GameMove<T>;
    fn color(&self) -> PlayerColor;
}