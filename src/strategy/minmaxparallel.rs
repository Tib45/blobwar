// //! Implementation of the min max algorithm.
// use super::Strategy;
// use crate::configuration::{Configuration, Movement};
// use crate::shmem::AtomicMove;
// use rayon::prelude::*;
// use std::fmt;


// /// Parallel implementation of the Min-Max algorithm with a given recursion depth.
// pub struct MinMaxParallel(pub u8);

// impl fmt::Display for MinMaxParallel {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "Min - Max Parallel (max level: {})", self.0)
//     }
// }

// impl Strategy for MinMaxParallel {
//     fn compute_next_move(&mut self, state: &Configuration) -> Option<Movement> {
//         // Main body:
//         return rec_compute(state, self.0 as i8, true).1;
//     }
// }

// /// A function to recursively compute the value of a given state and depth for the Min-Max algorithm.
// fn rec_compute(state: &Configuration, depth: i8, is_player: bool) -> (i8, Option<Movement>) {
//     // HashMap<Movement, (i8, HashMap<...>)>
//     // Le nombre de cases dans le board:
//     static BOARDSIZE: i8 = 64;

//     match (depth, state.movements(), is_player){
//         (0, _, true) => (-state.value(), None),
//         (0, _, false) => (state.value(), None),
//         (_, iter_mvt, true) =>{

//             // Recherche du maximum:
//             let (maxi, best_mvt) = iter_mvt
//             .collect::<Vec<Movement>>()
//             .into_par_iter()
//             .fold(||(0, None), | acc, mouvement|{
//                 // Evaluation du mouvement:
//                 let current_value = rec_compute(&state.play(&mouvement), depth - 1, !is_player).0;
//                 (current_value, Some(mouvement))
//             })
//             .reduce(|| (0, None), |a, b| {
//                 if b.0 > a.0 {
//                     b
//                 } else {
//                     a
//                 }});
//             return (maxi, best_mvt);
//         },

//         (_, iter_mvt, false) =>{

//            // Recherche du minimum:
//            let (mini, best_mvt) = iter_mvt
//            .collect::<Vec<Movement>>()
//            .into_par_iter()
//            .fold(||(BOARDSIZE, None), | acc, mouvement|{
//                // Evaluation du mouvement:
//                let current_value = rec_compute(&state.play(&mouvement), depth - 1, !is_player).0;
//                (current_value, Some(mouvement))
//            })
//            .reduce(|| (BOARDSIZE, None), |a, b| {
//                let truc = a.0;
//                let machin = b.0;
//                println!("{}, {}", truc, machin);
//                if b.0 < a.0 {
//                    b
//                } else {
//                    a
//                }});
//            return (mini, best_mvt);
//         }
//     }
// }

// /// Anytime min max algorithm.
// /// Any time algorithms will compute until a deadline is hit and the process is killed.
// /// They are therefore run in another process and communicate through shared memory.
// /// This function is intended to be called from blobwar_iterative_deepening.
// pub fn min_max_parallel_anytime(state: &Configuration) {
//     let mut movement = AtomicMove::connect().expect("failed connecting to shmem");
//     for depth in 1..100 {
//         movement.store(MinMaxParallel(depth).compute_next_move(state));
//     }
// }
