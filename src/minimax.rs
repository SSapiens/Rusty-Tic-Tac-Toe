use crate::game::*;
use std::cmp;

pub fn minimax (gameboard: &Board, player1: Cell, maxplayer: Cell, depeth: i32, ismaxturn: bool) -> i32 {


	let mut mentalboard = gameboard.copy();

	if mentalboard.check_win() == GameState::Win(maxplayer) {return 10}
	if mentalboard.check_win() == GameState::Tie {return 0}
	if mentalboard.check_win() == GameState::Win(player1) {return -10}
	else {

		let mut num = 0;

		if ismaxturn {


			let mut best = -1000;


			while num < 9 {

				num+=1;

				if mentalboard.is_legal(num) {

					mentalboard.append(maxplayer, num);


					best = cmp::max(best, minimax(&mentalboard, player1, maxplayer, depeth+1, !ismaxturn));


					mentalboard.undo(num);

				}


			}

		return best;


		}


		else {



			num = 0;

			let mut best = 1000;


			while num < 9 {

				num += 1;

				if mentalboard.is_legal(num) {

					mentalboard.append(player1, num);

					best = cmp::min(best, minimax(&mentalboard, player1, maxplayer, depeth+1, !ismaxturn));

					mentalboard.undo(num);


				}


			}


		return best;




		}

	}


}


pub fn minimax_find (gameboard: &Board, player1: Cell, maxplayer: Cell) -> i8 {

	let mut best_val = -1000;
	let mut best_move = -1;
	let mut move_val = 0;

	let mut num = 0;

	let mut mentalboard = gameboard.copy();

	while num < 9 {

		num += 1;

		if mentalboard.is_legal(num) {

			mentalboard.append(maxplayer, num);

			move_val = minimax(&mentalboard, player1, maxplayer, 0, false);

			mentalboard.undo(num);

			if move_val > best_val {best_move = num; best_val = move_val}

		}

	}

	return best_move;


}
