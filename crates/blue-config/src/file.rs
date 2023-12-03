use crate::git::GitConfig;

#[derive(serde::Deserialize)]
pub struct RootWorkspaceConfig {
    pub name: Option<String>,
    pub clean_files: Option<Vec<String>>,
    pub prefered_config_file_type: Option<String>,
}

#[derive(serde::Deserialize)]
pub struct JavaScriptConfig {
    pub workspace: Vec<String>,
    pub package_manager: String,
}

#[derive(serde::Deserialize)]
pub struct RustConfig {
    pub workspace: Vec<String>,
}

#[derive(serde::Deserialize)]
pub struct DockerComposeProfile {
    pub name: String,
    pub file: String,
}

#[derive(serde::Deserialize)]
pub struct DockerComposeConfig {
    pub profiles: Vec<DockerComposeProfile>,
}

#[derive(serde::Deserialize)]
pub struct DockerConfig {
    pub compose: DockerComposeConfig,
}

#[derive(serde::Deserialize)]
pub struct EnvTemplate {
    pub file: String,
    pub dest: String,
}

#[derive(serde::Deserialize)]
pub struct EnvConfig {
    pub delegate: bool,
    pub templates: Vec<EnvTemplate>,
}

#[derive(serde::Deserialize)]
pub struct Config {
    pub docker: Option<DockerConfig>,
    pub env: Option<EnvConfig>,
    pub git: Option<GitConfig>,
    pub javascript: Option<JavaScriptConfig>,
    pub rust: Option<RustConfig>,
    pub workspace: Option<RootWorkspaceConfig>,
}
