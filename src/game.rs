use std::{fmt,cmp};


const NEIGHBORS:[(i8,i8);8] = [ (-1,-1), (-1,0), (-1,1), (0,-1), (0,1), (1,-1), (1,0), (1,1) ];
const PASS:usize = 256;
const NOTEMAX:i32 = 1024*1024;
const CXSQUARE:i32 = -1;
const CORNER:i32 = 10;

///
/// A player color, `Black` or `White`
///
#[derive(Clone,Copy,PartialEq)]
pub enum Color {
    Black,
    White,
}

///
/// Cells are `Empty` or `Filled` by a pawn of a given `Color`
///
#[derive(Clone,Copy)]
pub enum Cell {
    Empty,
    Filled(Color),
}

///
/// A board situation has a next player and 8x8 cells
///
#[derive(Clone,Copy)]
pub struct Board {
    pub player: Color,
    pub cells: [Cell;64],
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            Color::Black => "Black",
            Color::White => "White",
        })
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", match *self {
            Cell::Empty => " ",
            Cell::Filled(Color::White) => "○",
            Cell::Filled(Color::Black) => "●",
        })
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}\n [{}{}{}{}{}{}{}{}]\n [{}{}{}{}{}{}{}{}]\n [{}{}{}{}{}{}{}{}]\n [{}{}{}{}{}{}{}{}]\n [{}{}{}{}{}{}{}{}]\n [{}{}{}{}{}{}{}{}]\n [{}{}{}{}{}{}{}{}]\n [{}{}{}{}{}{}{}{}]]",
            self.player,
            self.cells[0], self.cells[1], self.cells[2], self.cells[3], self.cells[4], self.cells[5], self.cells[6], self.cells[7],
            self.cells[8], self.cells[9], self.cells[10], self.cells[11], self.cells[12], self.cells[13], self.cells[14], self.cells[15],
            self.cells[16], self.cells[17], self.cells[18], self.cells[19], self.cells[20], self.cells[21], self.cells[22], self.cells[23],
            self.cells[24], self.cells[25], self.cells[26], self.cells[27], self.cells[28], self.cells[29], self.cells[30], self.cells[31],
            self.cells[32], self.cells[33], self.cells[34], self.cells[35], self.cells[36], self.cells[37], self.cells[38], self.cells[39],
            self.cells[40], self.cells[41], self.cells[42], self.cells[43], self.cells[44], self.cells[45], self.cells[46], self.cells[47],
            self.cells[48], self.cells[49], self.cells[50], self.cells[51], self.cells[52], self.cells[53], self.cells[54], self.cells[55],
            self.cells[56], self.cells[57], self.cells[58], self.cells[59], self.cells[60], self.cells[61], self.cells[62], self.cells[63])
    }
}

pub struct Movelist {
    pub moves:[u8;64],
    pub n:usize,
}

impl Board {
    ///
    /// Allocate a board from cstr bytes.
    ///
    pub fn from_bytes(bytes:Vec<u8>,color:Color) -> Board {
        let mut cells = [Cell::Empty;64];
        for i in 0..64 {
            cells[i] = match bytes[i] {
                49 => Cell::Filled(Color::White),
                50 => Cell::Filled(Color::Black),
                _ => Cell::Empty,
            };
        }
        Board {
            player:color,
            cells:cells,
        }
    }
    ///
    /// The starting position board, with `Color` playing first.
    ///
    pub fn starter(color:Color) -> Board {
        Board {
            player: color,
            cells: [
                Cell::Empty,Cell::Empty,Cell::Empty,Cell::Empty,Cell::Empty,Cell::Empty,Cell::Empty,Cell::Empty,
                Cell::Empty,Cell::Empty,Cell::Empty,Cell::Empty,Cell::Empty,Cell::Empty,Cell::Empty,Cell::Empty,
                Cell::Empty,Cell::Empty,Cell::Empty,Cell::Empty,Cell::Empty,Cell::Empty,Cell::Empty,Cell::Empty,

                Cell::Empty,Cell::Empty,Cell::Empty,
                Cell::Filled(Color::White),
                Cell::Filled(Color::Black),
                Cell::Empty,Cell::Empty,Cell::Empty,
                Cell::Empty,Cell::Empty,Cell::Empty,
                Cell::Filled(Color::Black),
                Cell::Filled(Color::White),
                Cell::Empty,Cell::Empty,Cell::Empty,

                Cell::Empty,Cell::Empty,Cell::Empty,Cell::Empty,Cell::Empty,Cell::Empty,Cell::Empty,Cell::Empty,
                Cell::Empty,Cell::Empty,Cell::Empty,Cell::Empty,Cell::Empty,Cell::Empty,Cell::Empty,Cell::Empty,
                Cell::Empty,Cell::Empty,Cell::Empty,Cell::Empty,Cell::Empty,Cell::Empty,Cell::Empty,Cell::Empty,
            ],
        }
    }

