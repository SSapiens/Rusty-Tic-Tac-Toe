use rand::Rng;
use crate::game::*;
use std::cmp;


fn compare (n1: &i8, n2: &i8, n3: &i8, n4: &i8, n5: &i8, n6: &i8, n7: &i8, n8: &i8, n9: &i8) -> i8 {

    let a1 = cmp::max(n1, n2);

    let a2 = cmp::max(a1, n3);

    let a3 = cmp::max(a2, n4);

    let a4 = cmp::max(a3, n5);

    let a5 = cmp::max(a4, n6);

    let a6 = cmp::max(a5, n7);

    let a7 = cmp::max(a6, n8);

    let a8 = cmp::max(a7, n9);

    return *a8;

}

pub fn maxi (gameboard: &Board, player1: Cell, maxplayer: Cell) -> i8 {

    let mut mentalboard = gameboard.copy();

    let mut num = 1;

    let mut score1 = 0;

    let mut score2 = 0;

    let mut score3 = 0;

    let mut score4 = 0;

    let mut score5 = 0;

    let mut score6 = 0;

    let mut score7 = 0;

    let mut score8 = 0;

    let mut score9 = 0;


    while num < 10 {

    let mut score = 0;

    if mentalboard.is_legal(num) == true {

        mentalboard.append(maxplayer, num);

        if mentalboard.check_win() == GameState::Win(maxplayer) {score = 5}
        else {

            mentalboard.append(player1, num);    
            if mentalboard.check_win() == GameState::Win(player1) {score  = 4}
            if mentalboard.check_win() == GameState::Tie {score = 3}
            if mentalboard.check_win() == GameState::Neutral {score = 2}
        }


        mentalboard.undo(num);

    }


    if num == 1 {score1 = score};
    if num == 2 {score2 = score};
    if num == 3 {score3 = score};
    if num == 4 {score4 = score};
    if num == 5 {score5 = score};
    if num == 6 {score6 = score};
    if num == 7 {score7 = score};
    if num == 8 {score8 = score};
    if num == 9 {score9 = score};

    num += 1;

    

    }


    //comparar



    let best = compare(&score1, &score2, &score3, &score4, &score5, &score6, &score7, &score8, &score9);
    let mut bestmove : i8 = 0;

    if best == score1 {bestmove = 1}
    if best == score2 {bestmove = 2}
    if best == score3 {bestmove = 3}
    if best == score4 {bestmove = 4}
    if best == score5 {bestmove = 5}
    if best == score6 {bestmove = 6}
    if best == score7 {bestmove = 7}
    if best == score8 {bestmove = 8}
    if best == score9 {bestmove = 9}


    if gameboard.is_first() == false {return bestmove;}

    else {


        let randmove = bestmove - rand::thread_rng().gen_range(1..9);

        if mentalboard.is_legal(randmove) {


            return randmove;

        }

        else {return bestmove;};



    }


    
}

