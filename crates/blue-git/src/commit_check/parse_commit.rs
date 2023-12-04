#[derive(Debug)]
pub struct CommitHeader {
    pub content: String,
    pub commit_type: String,
    pub scope: String,
    pub subject: String,
}
#[derive(Debug)]
pub struct Commit {
    pub header: CommitHeader,
    pub body: Vec<String>,
    pub footer: Vec<String>,
}

impl Commit {
    pub fn parse_message(message: &mut String) -> Commit {
        let mut body = String::new();
        let mut footer = String::new();

        let binding = message.replace("\\\\", "\\");
        let sections: Vec<&str> = binding.split("\n").collect();

        let header = sections
            .get(0)
            .map(|header_section| header_section.to_string())
            .unwrap_or_else(|| {
                eprintln!("No header found in commit message");
                std::process::exit(1);
            });

        if sections.len() > 1 {
            // Combine body sections, leaving the last one as the footer
            let body_sections = &sections[1..sections.len() - 1];
            for (index, section) in body_sections.iter().enumerate() {
                body.push_str(section);
                if index < body_sections.len() - 1 {
                    body.push_str("\n"); // Add newline between body sections
                }
            }
            footer = sections.last().unwrap_or(&"").to_string();
        }

        let commit_header = parse_header(header);
        let commit_body = parse_body(body);
        let commit_footer = parse_footer(footer);

        Commit {
            header: commit_header,
            body: commit_body,
            footer: commit_footer,
        }
    }
}

fn parse_header(header: String) -> CommitHeader {
    let commit_split = header.split(":").collect::<Vec<&str>>();

    if commit_split.len() < 2 {
        eprintln!("Invalid commit header. Required format: <type>([scope]): <subject>");
        std::process::exit(1);
    }

    let commit_type_scope = commit_split[0];

    let commit_type = commit_type_scope.split("(").collect::<Vec<&str>>()[0].to_string();
    let scope = commit_type_scope
        .split("(")
        .collect::<Vec<&str>>()
        .get(1)
        .unwrap_or(&"")
        .split(")")
        .collect::<Vec<&str>>()[0]
        .to_string();

    let subject = commit_split[1].trim().to_string();

    CommitHeader {
        content: header,
        commit_type,
        scope,
        subject,
    }
}

fn parse_body(body: String) -> Vec<String> {
    let mut body_lines = Vec::new();
    for line in body.lines() {
        body_lines.push(line.to_string());
    }
    body_lines
}

fn parse_footer(footer: String) -> Vec<String> {
    let mut footer_lines = Vec::new();
    for line in footer.lines() {
        footer_lines.push(line.to_string());
    }
    footer_lines
}
