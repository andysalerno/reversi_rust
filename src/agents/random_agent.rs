use game::game_agent::GameAgent;
use game::game_rules::GameRules;
use game::board::Piece;

pub struct RandomAgent;

impl RandomAgent {
    fn new() -> Self {
        RandomAgent {}
    }
}

impl GameAgent for RandomAgent {}
