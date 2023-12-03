pub struct Commit {
    pub header: String,
    pub body: String,
    pub footer: String,
}

pub fn parse_commit(message: String) -> Commit {
    Commit {
        header: message,
        body: "".to_string(),
        footer: "".to_string(),
    }
}
