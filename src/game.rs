use crate::game_state::GameState;
use crate::die::Die;
use crate::scoreboard::Scoreboard;

struct Game {
    dice: [Die; 5],
    game_state: GameState,
    scoreboard: Scoreboard
}

impl Game {
    fn new() -> Game {
        Game {
            dice: [Die::new(); 5],
            game_state: GameState::NewGame,
            scoreboard: Scoreboard::new()
        }
    }
}