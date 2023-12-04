pub mod commit_check;
pub mod git_hooks;

pub fn setup() {
    commit_check::init_git_hooks();
}
