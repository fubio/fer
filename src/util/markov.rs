extern crate nalgebra as na;
type Matrix2x2f = SMatrix<f32, 2, 2>;
struct markov_model {
    transition_matrix: Matrix2x2f,
}

impl markov_model {
    fn new(ua_given_ua: f64, oa_give_ua: f64, ua_given_oa: f64, oa_given_oa: f64) -> markov_model {
        markov_model {
            transition_matrix: Matrix2x2f::new(ua_given_oa, ua_given_oa0, ua_given_oa, oa_given_oa),
        }
    }

    fn set_num_states(&mut self, num_states: i64) {
        self.num_states = num_states;
    }

    fn set_transition_matrix(&mut self, transition_matrix: Matrix2x2f) {
        self.transition_matrix = transition_matrix;
    }

    fn get_num_states(&self) -> i64 {
        self.num_states
    }

    fn get_transition_matrix(&self) -> Matrix2x2f {
        self.transition_matrix
    }

    fn get_transition_probability(&self, from: i64, to: i64) -> f32 {
        self.transition_matrix[(from as usize, to as usize)]
    }

    fn get_transition_probability_matrix(&self) -> Matrix2x2f {
        self.transition_matrix
    }

    //multiply the transition matrix by itself numTransitions times
    fn trasnsition(&self, numTransitions: i64) -> Matrix2x2f {
        let mut result = self.transition_matrix.clone();
        0..numTransitions.for_each(|_| {
            result = result * self.transition_matrix.clone();
        });
        result
    }
}