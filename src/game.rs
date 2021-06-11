use crate::game_state::GameState;
use crate::die::Die;
use crate::scoreboard::Scoreboard;

struct Game {
    dice: [Die; 5]
    game_state: GameState,
    scoreboard: Scoreboard
}