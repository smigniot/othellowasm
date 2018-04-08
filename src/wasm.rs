pub mod game;

use std::mem;
use std::ffi::{CStr,CString};
use std::os::raw::{c_void,c_char};

#[no_mangle]
pub extern "C" fn alloc(size: usize) -> *mut c_void {
    let mut buf = Vec::with_capacity(size);
    let ptr = buf.as_mut_ptr();
    mem::forget(buf);
    return ptr as *mut c_void;
}

#[no_mangle]
pub extern "C" fn dealloc(ptr: *mut c_void, cap: usize) {
    unsafe  {
        let _buf = Vec::from_raw_parts(ptr, 0, cap);
    }
}

#[no_mangle]
pub fn othello_moves(ptr: *mut c_char, player:i32) -> *mut c_char {
    unsafe {
        let data = CStr::from_ptr(ptr);
        let txt = data.to_string_lossy().into_owned();
        let bytes = txt.into_bytes();
        let player = match player {
            1 => game::Color::Black,
            _ => game::Color::White,
        };
        let board = game::Board::from_bytes(bytes, player);
        let moves = board.moves();
        let repr = moves.iter().map(|x| { x.to_string() }).collect::<Vec<_>>().join(",");
        let s = CString::new(repr).unwrap();
        s.into_raw()
    }
}

#[no_mangle]
pub fn othello_play(ptr: *mut c_char, player:i32, cell:i32) -> *mut c_char {
    unsafe {
        let data = CStr::from_ptr(ptr);
        let txt = data.to_string_lossy().into_owned();
        let bytes = txt.into_bytes();
        let player = match player {
            1 => game::Color::Black,
            _ => game::Color::White,
        };
        let mut board = game::Board::from_bytes(bytes, player);
        board.play_move(cell as usize);
        let repr = board.cells.iter().map(|&c| match c {
            game::Cell::Empty => "0",
            game::Cell::Filled(ref co) => match *co {
                game::Color::White => "1",
                game::Color::Black => "2",
            },
        }).collect::<Vec<_>>().join("");
        let s = CString::new(repr).unwrap();
        s.into_raw()
    }
}

#[no_mangle]
pub fn othello_minimax(ptr: *mut c_char, player:i32, depth:i32) -> i32 {
    unsafe {
        let data = CStr::from_ptr(ptr);
        let txt = data.to_string_lossy().into_owned();
        let bytes = txt.into_bytes();
        let opponent = match player {
            1 => game::Color::White,
            _ => game::Color::Black,
        };
        let player = match player {
            1 => game::Color::Black,
            _ => game::Color::White,
        };
        let board = game::Board::from_bytes(bytes, player);
        board.minimax(&player, depth)
    }
}