    ///
    /// Return the counts of pawns, i.e. (player, opponent)
    ///
    pub fn count(&self) -> (i32,i32) {
        self.count_for(&self.player)
    }

    fn count_for(&self,player:&Color) -> (i32,i32) {
        self.cells.iter()
            .fold((0,0), |(mine,theirs), cell| {
                match *cell {
                    Cell::Empty => (mine,theirs),
                    Cell::Filled(ref who) => {
                        if who == player {
                            (mine+1,theirs)
                        } else {
                            (mine,theirs+1)
                        }
                    }
                }
            })
    }

    ///
    /// Return the moves, a vector of cell numbers. 256 means PASS.
    ///
    pub fn moves(&self) -> Vec<usize> {
        self.moves_for(&self.player, false)
    }

    fn moves_for(&self, player:&Color, previous_passed:bool) -> Vec<usize> {
        let opponent = match *player {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };
        let mut res:Vec<usize> = Vec::with_capacity(64);
        for i in 0..64 {
            // Find an empty cell
            match self.cells[i] { 
                Cell::Empty => { },
                _ => { continue },
            };
            let (x0,y0) = (i%8,i/8);
            // For all neighbor cells
            let neighbor = NEIGHBORS.iter().find(|&&(dx,dy)| {
                let (x,y) = ((x0 as i8)+dx,(y0 as i8)+dy);
                if (x < 0) || (x > 7) { return false; }
                if (y < 0) || (y > 7) { return false; }
                // If on board
                match self.cells[(y*8+x) as usize] {
                    Cell::Empty => { return false; },
                    Cell::Filled(ref c) => { return opponent == *c; }, // Is a neighbor opponent
                }
            });
            match neighbor {
                Some(_) => { 
                    // Empty cell has any neighbor opponent
                    if self.is_changed(i,player) {
                        // Would flip pawns so is a valid move
                        res.push(i); 
                    }
                },
                _ => { },
            }
        }
        if (!previous_passed) && res.is_empty() {
            let next_ones = self.moves_for(&opponent, true);
            if ! next_ones.is_empty() {
                res.push(PASS);
                res
            } else {
                res
            }
        } else {
            res
        }
    }

    fn is_changed(&self, cell:usize, player:&Color) -> bool {
        let opponent = match *player {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };
        let (x0,y0) = (cell%8,cell/8);
        for &(dx,dy) in NEIGHBORS.iter() {
            let (mut x,mut y) = ((x0 as i8)+dx,(y0 as i8)+dy);
            if (x < 0) || (x > 7) { continue };
            if (y < 0) || (y > 7) { continue };
            match self.cells[(y*8+x) as usize] {
                Cell::Empty => { continue },
                Cell::Filled(ref c) => { 
                    if opponent != *c {
                        continue;
                    }
                },
            }
            let mut done = false;
            while !done {
                x += dx;
                y += dy;
                if (x < 0) || (x > 7) { break; };
                if (y < 0) || (y > 7) { break; };
                match self.cells[(y*8+x) as usize] {
                    Cell::Empty => { done = true; },
                    Cell::Filled(ref c) => { 
                        if opponent != *c {
                            done = true;
                        }
                    },
                }
            }
            if (x < 0) || (x > 7) { continue };
            if (y < 0) || (y > 7) { continue };
            match self.cells[(y*8+x) as usize] {
                Cell::Empty => { },
                Cell::Filled(ref c) => { 
                    if *player == *c {
                        return true;
                    }
                },
            }
        }
        return false;
    }

