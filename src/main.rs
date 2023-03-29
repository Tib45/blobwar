extern crate blobwar;
//use blobwar::board::Board;
use blobwar::configuration::Configuration;
use blobwar::strategy::{Greedy, MinMax, AlphaBeta, MinMaxParallel};
use std::time::{Duration, Instant};

fn main() {
    let start = Instant::now();
    //let board = Board::load("x").expect("failed loading board");
    let board = Default::default();
    let mut game = Configuration::new(&board);
    game.battle(MinMaxParallel(2), MinMaxParallel(2));
    let duration = start.elapsed();
    println!("Time elapsed in expensive_function() is: {:?}", duration);
}
