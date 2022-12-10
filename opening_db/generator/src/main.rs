use std::env;
use std::fs::File;
use std::path::Path;
use std::io::{prelude::*, BufReader};
use shakmaty::CastlingMode;
use shakmaty::san::SanPlus;
use opening_db_types::{BuildNode};

fn main() {

    let dest_path = Path::new("node_map.rs");
    let mut file = File::create(&dest_path).unwrap();

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
    use shakmaty::{Chess, Position};

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

        if db_depth > env::var("DB_SIZE").unwrap().parse().unwrap() {
            return;
        }


    }
}