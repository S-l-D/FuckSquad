use crate::entity::solution::{Solution, SolutionHistory};
use common::dao::{DaoTrait, Validate};

pub struct SolutionDao {}

impl DaoTrait<Solution> for SolutionDao {}
impl Validate<Solution, SolutionHistory> for SolutionDao {}
    