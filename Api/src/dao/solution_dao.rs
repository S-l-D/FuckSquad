use crate::entity::solution::{Solution, SolutionHistory};
use common::dao::Connection;
use common::dao::{DaoTrait, Validate};
use common::entity::{EntityTrait, FromRow};
use common::enums::r_status::RStatus;
use uuid::Uuid;

pub struct SolutionDao {}

impl SolutionDao {
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
