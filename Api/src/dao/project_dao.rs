use crate::entity::project::{Project, ProjectHistory};
use common::dao::Connection;
use common::dao::{DaoTrait, Validate};
use common::entity::{EntityTrait, FromRow};
use common::enums::r_status::RStatus;

pub struct ProjectDao {}

impl ProjectDao {
    pub fn all(conn: &mut Connection) -> Result<Vec<Project>, postgres::Error> {
        let query = format!(
            "SELECT * FROM {} WHERE rstatus = $1 ORDER BY name, uuid",
            Project::get_table_name()
        );
        conn.query(&query, &[&RStatus::Active])
            .map(Project::from_vec_row)
    }
}

impl DaoTrait<Project> for ProjectDao {}
impl Validate<Project, ProjectHistory> for ProjectDao {}
