use common::dao::{Connection, Validate};

pub mod project_dao;
pub mod solution_dao;

pub fn validate_all(conn: &mut Connection) {
    project_dao::ProjectDao::validate(conn);
    solution_dao::SolutionDao::validate(conn);
}