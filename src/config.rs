use serde::Deserialize;
use std::{collections::HashMap, fs, path::Path};

#[derive(Debug, Deserialize)]
pub struct Config {
    pub types: HashMap<String, String>,
    pub shell: Option<String>,
    pub common: Option<CommonSteps>,
    pub new: Option<HashMap<String, ProjectSteps>>,
    pub clone: Option<HashMap<String, ProjectSteps>>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct CommonSteps {
    #[serde(flatten)]
    pub phases: Phases,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ProjectSteps {
    pub dir: Option<String>,
    #[serde(flatten)]
    pub phases: Phases,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Phases {
    pub pre: Option<Vec<Step>>,
    pub main: Option<Vec<Step>>,
    pub post: Option<Vec<Step>>,
}

#[derive(Debug, Deserialize, Clone)]
#[serde(untagged)]
pub enum Step {
    Cmd(String),
    Clone { clone: Option<()> },
    GitInit { git_init: Option<()> },
}

pub fn merge_phases(common: Option<&Phases>, project: Option<&Phases>) -> Phases {
    Phases {
        pre: merge_vecs(
            common.and_then(|c| c.pre.as_ref()),
            project.and_then(|p| p.pre.as_ref()),
        ),
        main: merge_vecs(
            common.and_then(|c| c.main.as_ref()),
            project.and_then(|p| p.main.as_ref()),
        ),
        post: merge_vecs(
            common.and_then(|c| c.post.as_ref()),
            project.and_then(|p| p.post.as_ref()),
        ),
    }
}

fn merge_vecs(a: Option<&Vec<Step>>, b: Option<&Vec<Step>>) -> Option<Vec<Step>> {
    match (a, b) {
        (Some(a), Some(b)) => Some([a.clone(), b.clone()].concat()),
        (Some(a), None) => Some(a.clone()),
        (None, Some(b)) => Some(b.clone()),
        (None, None) => None,
    }
}

pub fn load_config<P: AsRef<Path>>(path: P) -> Result<Config, Box<dyn std::error::Error>> {
    let yaml = fs::read_to_string(path)?;
    let config: Config = serde_yaml::from_str(&yaml)?;
    Ok(config)
}
