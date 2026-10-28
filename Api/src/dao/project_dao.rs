use crate::entity::project::{Project, ProjectHistory};
use common::dao::Connection;
use common::dao::{DaoTrait, Validate};
use common::entity::{EntityTrait, FromRow};
use common::enums::r_status::RStatus;

pub struct ProjectDao {}

impl ProjectDao {
    pub fn delete_with_solutions(
        mut conn: Connection,
        id: uuid::Uuid,
    ) -> Result<Project, postgres::Error> {
        let mut transaction = conn.transaction()?;
        let now = chrono::Utc::now().naive_utc();
        let user = uuid::Uuid::nil();
        let row = transaction.query_one(
            &format!("UPDATE {} SET rstatus = $1, updt = $2, updu = $3, ver = ver + 1 WHERE uuid = $4 RETURNING *", Project::get_table_name()),
            &[&RStatus::Removed, &now, &user, &id],
        )?;
        transaction.execute(
            &format!("UPDATE {} SET rstatus = $1, updt = $2, updu = $3, ver = ver + 1 WHERE project = $4 AND rstatus != $1", crate::entity::solution::Solution::get_table_name()),
            &[&RStatus::Removed, &now, &user, &id],
        )?;
        transaction.commit()?;
        Ok(Project::from_row(row))
    }

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
