//! Implementation of the min max algorithm.
use super::Strategy;
use crate::configuration::{Configuration, Movement};
use crate::shmem::AtomicMove;
use std::fmt;

/// Min-Max algorithm with a given recursion depth.
pub struct MinMax(pub u8);

/// Une fonction auxiliaire pour gérer la profondeur:
fn rec_compute(state: &Configuration, depth: i8, is_player: bool) -> (i8, Option<Movement>) {
// HashMap<Movement, (i8, HashMap<...>)>
    // Le nombre de cases dans le board:
    static BOARDSIZE: i8 = 64;

    match (depth, state.movements(), is_player){
        (0, _, true) => (-state.value(), None),
        (0, _, false) => (state.value(), None),
        (_, iter_mvt, true) =>{
            // Initialisation:
            let mut maxi = -BOARDSIZE;
            let mut best_mvt = None;

            // Recherche du maximum:
            for mouvement in iter_mvt{
                // Initialisation:
                let current_value = rec_compute(&state.play(&mouvement), depth - 1, !is_player).0;

                if current_value > maxi {
                    maxi = current_value;
                    best_mvt = Some(mouvement);
                }
            }
            return (maxi, best_mvt);
        },

        (_, iter_mvt, false) =>{
            // Initialisation:
            let mut mini = BOARDSIZE;
            let mut best_mvt = None;

            // Recherche du maximum:
            for mouvement in iter_mvt{
                let current_value = rec_compute(&state.play(&mouvement), depth - 1, !is_player).0;

                if current_value < mini{
                    mini = current_value;
                    best_mvt = Some(mouvement);
                }
            }
            return (mini, best_mvt);
        }
    }
}

impl Strategy for MinMax {
    // Optimisation: on utilise un hashmap pour stocker les valeurs:
    fn compute_next_move(&mut self, state: &Configuration) -> Option<Movement> {
        // Main body:
        return rec_compute(state, self.0 as i8, true).1;
    }
}

impl fmt::Display for MinMax {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Min - Max (max level: {})", self.0)
    }
}

/// Anytime min max algorithm.
/// Any time algorithms will compute until a deadline is hit and the process is killed.
/// They are therefore run in another process and communicate through shared memory.
/// This function is intended to be called from blobwar_iterative_deepening.
pub fn min_max_anytime(state: &Configuration) {
    let mut movement = AtomicMove::connect().expect("failed connecting to shmem");
    for depth in 1..100 {
        movement.store(MinMax(depth).compute_next_move(state));
    }
}
