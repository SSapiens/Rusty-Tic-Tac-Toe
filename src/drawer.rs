use crate::game::Board;
use crate::game::GameState;
use crate::game::Cell;

pub fn print_winner (gameboard: &Board, player1: Cell, is_ai: bool) {

	match gameboard.check_win() {

		GameState::Neutral => {println!("\n*****RESULTADO: LA PARTIDA AÚN NO HA TERMINADO*****")},
		GameState::Tie => {println!("\n*****RESULTADO: LA PARTIDA HA TERMINADO EN EMPATE******")},
		GameState::Win(winner) => {

			if is_ai {

				if winner == player1 {println!("\n*****RESULTADO: LA PARTIDA HA GANADO EL JUGADOR UNO******")}
				else {println!("\n*****RESULTADO: LA PARTIDA LA HA GANADO LA INTELIGENCIA ARTIFICIAL******")}

			}

			else {

				if winner == player1 {println!("\n*****RESULTADO: LA PARTIDA LA HA GANADO EL JUGADOR UNO******")}
				else {println!("\n*****RESULTADO: LA PARTIDA HA LA HA GANADO EL JUGADOR DOS******")}

			}

		},
	}




}

pub fn draw_start () {


    println!("

         _______  __        _______               _______               
        |_     _||__|.----.|_     _|.---.-..----.|_     _|.-----..-----.
          |   |  |  ||  __|  |   |  |  _  ||  __|  |   |  |  _  ||  -__|
          |___|  |__||____|  |___|  |___._||____|  |___|  |_____||_____|
                                                                        

        ");

    println!("Bienvenido a TTT (TicTacToe), puede jugar con una inteligencia artificial o con un alguien...");



}


pub fn draw_board (gameboard: &Board) {

	println!("

	    |     |     
	 {}  |  {}  |  {}  
	____|_____|_____
	    |     |     
	 {}  |  {}  |  {}  
	____|_____|_____
	    |     |     
	 {}  |  {}  |  {}  
	    |     |     ", 
	gameboard.c1.to_char(), 
	gameboard.c2.to_char(), gameboard.c3.to_char(),
	gameboard.c4.to_char(), gameboard.c5.to_char(), 
	gameboard.c6.to_char(), gameboard.c7.to_char(), 
	gameboard.c8.to_char(), gameboard.c9.to_char());

}


