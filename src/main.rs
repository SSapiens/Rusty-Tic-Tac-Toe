mod game;
mod maxiai;
mod minimax;
mod selector;
mod game_flow;
mod drawer;
use crate::game::*;
use std::io;
use crate::drawer::draw_start;
use std::thread::sleep;
use std::time::Duration;

pub fn main () {


    loop {


        //establecer variables

        let mut player1 = Cell::E;
        let mut player2 = Cell::E;
        let mut who = String::new();
        let mut number_of_players = String::new();
        let mut difficulty_str = String::new();
        let mut difficulty = selector::AI::DefaultAI;

        //Imprimir y establecer variables

        draw_start();

        println!("¿Desea jugar con alguien? (S/N): ");

        io::stdin()
            .read_line(&mut number_of_players)
            .expect("FIN DE EJECUCIÓN: Codigo de Error 001 [Parece que hubo un problema al leer el texto introducido, esto puede deberse a una incompatiblidad con su sistema
                operativo]"); let number_of_players = number_of_players.trim().to_uppercase();


        println!("Bien, ahora eliga la letra del primer jugador: ");

        io::stdin()
            .read_line(&mut who)
            .expect("FIN DE EJECUCIÓN: Codigo de Error 002 [Parece que no introdujo una letra válida o hubo un problema con al leer el input"); let who =
            who.trim().to_uppercase();


        if who == "X" {player1 = Cell::X} else {player1 = Cell::O};


        if player1 == Cell::X {player2 = Cell::O} else {player2 = Cell::X};


        if number_of_players == "N" {

            println!("Ahora escoga la dificultad de la inteligencia artificial, (M)edio o (I)mposible: ");

            io::stdin()
                .read_line(&mut difficulty_str)
                .expect("FIN DE EJECUCIÓN: Codigo de Error 001 [Parece que hubo un problema al leer el texto introducido, esto puede deberse a una incompatiblidad con su sistema
                operativo]");


            if difficulty_str.trim().to_uppercase() == "I" {difficulty = selector::AI::Minimax}
            else {difficulty = selector::AI::Maxai}

            game_flow::alone(player1, player2, difficulty);

        }



        else if number_of_players == "S" {

            game_flow::two_players(player1, player2);


        };

        if number_of_players == "VS" {game_flow::vs(selector::AI::Minimax)}


        let mut again = String::new();

        println!("\n¿Desea jugar otra vez? (S/N)");

        io::stdin()
            .read_line(&mut again)
            .expect("FIN DE EJECUCIÓN: Codigo de Error 001 [Parece que hubo un problema al leer el texto introducido, esto puede deberse a una incompatiblidad con su sistema
                operativo]");

        if again.trim().to_uppercase() == "N" {println!("¡Entendido!"); sleep(Duration::new(1, 0)); break};

    }

}
