extern crate nalgebra as na;
use na::{U2, U3, Dynamic, ArrayStorage, VecStorage, SMatrix};
type Matrix2x2f = na::SMatrix<f64, 2, 2>;
pub struct markov_model {
    transition_matrix: Matrix2x2f,
}

impl markov_model {
    pub fn new(ua_given_ua: f64, oa_give_ua: f64, ua_given_oa: f64, oa_given_oa: f64) -> markov_model {
        markov_model {
            transition_matrix: Matrix2x2f::new(ua_given_ua, oa_give_ua, ua_given_oa, oa_given_oa),
        }
    }

    pub fn set_transition_matrix(&mut self, transition_matrix: Matrix2x2f) {
        self.transition_matrix = transition_matrix;
    }

    fn get_transition_matrix(&self) -> Matrix2x2f {
        self.transition_matrix
    }

    pub fn get_transition_probability(&self, from: i64, to: i64) -> f64 {
        self.transition_matrix[(from as usize, to as usize)]
    }

    pub fn get_transition_probability_matrix(&self) -> Matrix2x2f {
        self.transition_matrix
    }

    //multiply the transition matrix by itself numTransitions times
    pub fn transition(&self, numTransitions: i64) -> Matrix2x2f {
        let mut result = self.transition_matrix.clone();
        (0..numTransitions).for_each(|_| {
            result *= self.transition_matrix;
        });
        result
    }
}