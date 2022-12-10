generate-opening-db DEPTH:
    rm $(pwd)/opening_db/generator/opening_db.pgn
    ln -s $(pwd)/opening_db.pgn $(pwd)/opening_db/generator/opening_db.pgn
    cd opening_db/generator && DB_SIZE={{DEPTH}} cargo run
    cp $(pwd)/opening_db/generator/node_map.rs $(pwd)/opening_db/src/node_map.rs

generate-empty-opening-db:
    cp $(pwd)/opening_db/src/node_map_default.rs $(pwd)/opening_db/src/node_map.rs