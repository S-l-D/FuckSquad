use crate::entity::project::{Project, ProjectHistory};
use common::dao::{DaoTrait, Validate};

pub struct ProjectDao {}

impl DaoTrait<Project> for ProjectDao {}
impl Validate<Project, ProjectHistory> for ProjectDao {}
    