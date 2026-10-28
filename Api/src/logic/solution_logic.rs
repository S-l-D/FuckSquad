use crate::dao::solution_dao::SolutionDao;
use crate::entity::solution::Solution;
use crate::logic::solution_files::SolutionFiles;
use actix_web::web;
use common::config::app_data::AppData;
use common::dao::DaoTrait;
use common::model::status_message::StatusMessage;
use tetrislib::dto::model::Layout;
use uuid::Uuid;

pub struct SolutionLogic {}

impl SolutionLogic {
    pub fn create(
        app_data: web::Data<AppData>,
        mut entity: Solution,
    ) -> Result<Solution, StatusMessage> {
        let files = SolutionFiles::default();
        let id = entity.uuid;
        files.create(id)?;
        entity.input_path = files.source(id).to_string_lossy().into_owned();
        entity.output_path = None;
        let conn = app_data.conn();
        SolutionDao::create(conn, entity, Uuid::nil()).map_err(|e| {
            // Only remove the empty directory we just created, never existing data.
            let _ = std::fs::remove_dir(files.directory(id));
            eprintln!("{e}");
            StatusMessage::make_internal_server_error_message(
                format!("Create entity error!").as_str(),
            )
        })
    }

    pub fn update(
        app_data: web::Data<AppData>,
        mut entity: Solution,
    ) -> Result<Solution, StatusMessage> {
        let existing = Self::find(app_data.clone(), entity.uuid)?;
        entity.input_path = existing.input_path;
        entity.output_path = existing.output_path;
        let conn = app_data.conn();
        SolutionDao::update(conn, entity, Uuid::nil()).map_err(|e| {
            eprintln!("{e}");
            StatusMessage::make_internal_server_error_message(
                format!("Update entity error!").as_str(),
            )
        })
    }
    pub fn delete(app_data: web::Data<AppData>, id: Uuid) -> Result<Solution, StatusMessage> {
        let conn = app_data.conn();
        SolutionDao::delete(conn, id, Uuid::nil()).map_err(|e| {
            eprintln!("{e}");
            StatusMessage::make_internal_server_error_message(
                format!("Delete entity error!").as_str(),
            )
        })
    }

    pub fn find(app_data: web::Data<AppData>, id: Uuid) -> Result<Solution, StatusMessage> {
        let conn = app_data.conn();
        SolutionDao::find_active(conn, id)
            .map_err(|e| {
                eprintln!("{e}");
                StatusMessage::make_internal_server_error_message(
                    format!("Find entity error!").as_str(),
                )
            })?
            .ok_or_else(|| StatusMessage::make_not_found_message("Solution not found in database"))
    }

    pub fn upload_source(
        app_data: web::Data<AppData>,
        id: Uuid,
        bytes: &[u8],
    ) -> Result<Layout, StatusMessage> {
        let layout = SolutionFiles::parse_source(bytes)?;
        Self::find(app_data.clone(), id)?;
        let files = SolutionFiles::default();
        files.save_source(id, &layout)?;
        SolutionDao::set_source_path(app_data.conn(), id, &files.source(id).to_string_lossy())
            .map_err(|error| {
                eprintln!("{error}");
                StatusMessage::make_internal_server_error_message("Cannot update solution source")
            })?;
        Ok(layout)
    }

    pub fn read_layout(
        app_data: web::Data<AppData>,
        id: Uuid,
        output: bool,
    ) -> Result<Layout, StatusMessage> {
        Self::find(app_data, id)?;
        SolutionFiles::default().read(id, output)
    }
}
