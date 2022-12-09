#[macro_use]
extern crate build_const;

use opening_db_types::Node;

include!(
    concat!(
        env!("OUT_DIR"),
        concat!("/", concat!("NodeMap", ".rs"))
    )
);

pub const NODE_MAP: Node = MAP;

#[test]
fn it_works() {
    assert_eq!(NODE_MAP.get_best_move(), Some("d5".to_string()))
}