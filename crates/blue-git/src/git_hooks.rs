use std::{fs, path::PathBuf};

pub enum GitHookType {
    PreCommit,
    CommitMsg,
    PrePush,
}

pub struct GitHook {
    hook_type: GitHookType,
    path: String,
}

const DEFAULT_GIT_HOOKS_PATH: &str = ".blue/hooks";

impl GitHook {
    pub fn new(hook_type: GitHookType, hooks_path: Option<String>) -> Self {
        let hooks_path = match hooks_path {
            Some(path) => path,
            None => DEFAULT_GIT_HOOKS_PATH.to_string(),
        };
        GitHook {
            hook_type,
            path: hooks_path,
        }
    }

    pub fn init(&self) {
        fs::create_dir_all(&self.path).unwrap_or_else(|err| {
            tracing::error!("{}", err);
            tracing::error!("Unable to create directory: {}", self.path);
            std::process::exit(1);
        });
    }

    pub fn create(&self, contents: &str) {
        self.init();

        let filename = match self.hook_type {
            GitHookType::PreCommit => "pre-commit",
            GitHookType::CommitMsg => "commit-msg",
            GitHookType::PrePush => "pre-push",
        };

        let file_path = PathBuf::from(format!("{}/{}", self.path, filename));
        fs::write(&file_path, contents).unwrap_or_else(|err| {
            tracing::error!("{}", err);
            tracing::error!("Unable to write file: {}", file_path.display());
            std::process::exit(1);
        });
    }
}
