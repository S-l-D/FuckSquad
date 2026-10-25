use uuid::Uuid;
use tetrislib::dto::project_request::{ProjectRequest, SolutionRequest};
use tetrislib::dto::project_response::{ProjectResponse, SolutionResponse};

pub struct Project {
    pub uuid: Uuid,
    pub name: String,
    pub description: String,
    pub solution: Vec<Solution>,
}

pub struct Solution {
    pub uuid: Uuid,
    pub input_path: String, // pointer to file with unplaced layout
    pub output_path: Option<String>, // pointer to file with layout
    pub name: String,
    pub version: String,
}

impl Solution {
    pub fn from_request(request: &SolutionRequest) -> Solution {
        Solution {
            uuid: request.id.unwrap_or(Uuid::new_v4()),
            input_path: request.input_path.clone(),
            output_path: request.output_path.clone(),
            name: request.name.clone(),
            version: request.version.clone(),
        }
    }

    pub fn to_response(&self) -> SolutionResponse {
        SolutionResponse {
            id: self.uuid.clone(),
            input_path: self.input_path.clone(),
            output_path: self.output_path.clone(),
            name: self.name.clone(),
            version: self.version.clone(),
        }
    }
}

impl Project {
    pub fn from_request(req: &ProjectRequest) -> Project {
        Project {
            uuid: req.id.unwrap_or(Uuid::new_v4()).clone(),
            name: req.name.clone(),
            description: req.description.clone().clone(),
            solution: req.solution.iter().map(|req| Solution::from_request(req)).collect(),
        }
    }

    pub fn to_response(&self) -> ProjectResponse {
        ProjectResponse {
            id: self.uuid.clone(),
            name: self.name.clone(),
            description: self.description.clone(),
            solution: self.solution.iter().map(|ent| ent.to_response()).collect(),
        }
    }

    pub fn to_response_list(values: Vec<Project>) -> Vec<ProjectResponse> {
        values.into_iter().map(|ent| Project::to_response(&ent)).collect()
    }
}