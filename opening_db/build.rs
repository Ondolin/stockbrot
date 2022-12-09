use std::collections::HashMap;
use std::{env, fs};
use std::fs::File;
use std::path::Path;
use build_const::ConstWriter;
use std::io::{self, prelude::*, BufReader};
use shakmaty::{CastlingMode, Move};
use shakmaty::san::SanPlus;
use opening_db_types::{BuildNode, Node, NodeCount};

fn main() {

    let out_dir = env::var("OUT_DIR").unwrap();
    let mod_name = format!("NodeMap.rs");
    let dest_path = Path::new(&out_dir).join(mod_name);
    let mut file = File::create(&dest_path).unwrap();

    // write!(file, "pub use opening_db_types::Node; \n").unwrap();

    let mut root = BuildNode::new("".to_string());

    add_sub_node(&mut root);

    let text = format!("{:?}", root);
    let mut result = String::new();
    for c in text.chars() {

        if c == '[' {
            result += "&";
        }

        result.push(c);
    }

    result = result.replace("\\", "");

    let type_string = format!(
        "pub const {}: {} = {};\n",
        "MAP",
        "opening_db_types::Node",
        result,
    );

    let type_string = type_string.replace("BuildNode", "opening_db_types::Node");

    write!(file, "{}", type_string).unwrap();

}

fn add_sub_node(root: &mut BuildNode) {
    use shakmaty::{Chess, Position, san::San};

    let file = File::open("opening_db.pgn").unwrap();
    let reader = BufReader::new(file);

    let mut db_depth = 0;

    for line in reader.lines() {

        let line = line.unwrap();

        if !line.starts_with("1.") { continue; }

        if line.contains("{") { continue; }

        let mut pos = Chess::default();
        let mut current_node = &mut *root;

        let mut iter = line.split(".");
        iter.next();

        for turn in iter {
            let mut turn = turn.split(" ");
            turn.next();

            let one = turn.next().unwrap();
            if one.contains("-") && !one.contains("O-O") { continue }

            let one: SanPlus = one.parse().unwrap();

            let one = one.san.to_move(&pos).unwrap();
            //assert_eq!(one, SanPlus::from_ascii(&[b'e', b'4']).unwrap());

            pos = pos.play(&one).expect("Valid move");

            let one = one.to_uci(CastlingMode::Standard).to_string();

            current_node = current_node.add_child(BuildNode::new(one));

            let two = turn.next().unwrap();

            if two.contains("-") && !two.contains("O-O")  { continue }

            let two: SanPlus = two.parse().unwrap();
            let two = two.san.to_move(&pos).unwrap();
            pos = pos.play(&two).expect("Valid move");

            let two = two.to_uci(CastlingMode::Standard).to_string();

            current_node = current_node.add_child(BuildNode::new(two));

        }

        db_depth += 1;

        if db_depth > 5000 {
            return;
        }


    }
}