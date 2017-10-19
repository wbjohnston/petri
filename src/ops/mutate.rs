//! Operators for mutating genomes

use genome::Genome;

use std::ops::Not;

use rand::Rng;

/// Operator for mutating a genome
pub trait MutateOperator<G, C>
where
    G: Genome<C>,
    C: Clone + Sized,
{
    /// Mutate an indiviudal
    fn mutate<R: Rng>(&self, indv: &G, rng: &mut R) -> G;
}


#[derive(Debug, Serialize, Deserialize)]
pub struct FlipBit {
    pub ind_pb: f32,
}

/// This operator will work on any genome with an invertible chromsome
impl<C> MutateOperator<Vec<C>, C> for FlipBit
where
    C: Clone + Sized + Not<Output = C>,
{
    /// Mutate an indiviudal
    fn mutate<R: Rng>(&self, indv: &Vec<C>, rng: &mut R) -> Vec<C>
    {
        indv.iter()
            .cloned()
            .map(|x| {
                let pb = (self.ind_pb * 100.0) as u32;
                let should_mut = rng.gen_weighted_bool(pb);
                if should_mut { !x } else { x }
            })
            .collect()
    }
}
