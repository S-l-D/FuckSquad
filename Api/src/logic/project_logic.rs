use crate::dao::project_dao::ProjectDao;
use crate::entity::project::Project;
use common::logic::LogicTrait;

pub struct ProjectLogic {}

impl LogicTrait<Project, ProjectDao> for ProjectLogic {}
    