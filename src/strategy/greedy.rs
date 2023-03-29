//! Dumb greedy algorithm.
use super::Strategy;
use crate::configuration::{Configuration, Movement};
use std::{fmt};

/// Dumb algorithm.
/// Amongst all possible movements return the one which yields the configuration with the best
/// immediate value.
pub struct Greedy();

impl fmt::Display for Greedy {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Greedy")
    }
}

impl Strategy for Greedy {
    fn compute_next_move(&mut self, state: &Configuration) -> Option<Movement> {
        // Initialisation:
        let iter_mvt = state.movements();
        let mut mini: i8 = state.value() + 1;
        let mut mvt_choisi: Option<Movement> = None;

        // Recherche de minimum:
        for movement in iter_mvt{
            let valeur = -1*state.play(&movement).value();
            if valeur < mini{
                mini = valeur;
                mvt_choisi = Some(movement);
            }        
        }
        return mvt_choisi;
    }
}
