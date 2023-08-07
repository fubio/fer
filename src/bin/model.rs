use std::collections::HashMap;
use crate::util::vcsd;

pub struct FER_calculator {
    td: Vec<(u64, f64)>,
    //this is just the expectation of the tenancy distribution
    pcs: u64,
    vcs_dist: HashMap<u64, f64>,
}

impl FER_calculator {
    pub fn new(td: Vec<(u64, f64)>) -> FER_calculator {
        let pcs = tenancy_expectation(&td);
        let vcs_dist = vcsd::generate_vcsd(td.iter().map(|(tenacy, prob)| (*tenacy, *prob)).collect::<HashMap<_, _>>());
        FER_calculator {
            td,
            pcs,
            vcs_dist,
        }
    }
    /*
    Calculates the remaining or unstored tenancy on average per access.
    This is slightly lower then what we want, what we really want is tenancy remainging when we VCS>PCS (a forced eviction takes place)
     */
    pub fn unstored_per_access(&self) -> f64 {
        self.td.iter().fold(0.0, |acc, (tenancy, prob)| acc + tenancy.pow(2) as f64 * *prob) / (self.pcs as f64 * 2.0)
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

    fn vcsd_expectation(&self) -> f64 {
        self.vcs_dist.iter().fold(0.0, |acc, (vcs, prob)| acc + *vcs as f64 * *prob)
    }

    pub fn oa_expectation(&self) -> f64 {
        self.overalloc_dist().iter().fold(0.0, |acc, (vcs, prob)| acc + *vcs as f64 * *prob)
    }

    pub fn get_results(&self) -> (f64, f64, f64, u64) {
        let overage = self.oa_expectation();
        let unstored = self.unstored_per_access();
        (overage, unstored, overage/unstored, self.pcs)
    }

}

fn tenancy_expectation(td: &Vec<(u64, f64)>) -> u64 {
    let expectation = td.iter().fold(0.0, |acc, (tenancy, prob)| acc + *tenancy as f64 * prob);
    if (expectation as u64) as f64 == expectation {expectation as u64} else {(expectation.floor() + 1.0) as u64}
}