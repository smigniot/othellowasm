pub mod game;

fn main() {
    let board = game::Board::starter(game::Color::White);
    println!("Starting board, white starts = {}", board);
    let board = game::Board::starter(game::Color::Black);
    println!("Starting board, black starts = {}", board);

    let mut board = game::Board::starter(game::Color::Black);
    board.cells[19] = game::Cell::Filled(game::Color::Black);
    board.cells[27] = game::Cell::Filled(game::Color::Black);
    let (mine,theirs) = board.count();
    println!("Counts are {:?}/{:?} for board = {}", mine,theirs, board);

    let board = game::Board::starter(game::Color::Black);
    let moves = board.moves();
    let asxy:Vec<(usize,usize)> = moves.iter().map(|m| {
        (m%8,m/8)
    }).collect();
    println!("Moves for board = {} are {:?}", board, asxy);

    let mut board = game::Board::starter(game::Color::Black);
    let moves = board.moves();
    let asxy:Vec<(usize,usize)> = moves.iter().map(|m| {
        (m%8,m/8)
    }).collect();
    println!("Moves for board = {} are {:?}", board, asxy);

    let mv = moves[0]; 
    board.play_move(mv);
    println!("Move played = {}", board);

    let mut board = game::Board::starter(game::Color::Black);
    board.cells[35] = game::Cell::Empty;
    board.cells[36+8] = game::Cell::Filled(game::Color::White);
    board.cells[36+16] = game::Cell::Filled(game::Color::White);
    board.cells[36+24] = game::Cell::Filled(game::Color::White);
    let moves = board.moves();
    let asxy:Vec<(usize,usize)> = moves.iter().map(|m| {
        (m%8,m/8)
    }).collect();
    println!("Moves for board = {} are {:?}", board, asxy);

    board.cells[26] = game::Cell::Filled(game::Color::White);
    board.cells[25] = game::Cell::Filled(game::Color::White);
    board.cells[24] = game::Cell::Filled(game::Color::White);
    let moves = board.moves();
    println!("Moves for board = {} are {:?}", board, moves);

    board.cells[29] = game::Cell::Filled(game::Color::White);
    board.cells[30] = game::Cell::Filled(game::Color::White);
    board.cells[31] = game::Cell::Filled(game::Color::White);
    board.cells[20] = game::Cell::Filled(game::Color::White);
    let moves = board.moves();
    println!("Moves for board = {} are {:?}", board, moves);

    board.cells[12] = game::Cell::Filled(game::Color::White);
    board.cells[4] = game::Cell::Filled(game::Color::White);
    let moves = board.moves();
    println!("Moves for board = {} are {:?}", board, moves);
    println!("Note for board = {} is {:?}", board, board.note_for(game::Color::White));
    println!("Minimax for board = {} is {:?}", board, board.minimax(&game::Color::White,2));


    let board = game::Board::starter(game::Color::Black);
    println!("Minimax for board = {} is {:?}", board, board.minimax(&game::Color::Black,4));

}

