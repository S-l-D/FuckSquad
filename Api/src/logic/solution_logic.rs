use crate::dao::solution_dao::SolutionDao;
use crate::entity::solution::Solution;
use common::logic::LogicTrait;

pub struct SolutionLogic {}

impl LogicTrait<Solution, SolutionDao> for SolutionLogic {}
    