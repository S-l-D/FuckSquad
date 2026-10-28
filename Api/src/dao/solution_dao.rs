use crate::entity::solution::{Solution, SolutionHistory};
use common::dao::Connection;
use common::dao::{DaoTrait, Validate};
use common::entity::{EntityTrait, FromRow};
use common::enums::r_status::RStatus;
use uuid::Uuid;

pub struct SolutionDao {}

impl SolutionDao {
    pub fn find_active(
        mut conn: Connection,
        id: Uuid,
    ) -> Result<Option<Solution>, postgres::Error> {
        let query = format!(
            "SELECT * FROM {} WHERE uuid = $1 AND rstatus = $2",
            Solution::get_table_name()
        );
        conn.query_opt(&query, &[&id, &RStatus::Active])
            .map(|row| row.map(Solution::from_row))
    }

    pub fn set_source_path(
        mut conn: Connection,
        id: Uuid,
        path: &str,
    ) -> Result<(), postgres::Error> {
        let query = format!(
            "UPDATE {} SET input_path = $1, output_path = NULL, updt = $2, ver = ver + 1 WHERE uuid = $3 AND rstatus = $4 RETURNING uuid",
            Solution::get_table_name()
        );
        conn.query_one(
            &query,
            &[
                &path,
                &chrono::Utc::now().naive_utc(),
                &id,
                &RStatus::Active,
            ],
        )?;
        Ok(())
    }

    pub fn for_projects(
        conn: &mut Connection,
        projects: &[Uuid],
    ) -> Result<Vec<Solution>, postgres::Error> {
        let query = format!(
            "SELECT * FROM {} WHERE project = ANY($1) AND rstatus = $2 ORDER BY name, uuid",
            Solution::get_table_name()
        );
        conn.query(&query, &[&projects, &RStatus::Active])
            .map(Solution::from_vec_row)
    }
}

impl DaoTrait<Solution> for SolutionDao {}
impl Validate<Solution, SolutionHistory> for SolutionDao {}
