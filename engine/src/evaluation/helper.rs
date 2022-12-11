use chess::{BitBoard, Board, Color, get_rook_moves, Piece, Square};

pub fn rook_xray(board: &Board, field: Square, color: Color) -> BitBoard {
    let my_supporters = (board.pieces(Piece::Rook)
        | board.pieces(Piece::Queen)) & board.color_combined(color);

    get_rook_moves(field, *board.combined() & !my_supporters)
}

pub fn bishop_xray(board: &Board, field: Square, color: Color) -> BitBoard {
    let my_supporters = (board.pieces(Piece::Rook)
        | board.pieces(Piece::Queen)) & board.color_combined(color);

    get_rook_moves(field, *board.combined() & !my_supporters)
}