use std::collections::HashMap;
use crate::util::vcsd;
use crate::util::markov;
use crate::util::markov::markov_model;

pub struct FERCalculator {
    td: Vec<(u64, f64)>,
    //this is just the expectation of the tenancy distribution
    pcs: u64,
    vcs_dist: HashMap<u64, f64>,
}

impl FERCalculator {
    pub fn new(td: Vec<(u64, f64)>, efficiency: f64) -> FERCalculator {
        let pcs = tenancy_expectation(&td);
        let td_map = td.iter().map(|(tenacy, prob)| (*tenacy, *prob)).collect::<HashMap<_, _>>();
        let vcs_dist = vcsd::generate_vcsd(td_map);
        FERCalculator {
            td,
            pcs,
            vcs_dist,
        }
    }
    // pub fn efficiency(&self) -> f64 {
    //
    // }
    /*
    Calculates the remaining or unstored tenancy on average per access.
    This is slightly lower then what we want, what we really want is tenancy remainging when we VCS>PCS (a forced eviction takes place)
     */
    pub fn unstored_per_access(&self) -> f64 {
        self.td.iter().fold(0.0, |acc, (tenancy, prob)| acc + tenancy.pow(2) as f64 * *prob) / (self.pcs as f64 * 2.0)
    }

    pub fn tenancy_remaining_given_fe(&self) -> f64 {
        self.unstored_per_access()/self.pcs as f64 * (self.pcs as f64 + self.oa_expectation())
    }

    pub fn print_oa_dist(&self) {
        self.vcs_dist.iter().for_each(|(vcs, prob)| {
            let overalloc = *vcs as isize - self.pcs as isize;
            if overalloc > 0 && *prob > 0.0 {
                println!("{} {}", overalloc, prob);
            }
        });
    }

    pub fn overalloc_dist(&self) -> HashMap<u64, f64> {
        let mut map: HashMap<u64, f64> = HashMap::new();
        let mut total = 0.0;
        self.vcs_dist.iter().for_each(|(vcs, prob)| {
            let overalloc = *vcs as isize - self.pcs as isize;
            if overalloc > 0 {
                total += prob;
                map.insert(overalloc as u64, *prob);
            }
        });
        map.insert(0, 1.0-total);
        map
    }

    pub fn overalloc_dist_renormalized(&self) -> HashMap<u64, f64> {
        let mut map: HashMap<u64, f64> = HashMap::new();
        let mut total = 0.0;
        self.vcs_dist.iter().for_each(|(vcs, prob)| {
            let overalloc = *vcs as isize - self.pcs as isize;
            if overalloc > 0 {
                total += prob;
                map.insert(overalloc as u64, *prob);
            }
        });
        map.iter().map(|(vcs, prob)| (*vcs, *prob / total)).collect()
    }

    fn vcsd_expectation(&self) -> f64 {
        self.vcs_dist.iter().fold(0.0, |acc, (vcs, prob)| acc + *vcs as f64 * *prob)
    }

    pub fn oa_expectation(&self) -> f64 {
        self.overalloc_dist().iter().fold(0.0, |acc, (oa, prob)| acc + *oa as f64 * *prob)
    }

    pub fn oa_expectation_renormalized(&self) -> f64 {
        self.overalloc_dist_renormalized().iter().fold(0.0, |acc, (oa, prob)| acc + *oa as f64 * *prob)
    }



    pub fn efficiency(&self) -> f64 {
        //probability of being underallocated
        let p_ua = self.vcs_dist.iter()
            .fold(0.0, |acc, (vcs, prob)| acc + if *vcs <= self.pcs {prob} else {&0.0});
        //p(oa) = 1-p(ua)
        let p_oa = 1.0 - p_ua;
        //this is P(VCS=PCS)*mult_{i=1}^n (1-p_i)
        let p_oa_given_ua = self.vcs_dist.get(&self.pcs)
            .unwrap_or(&0.0) * self.td.iter()
                .fold(1.0, |acc, (tenancy, prob)| acc * (1.0 - *prob));
        //apply bayes rule to the previous
        let p_ua_given_oa = p_oa_given_ua * p_ua / p_oa;
        //p(ua|ua) = 1 - p(oa|ua)
        let p_ua_given_ua = 1.0 - p_oa_given_ua;
        //p(oa|oa) = 1 - p(ua|oa)
        let p_oa_given_oa = 1.0-p_ua_given_oa;
        let markov_model = markov_model
            ::new(p_ua_given_ua, p_oa_given_ua, p_ua_given_oa, p_oa_given_oa);
        let mut total_visits_to_oa = 1.0;
        //WE ARE FLOORING TENANCY REAMINGING GIVEN FE COULD BE A SOURCE OF ERROR
        (1..self.tenancy_remaining_given_fe() as i64).for_each(|i| {
            total_visits_to_oa += markov_model.transition(i)[(1,1)];
        });
        total_visits_to_oa/self.tenancy_remaining_given_fe()
    }

    pub fn get_results(&self) -> (f64, f64, f64, u64, f64, f64) {
        // self.print_oa_dist();
        let efficiency = self.efficiency();
        let overage = self.oa_expectation();
        let unstored = self.unstored_per_access();
        let overage_normalized = self.oa_expectation_renormalized();
        (overage, unstored, overage/unstored, self.pcs, overage_normalized, efficiency)
    }

}

fn tenancy_expectation(td: &Vec<(u64, f64)>) -> u64 {
    let expectation = td.iter().fold(0.0, |acc, (tenancy, prob)| acc + *tenancy as f64 * prob);
    if (expectation as u64) as f64 == expectation {expectation as u64} else {(expectation.floor() + 1.0) as u64}
}