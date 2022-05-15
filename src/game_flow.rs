use crate::selector::ai_calculation;
use crate::selector::AI;
use crate::game::Board;
use crate::game::GameState;
use crate::game::Cell;
use std::io;
use crate::drawer::draw_board;
use crate::drawer::print_winner;

pub fn alone (player1: Cell, aiplayer: Cell, ai_to_use: AI) {

	let mut gameboard = Board::new();

	loop {

		let mut move_to = String::new();

		println!("\nTurno del Jugador UNO");

		draw_board(&gameboard);

		println!("\nColoque una posición en tablero desde 1 hasta 9:");


		io::stdin()
			.read_line(&mut move_to)
			.expect("FIN DE EJECUCIÓN: Codigo de Error 001 [Parece que hubo un problema al leer el texto introducido, esto puede deberse a una incompatiblidad con su sistema
	            operativo]");


		let move_to : i8 = move_to.trim().parse().expect("FIN DE EJECUCIÓN: Codigo de Error 003 
			[Parece que no introdujo un número válida o hubo un problema con al leer el input");


		if gameboard.is_legal(move_to) {

			gameboard.append(player1, move_to);

			if gameboard.check_win() != GameState::Neutral {

				draw_board(&gameboard);

				print_winner(&gameboard, player1, true); break;

			}


		}

		else {

			println!("\nEse movimiento no esta permitido (Pierde un turno)");

		}


		println!("\nLa inteligencia artificial ha realizado su movimiento...");


		let ai_move_to = ai_calculation(&gameboard, player1, aiplayer, &ai_to_use);
		gameboard.append(aiplayer, ai_move_to);


			if gameboard.check_win() != GameState::Neutral {

				draw_board(&gameboard);

				print_winner(&gameboard, player1, true); break;

			}

	}

}


pub fn two_players (player1: Cell, player2: Cell) {

	let mut gameboard = Board::new();



	loop {


		let mut player1_move_to = String::new();


		println!("\nTurno del Jugador UNO");

		draw_board(&gameboard);

		println!("\nColoque una posición en tablero desde 1 hasta 9:");

		io::stdin()
			.read_line(&mut player1_move_to)
			.expect("FIN DE EJECUCIÓN: Codigo de Error 001 [Parece que hubo un problema al leer el texto introducido, esto puede deberse a una incompatiblidad con su sistema
	            operativo]");


		let player1_move_to : i8 = player1_move_to.trim().parse().expect("FIN DE EJECUCIÓN: Codigo de Error 003 
			[Parece que no introdujo un número válida o hubo un problema con al leer el input");


		if gameboard.is_legal(player1_move_to) {

			gameboard.append(player1, player1_move_to);

			if gameboard.check_win() != GameState::Neutral {

				draw_board(&gameboard);

				print_winner(&gameboard, player1, false); break;

			}


		}

		else {

			println!("\nEse movimiento no esta permitido (Pierde un turno)");

		}

		let mut player2_move_to = String::new();

		println!("\nTurno del Jugador DOS");

		draw_board(&gameboard);

		println!("\nColoque una posición en tablero desde 1 hasta 9:");

		io::stdin()
			.read_line(&mut player2_move_to)
			.expect("FIN DE EJECUCIÓN: Codigo de Error 001 [Parece que hubo un problema al leer el texto introducido, esto puede deberse a una incompatiblidad con su sistema
	            operativo]");


		let player2_move_to: i8 = player2_move_to.trim().parse().expect("FIN DE EJECUCIÓN: Codigo de Error 003 
			[Parece que no introdujo un número válida o hubo un problema con al leer el input");


		if gameboard.is_legal(player2_move_to) {

			gameboard.append(player2, player2_move_to);

			if gameboard.check_win() != GameState::Neutral {

				draw_board(&gameboard);

				print_winner(&gameboard, player1, false); break;

			}


		}

		else {

			println!("\nEse movimiento no esta permitido (Pierde un turno)");

		}

	}

}

pub fn vs (ai_to_use: AI) {

	let mut gameboard = Board::new();
	let ai1 = Cell::X;
	let ai2 = Cell::O;

	loop {

		let ai1_move = ai_calculation(&gameboard, ai2, ai1, &ai_to_use);

		gameboard.append(ai1, ai1_move);

		draw_board(&gameboard);

		if gameboard.check_win() != GameState::Neutral {

			print_winner(&gameboard, ai1, false); break;

		}

		let ai2_move = ai_calculation(&gameboard, ai1, ai2, &ai_to_use);

		gameboard.append(ai2, ai2_move);

		draw_board(&gameboard);

		if gameboard.check_win() != GameState::Neutral {

			print_winner(&gameboard, ai1, false); break;

		}








	}






}



