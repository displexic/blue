use crate::git::GitConfig;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct RootWorkspaceConfig {
    pub name: Option<String>,
    pub clean_files: Option<Vec<String>>,
    pub prefered_config_file_type: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct JavaScriptConfig {
    pub workspace: Vec<String>,
    pub package_manager: String,
}

#[derive(Serialize, Deserialize)]
pub struct RustConfig {
    pub workspace: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct DockerComposeProfile {
    pub name: String,
    pub file: String,
}

#[derive(Serialize, Deserialize)]
pub struct DockerComposeConfig {
    pub profiles: Vec<DockerComposeProfile>,
}

#[derive(Serialize, Deserialize)]
pub struct DockerConfig {
    pub compose: DockerComposeConfig,
}

#[derive(Serialize, Deserialize)]
pub struct EnvTemplate {
    pub file: String,
    pub dest: String,
}

#[derive(Serialize, Deserialize)]
pub struct EnvConfig {
    pub delegate: bool,
    pub templates: Vec<EnvTemplate>,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub docker: Option<DockerConfig>,
    pub env: Option<EnvConfig>,
    pub git: Option<GitConfig>,
    pub javascript: Option<JavaScriptConfig>,
    pub rust: Option<RustConfig>,
    pub workspace: Option<RootWorkspaceConfig>,
}