    ///
    /// Play a move by cell number
    ///
    pub fn play_move(&mut self, cell:usize) -> Vec<usize> {
        let mut swapped:Vec<usize> = Vec::with_capacity(32);
        let mut toswap:Vec<usize> = Vec::with_capacity(32);
        let opponent = match self.player {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };
        if cell > 63 { 
            self.player = opponent;
            return swapped; 
        }
        match self.cells[cell] {
            Cell::Empty => {
                let (x0,y0) = (cell%8,cell/8);
                for &(dx,dy) in NEIGHBORS.iter() {
                    toswap.clear();
                    let (mut x,mut y) = ((x0 as i8)+dx,(y0 as i8)+dy);
                    if (x < 0) || (x > 7) { continue };
                    if (y < 0) || (y > 7) { continue };
                    match self.cells[(y*8+x) as usize] {
                        Cell::Empty => { continue },
                        Cell::Filled(ref c) => { 
                            if opponent != *c {
                                continue;
                            } else {
                                toswap.push((y*8+x) as usize)
                            }
                        },
                    }
                    let mut done = false;
                    while !done {
                        x += dx;
                        y += dy;
                        if (x < 0) || (x > 7) { done = true; };
                        if (y < 0) || (y > 7) { done = true; };
                        let n = y*8+x;
                        if !done {
                            match self.cells[(y*8+x) as usize] {
                                Cell::Empty => { done = true; },
                                Cell::Filled(ref c) => { 
                                    if opponent != *c {
                                        done = true;
                                    } else {
                                        toswap.push(n as usize)
                                    }
                                },
                            }
                        }
                    }
                    if (x < 0) || (x > 7) { continue };
                    if (y < 0) || (y > 7) { continue };
                    match self.cells[(y*8+x) as usize] {
                        Cell::Empty => { },
                        Cell::Filled(c) => { 
                            if self.player == c {
                                for &n in toswap.iter() {
                                    self.cells[n] = Cell::Filled(self.player);
                                }
                                swapped.append(&mut toswap);
                            }
                        },
                    }
                }
                self.cells[cell] = Cell::Filled(self.player);
                self.player = opponent;
            },
            _ => { }
        }
        return swapped;
    }

    pub fn note_for(&self, player:Color) -> i32 {
        let (mine,theirs) = self.cells.iter().enumerate().fold((0,0), |acc,(i,&cell)| {
            match cell {
                Cell::Filled(ref c) => {
                    let iscorner = (i==0)||(i==7)||(i==56)||(i==63);
                    let c_square = (i==1)||(i==8)||(i==9)||
                                   (i==6)||(i==14)||(i==15)||
                                   (i==48)||(i==49)||(i==57)||
                                   (i==54)||(i==55)||(i==62);
                    let diff = if iscorner { CORNER } else if c_square { CXSQUARE } else { 1 };
                    if player == *c {
                        (acc.0+diff,acc.1)
                    } else {
                        (acc.0,acc.1+diff)
                    }
                }
                _ => acc,
            }
        });
        mine-theirs
    }

    pub fn minimax(&self, player:&Color, depth:i32) -> i32 {
        let mut copy:Board = self.clone();
        copy.mut_minimax(player, depth, true)
    }

