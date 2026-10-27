use actix_web::web;
use common::config::app_data::AppData;
use common::dao::DaoTrait;
use crate::dao::solution_dao::SolutionDao;
use crate::entity::solution::Solution;
use common::model::status_message::StatusMessage;
use uuid::Uuid;

pub struct SolutionLogic {}

impl SolutionLogic {
    pub fn create(app_data: web::Data<AppData>, entity: Solution) -> Result<Solution, StatusMessage> {
        let conn = app_data.conn();
        SolutionDao::create(conn, entity, Uuid::nil())
            .map_err(|e| {
                eprintln!("{e}");
                StatusMessage::make_internal_server_error_message(format!("Create entity error!").as_str())
            })
    }

    pub fn update(app_data: web::Data<AppData>, entity: Solution) -> Result<Solution, StatusMessage> {
        let conn = app_data.conn();
        SolutionDao::update(conn, entity, Uuid::nil())
            .map_err(|e| {
                eprintln!("{e}");
                StatusMessage::make_internal_server_error_message(format!("Update entity error!").as_str())
            })
    }
    pub fn delete(app_data: web::Data<AppData>, id: Uuid) -> Result<Solution, StatusMessage> {
        let conn = app_data.conn();
        SolutionDao::delete(conn, id, Uuid::nil())
            .map_err(|e| {
                eprintln!("{e}");
                StatusMessage::make_internal_server_error_message(format!("Delete entity error!").as_str())
            })
    }

    pub fn find(app_data: web::Data<AppData>, id: Uuid) -> Result<Solution, StatusMessage> {
        let conn = app_data.conn();
        SolutionDao::find(conn, id)
            .map_err(|e| {
                eprintln!("{e}");
                if e.as_db_error() == None {
                    return StatusMessage::constructor("540", "Solution not found in database")
                }
                StatusMessage::make_internal_server_error_message(format!("Find entity error!").as_str())
            })
    }
}