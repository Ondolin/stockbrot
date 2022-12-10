# Stockbrot Chess Engine

*The stockbrot engine is currently under development.*

![Logo](https://github.com/Ondolin/stockbrot/blob/master/Stockbrot.png?raw=true)

This project contains an Open Source chess engine written in pure rust.

## How to use Stockbrot?

First read the content in the Opening DB section.

Copy the content of `.env.example` to a `.env` file and populate its fields.

To use the engine run `just run`.

## Lichess integration

To make it easier to play against Stockbrot there is an integration with the Lichess API.  
The official Stockbrot Lichess Account is [this](https://lichess.org/@/StockbrotEngine).

## Opening Database

Stockbrot can be used in combination with an opening DB.

### Generate Database

To generate a Database you first need to download a bunch of games as a `.pgn` file. For example, you can get them from [lichess](https://database.lichess.org).

Put the database in the root of the project and rename it to `opening_db.pgn`.

To generate the rust code run `just generate-opening-db [LENGTH]`. You can limit the size of the Database by setting a low `[LENGTH]` value. A value of 5000 is quite good and reasonably fast to calculate.

### Empty Database

If you don't want to use a database you have to type `just generate-empty-opening-db`. If you don't do this the program will not compile!