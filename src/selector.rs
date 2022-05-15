use crate::Board;
use crate::Cell;
use crate::minimax::minimax_find;
use crate::maxiai::maxi;


#[derive(Clone, Copy)]
pub enum AI {
	Maxai,
	Minimax,
	DefaultAI,
}
 
pub fn ai_calculation (gameboard: &Board, player1: Cell, maxplayer: Cell, ai: &AI) -> i8 {

	match ai {
		AI::Maxai => {

			return maxi(&gameboard, player1, maxplayer);
		},
		AI::Minimax => {

			return minimax_find(&gameboard, player1, maxplayer);

		},
		AI::DefaultAI => {

			return maxi(&gameboard, player1, maxplayer);
		},


	}

}
