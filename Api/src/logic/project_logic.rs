use crate::dao::project_dao::ProjectDao;
use crate::dao::solution_dao::SolutionDao;
use crate::entity::project::Project;
use crate::entity::solution::Solution;
use actix_web::web;
use common::config::app_data::AppData;
use common::model::status_message::StatusMessage;
use std::collections::HashMap;
use common::dao::DaoTrait;
use tetrislib::dto::project_response::ProjectResponse;
use tetrislib::dto::solution_response::SolutionResponse;
use uuid::Uuid;

pub struct ProjectLogic {}

impl ProjectLogic {
    pub fn create(app_data: web::Data<AppData>, entity: Project) -> Result<Project, StatusMessage> {
        let conn = app_data.conn();
        ProjectDao::create(conn, entity, Uuid::nil())
            .map_err(|e| {
                eprintln!("{e}");
                StatusMessage::make_internal_server_error_message(format!("Create entity error!").as_str())
            })
    }
    
    pub fn update(app_data: web::Data<AppData>, entity: Project) -> Result<Project, StatusMessage> {
        let conn = app_data.conn();
        ProjectDao::update(conn, entity, Uuid::nil())
            .map_err(|e| {
                eprintln!("{e}");
                StatusMessage::make_internal_server_error_message(format!("Update entity error!").as_str())
            })
    }
    pub fn delete(app_data: web::Data<AppData>, id: Uuid) -> Result<Project, StatusMessage> {
        let conn = app_data.conn();
        ProjectDao::delete(conn, id, Uuid::nil())
            .map_err(|e| {
                eprintln!("{e}");
                StatusMessage::make_internal_server_error_message(format!("Delete entity error!").as_str())
            })
    }

    pub fn find(app_data: web::Data<AppData>, id: Uuid) -> Result<Project, StatusMessage> {
        let conn = app_data.conn();
        ProjectDao::find(conn, id)
            .map_err(|e| {
                eprintln!("{e}");
                if e.as_db_error() == None {
                    return StatusMessage::constructor("540", "Project not found in database")
                }
                StatusMessage::make_internal_server_error_message(format!("Find entity error!").as_str())
            })
    }
}

impl ProjectLogic {
    pub fn all(app_data: web::Data<AppData>) -> Result<Vec<ProjectResponse>, StatusMessage> {
        let mut conn = app_data.conn();
        let projects = ProjectDao::all(&mut conn).map_err(|e| {
            eprintln!("{e}");
            StatusMessage::make_internal_server_error_message("List projects error!")
        })?;
        if projects.is_empty() {
            return Ok(Vec::new());
        }
        let ids: Vec<Uuid> = projects.iter().map(|project| project.uuid).collect();
        let solutions = SolutionDao::for_projects(&mut conn, &ids).map_err(|e| {
            eprintln!("{e}");
            StatusMessage::make_internal_server_error_message("List solutions error!")
        })?;
        Ok(Self::with_solutions(projects, solutions))
    }

    fn with_solutions(projects: Vec<Project>, solutions: Vec<Solution>) -> Vec<ProjectResponse> {
        let mut by_project: HashMap<Uuid, Vec<SolutionResponse>> = HashMap::new();
        for solution in solutions {
            by_project
                .entry(solution.project)
                .or_default()
                .push(solution.to_response());
        }
        projects
            .into_iter()
            .map(|project| {
                let mut response = project.to_response();
                response.solutions = by_project.remove(&project.uuid).unwrap_or_default();
                response
            })
            .collect()
    }
}
