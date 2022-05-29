#[derive(PartialEq)]
#[derive(Debug)]
#[derive(Clone, Copy)]

pub enum Cell {

    X,
    O,
    E,

}

#[derive(Debug)]
#[derive(PartialEq)]

pub enum GameState {

    Win (Cell),
    Neutral,
    Tie,

}

#[derive(Debug)]
#[derive(PartialEq)]

pub struct Board {

    pub c1 : Cell,
    pub c2 : Cell,
    pub c3 : Cell,
    pub c4 : Cell,
    pub c5 : Cell,
    pub c6 : Cell,
    pub c7 : Cell,
    pub c8 : Cell,
    pub c9 : Cell,

}

impl Cell {

    pub fn from (player: &str) -> Cell {

        if player == "X" {return Cell::X}
        else if player == "O" {return Cell::O}
        else {return Cell::E}


    }

   pub fn to_char (self) -> char {

        if self == Cell::X {return 'X'}
        else if self == Cell::O {return 'O'}
        else if self == Cell::E {return ' '}
        else {return '?'}


    }




}

impl Board {

    pub fn new () -> Board {

        Board {

            c1 : Cell::E,
            c2 : Cell::E,
            c3 : Cell::E,
            c4 : Cell::E,
            c5 : Cell::E,
            c6 : Cell::E,
            c7 : Cell::E,
            c8 : Cell::E,
            c9 : Cell::E,

        }


    }


    pub fn append (&mut self, player: Cell, num: i8) {


        if num == 1 {self.c1 = player}
        if num == 2 {self.c2 = player}
        if num == 3 {self.c3 = player}
        if num == 4 {self.c4 = player}
        if num == 5 {self.c5 = player}
        if num == 6 {self.c6 = player}
        if num == 7 {self.c7 = player}
        if num == 8 {self.c8 = player}
        if num == 9 {self.c9 = player}


    }


    pub fn is_legal (&self, num: i8) -> bool {


        if num == 1 {if self.c1 == Cell::E {return true} else {return false}}
        if num == 2 {if self.c2 == Cell::E {return true} else {return false}}
        if num == 3 {if self.c3 == Cell::E {return true} else {return false}}
        if num == 4 {if self.c4 == Cell::E {return true} else {return false}}
        if num == 5 {if self.c5 == Cell::E {return true} else {return false}}
        if num == 6 {if self.c6 == Cell::E {return true} else {return false}}
        if num == 7 {if self.c7 == Cell::E {return true} else {return false}}
        if num == 8 {if self.c8 == Cell::E {return true} else {return false}}
        if num == 9 {if self.c9 == Cell::E {return true} else {return false}}
        else {return false}



    }


    pub fn is_first (&self) -> bool {

        let mut result = 0;

        if self.c1 == Cell::E {result += 1}
        if self.c2 == Cell::E {result += 1}
        if self.c3 == Cell::E {result += 1}
        if self.c4 == Cell::E {result += 1}
        if self.c5 == Cell::E {result += 1}
        if self.c6 == Cell::E {result += 1}
        if self.c7 == Cell::E {result += 1}
        if self.c8 == Cell::E {result += 1}
        if self.c9 == Cell::E {result += 1}

        if result == 9 || result == 8 {return true}
        else {return false}

    }

    pub fn check_win (&self) -> GameState {

        if self.c1 == self.c2 && self.c2 == self.c3 {if self.c1 != Cell::E {return GameState::Win(self.c1)}}
        if self.c4 == self.c5 && self.c5 == self.c6 {if self.c4 != Cell::E {return GameState::Win(self.c4)}}
        if self.c7 == self.c8 && self.c8 == self.c9 {if self.c7 != Cell::E {return GameState::Win(self.c7)}}
        if self.c1 == self.c4 && self.c4 == self.c7 {if self.c1 != Cell::E {return GameState::Win(self.c1)}}
        if self.c2 == self.c5 && self.c5 == self.c8 {if self.c2 != Cell::E {return GameState::Win(self.c2)}}
        if self.c3 == self.c6 && self.c6 == self.c9 {if self.c3 != Cell::E {return GameState::Win(self.c3)}}
        if self.c1 == self.c5 && self.c5 == self.c9 {if self.c1 != Cell::E {return GameState::Win(self.c1)}}
        if self.c3 == self.c5 && self.c5 == self.c7 {if self.c3 != Cell::E {return GameState::Win(self.c3)}}

        if self.c1 != Cell::E && self.c2 != Cell::E && self.c3 != Cell::E &&
            self.c4 != Cell::E && self.c5 != Cell::E && self.c6 != Cell::E &&
            self.c7 != Cell::E && self.c8 !=Cell::E && self.c9 != Cell::E {return GameState::Tie}

        else {return GameState::Neutral}



    }


    pub fn copy (&self) -> Board {

        return Board {c1: self.c1, c2: self.c2, c3: self.c3, c4: self.c4, 
            c5: self.c5, c6: self.c6, c7: self.c7, c8: self.c8, c9: self.c9}


    }


    pub fn undo (&mut self, num: i8) {


        if num == 1 {self.c1 = Cell::E}
        if num == 2 {self.c2 = Cell::E}
        if num == 3 {self.c3 = Cell::E}
        if num == 4 {self.c4 = Cell::E}
        if num == 5 {self.c5 = Cell::E}
        if num == 6 {self.c6 = Cell::E}
        if num == 7 {self.c7 = Cell::E}
        if num == 8 {self.c8 = Cell::E}
        if num == 9 {self.c9 = Cell::E}


    }

}

