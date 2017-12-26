use game::game_agent::GameAgent;
use game::game_rules::GameRules;
use game::board::Piece;

pub struct RandomAgent;

impl<T: GameRules + Default> GameAgent<T> for RandomAgent {}
