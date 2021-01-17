use serde::Serialize;

use crate::toggl::Project;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AlfredText {
    copy: String,
    large_type: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AlfredItem {
    title: String,
    subtitle: String,
    arg: String,
    text: AlfredText,
}

impl AlfredItem {
    pub fn from_project(project: &Project) -> Self {
        Self {
            title: project.name.clone(),
            subtitle: project.id.to_string(),
            arg: project.id.to_string(),
            text: AlfredText {
                copy: project.name.clone(),
                large_type: project.name.clone(),
            },
        }
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AlfredFormat {
    pub items: Vec<AlfredItem>,
}

impl AlfredFormat {
    pub fn from_projects(projects: &Vec<Project>) -> Self {
        let mut items: Vec<AlfredItem> = vec![];
        for project in projects {
            items.push(AlfredItem::from_project(&project));
        }

        Self { items }
    }
}
