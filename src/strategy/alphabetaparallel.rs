// //! Alpha - Beta algorithm.
// use std::fmt;

// use super::Strategy;
// use crate::configuration::{Configuration, Movement};
// use crate::shmem::AtomicMove;
// use rayon::prelude::*;

// /// Anytime alpha beta algorithm.
// /// Any time algorithms will compute until a deadline is hit and the process is killed.
// /// They are therefore run in another process and communicate through shared memory.
// /// This function is intended to be called from blobwar_iterative_deepening.
// pub fn alpha_beta_parallel_anytime(state: &Configuration) {
//     let mut movement = AtomicMove::connect().expect("failed connecting to shmem");
//     for depth in 1..100 {
//         let chosen_movement = AlphaBetaParallel(depth).compute_next_move(state);
//         movement.store(chosen_movement);
//     }
// }

// /// Alpha - Beta algorithm with given maximum number of recursions.
// pub struct AlphaBetaParallel(pub u8);

// impl fmt::Display for AlphaBetaParallel {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         write!(f, "Alpha - Beta Parallel (max level: {})", self.0)
//     }
// }

// fn rec_compute(state: &Configuration, depth: i8, is_player: bool, mut alpha: i8, mut beta: i8) -> (i8, Option<Movement>) {
//     // HashMap<Movement, (i8, HashMap<...>)>
//     // Le nombre de cases dans le board:
//     static BOARDSIZE: i8 = 64;

//     match (depth, state.movements(), is_player){
//         (0, _, true) => (-state.value(), None),
//         (0, _, false) => (state.value(), None),
//         (_, iter_mvt, true) =>{
//             // Initialisation:
//             let mut maxi = -BOARDSIZE;
//             let mut best_mvt = None;

//             // Recherche du maximum:
//             for mouvement in iter_mvt{
//                 // Evaluation du mouvement:
//                 let current_value = rec_compute(&state.play(&mouvement), depth - 1, !is_player, alpha, beta).0;

//                 if current_value > maxi {
//                     maxi = current_value;
//                     best_mvt = Some(mouvement);
//                 }

//                 // Mise à jour d'alpha:
//                 alpha = std::cmp::max(alpha, maxi);

//                 // Vérification de la condition d'élagage:
//                 if beta <= alpha {
//                     break;
//                 }
//             }
//             return (maxi, best_mvt);
//         },

//         (_, iter_mvt, false) =>{
//             // Initialisation:
//             let mut mini = BOARDSIZE;
//             let mut best_mvt = None;

//             // Recherche du minimum:
//             let (mini, best_mvt) = iter_mvt
//             .collect::<Vec<Movement>>()
//             .into_par_iter()
//             .fold(||(BOARDSIZE, None), | acc, mouvement|{
//                 // Evaluation du mouvement:
//                 let current_value = rec_compute(&state.play(&mouvement), depth - 1, !is_player, alpha, beta).0;
//                 (current_value, Some(mouvement))
//             }) 
//             .max_by_key(|&(minimum, mvt)| minimum)
//             .unwrap();

//             // Mise à jour de beta:
//             beta = std::cmp::min(beta, mini);

//             // Vérification de la condition d'élagage:
//             if beta <= alpha {
//                 return (mini, best_mvt);
//             }
//         }
//     }
// }

// impl Strategy for AlphaBetaParallel {
//     // Optimisation: on utilise un hashmap pour stocker les valeurs:
//     fn compute_next_move(&mut self, state: &Configuration) -> Option<Movement> {
//         // Initialisation d'alpha et beta:
//         let alpha = -64;
//         let beta = 64;

//         // Main body:
//         return rec_compute(state, self.0 as i8, true, alpha, beta).1;
//     }
// }