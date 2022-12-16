use chess::{BitBoard, get_bishop_moves, get_knight_moves, get_rook_moves, Piece, Square};

pub fn piece_mobility(p: &Piece, board: BitBoard, field: Square) -> (i32, i32) {
    match p {
        Piece::Knight => {
            let free_fields =  get_knight_moves(field) & !board;
            let count = free_fields.count();
            (MG_KNIGHT[count], EG_KNIGHT[count])
        },
        Piece::Bishop => {
            let free_fields =  get_bishop_moves(field, board) & !board;
            let count = free_fields.count();
            (MG_BISHOP[count], EG_BISHOP[count])
        },
        Piece::Rook => {
            let free_fields =  get_rook_moves(field, board) & !board;
            let count = free_fields.count();
            (MG_ROOK[count], EG_ROOK[count])
        },
        Piece::Queen => {
            let bishop_moves = get_bishop_moves(field, board);
            let rook_moves = get_rook_moves(field, board);

            let queen_moves = (bishop_moves | rook_moves) & !board;

            let count = queen_moves.count();
            (MG_QUEEN[count], EG_QUEEN[count])
        },
        _ => (0, 0)
    }
}

const MG_KNIGHT: [i32; 9] = [-62,-53,-12,-4,3,13,22,28,33];
const MG_BISHOP: [i32; 14] = [-48,-20,16,26,38,51,55,63,63,68,81,81,91,98];
const MG_ROOK: [i32; 15] = [-60,-20,2,3,3,11,22,31,40,40,41,48,57,57,62];
const MG_QUEEN: [i32; 28] = [-30,-12,-8,-9,20,23,23,35,38,53,64,65,65,66,67,67,72,72,77,79,93,108,108,108,110,114,114,116];

const EG_KNIGHT: [i32; 9] = [-81,-56,-31,-16,5,11,17,20,25];
const EG_BISHOP: [i32; 14] = [-59,-23,-3,13,24,42,54,57,65,73,78,86,88,97];
const EG_ROOK: [i32; 15] = [-78,-17,23,39,70,99,103,121,134,139,158,164,168,169,172];
const EG_QUEEN: [i32; 28] = [-48,-30,-7,19,40,55,59,75,78,96,96,100,121,127,131,133,136,141,147,150,151,168,168,171,182,182,192,219];