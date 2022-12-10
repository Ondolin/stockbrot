use chess::BitBoard;

pub mod connectivity;
pub mod isolated;

const RANK_BLOCKS: [BitBoard; 8] = [
    BitBoard(255),
    BitBoard(65535),
    BitBoard(16777215),
    BitBoard(4294967295),
    BitBoard(1099511627775),
    BitBoard(281474976710655),
    BitBoard(72057594037927935),
    BitBoard(18446744073709551615),
];
