use crate::project::Project;

pub struct Platform {
    pub projects: Vec<Project>,
}

impl Platform {
    pub fn new() -> Self {
        Platform {
            projects: Vec::new(),
        }
    }

    pub fn add_project(&mut self, project: Project) {
        self.projects.push(project);
    }

    pub fn get_projects(&self) -> &Vec<Project> {
        &self.projects
    }
}