    fn mut_minimax(&mut self, player:&Color, depth:i32, maximizing:bool) -> i32 {
        if depth <= 0 {
            self.note_for(*player)
        } else {
            let moves = self.moves();
            if moves.is_empty() {
                self.note_for(*player)
            } else {
                let mut res:i32 = if maximizing { -NOTEMAX } else { NOTEMAX };
                let playing = self.player.clone();
                for c2 in moves {
                    let swapped = self.play_move(c2);
                    let minimax2 = self.mut_minimax(player, depth-1, !maximizing);
                    if maximizing {
                        res = cmp::max(res,minimax2);
                    } else {
                        res = cmp::min(res,minimax2);
                    }
                    self.unplay(playing, c2, swapped);
                }
                res
            }
        }
    }
     
    fn unplay(&mut self, player:Color, cell:usize, swapped:Vec<usize>) {
        // empty cell at move
        if cell < 63 {
            self.cells[cell] = Cell::Empty;
        }
        // re-flip pawns
        let opponent = match player {
            Color::White => Color::Black,
            Color::Black => Color::White,
        };
        for n in swapped {
            self.cells[n] = Cell::Filled(opponent);
        }
        self.player = player;
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_moves_are_ok() {
        let starter = Board::starter(Color::Black);
        let moves = starter.moves();
        assert_eq!(4, moves.len());

        let e = Cell::Empty;
        let w = Cell::Filled(Color::White);
        let b = Cell::Filled(Color::Black);
        assert_eq!(4, Board { player:Color::Black, cells: [
            e,e,e,e,e,e,e,e,
            e,e,e,e,e,e,e,e,
            e,e,e,e,e,e,e,e,
            e,e,e,w,b,e,e,e,
            e,e,e,b,w,e,e,e,
            e,e,e,e,e,e,e,e,
            e,e,e,e,e,e,e,e,
            e,e,e,e,e,e,e,e,
        ] }.moves().len());

        assert_eq!(5, Board { player:Color::Black, cells: [
            e,e,e,e,e,e,e,e,
            e,e,e,e,e,e,e,e,
            e,e,w,w,w,e,e,e,
            e,e,e,w,b,e,e,e,
            e,e,e,b,b,e,e,e,
            e,e,e,e,b,e,e,e,
            e,e,e,e,e,e,e,e,
            e,e,e,e,e,e,e,e,
        ] }.moves().len());
    }

    #[test]
    fn pass_move_is_detected() {
        let e = Cell::Empty;
        let w = Cell::Filled(Color::White);
        let b = Cell::Filled(Color::Black);
        assert_eq!(1, Board { player:Color::Black, cells: [
            e,e,e,e,e,e,e,e,
            e,e,e,e,e,e,e,e,
            e,e,e,e,e,e,e,e,
            e,e,e,b,e,e,e,e,
            e,e,e,w,w,e,e,e,
            e,e,e,w,e,w,e,e,
            e,e,e,w,e,e,w,e,
            e,e,e,w,e,e,e,w,
        ] }.moves().len());
        assert_eq!(0, Board { player:Color::Black, cells: [
            w,e,e,w,e,e,e,e,
            e,w,e,w,e,e,e,e,
            e,e,w,w,e,e,e,e,
            e,e,e,b,e,e,e,e,
            e,e,e,w,w,e,e,e,
            e,e,e,w,e,w,e,e,
            e,e,e,w,e,e,w,e,
            e,e,e,w,e,e,e,w,
        ] }.moves().len());
    }

    #[test]
    fn simple_minimax_invariant() {
        let starter = Board::starter(Color::Black);
        assert_eq!(0, starter.minimax(&Color::Black,0));
        assert_eq!(3, starter.minimax(&Color::Black,1));
        assert_eq!(0, starter.minimax(&Color::Black,2));

        assert_eq!(0, starter.minimax(&Color::White,0));
        assert_eq!(-3, starter.minimax(&Color::White,1));
        assert_eq!(0, starter.minimax(&Color::White,2));
    }
}
