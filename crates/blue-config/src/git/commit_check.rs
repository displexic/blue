#[derive(serde::Deserialize)]
pub enum Level {
    #[serde(rename = "disabled")]
    Disabled,
    #[serde(rename = "warning")]
    Warning,
    #[serde(rename = "error")]
    Error,
}

#[derive(serde::Deserialize)]
pub enum Rule {
    #[serde(rename = "always")]
    Always,
    #[serde(rename = "never")]
    Never,
}

#[derive(serde::Deserialize)]
pub enum Case {
    /// lowercase
    #[serde(rename = "lower-case")]
    Lower,
    /// UPPERCASE
    #[serde(rename = "upper-case")]
    Upper,
    /// camelCase
    #[serde(rename = "camel-case")]
    Camel,
    /// kebab-case
    #[serde(rename = "kebab-case")]
    Kebab,
    /// PascalCase  
    #[serde(rename = "pascal-case")]
    Pascal,
    /// Sentence case
    #[serde(rename = "sentence-case")]
    Sentence,
    /// snake_case
    #[serde(rename = "snake")]
    Snake,
    /// Start Case
    #[serde(rename = "start-case")]
    Start,
}

#[derive(serde::Deserialize)]
pub struct CheckCommitConfig {
    pub preset: Option<String>,
    pub rules: Option<CommitCheckRules>,
}

#[derive(serde::Deserialize)]
pub struct DefaultRule {
    pub level: Level,
    pub rule: Rule,
}
#[derive(serde::Deserialize)]
pub struct NumberRule {
    pub level: Level,
    pub rule: Rule,
    pub value: usize,
}
#[derive(serde::Deserialize)]
pub struct CaseRule {
    pub level: Level,
    pub rule: Rule,
    pub value: Vec<Case>,
}
#[derive(serde::Deserialize)]
pub struct StringRule {
    pub level: Level,
    pub rule: Rule,
    pub value: String,
}
#[derive(serde::Deserialize)]
pub struct StringListRule {
    pub level: Level,
    pub rule: Rule,
    pub value: Vec<String>,
}

pub type CommitCheckRules = CommitCheckRulesBase<
    Option<DefaultRule>,
    Option<NumberRule>,
    Option<CaseRule>,
    Option<StringRule>,
    Option<StringListRule>,
>;

type PresetDefaultRule = (Level, Rule);
type PresetNumberRule = (Level, Rule, usize);
type PresetCaseRule = (Level, Rule, Vec<Case>);
type PresetStringRule = (Level, Rule, String);
type PresetStringListRule = (Level, Rule, Vec<String>);

pub type PresetCommitCheckRules = CommitCheckRulesBase<
    PresetDefaultRule,
    PresetNumberRule,
    PresetCaseRule,
    PresetStringRule,
    PresetStringListRule,
>;

#[derive(serde::Deserialize)]
pub struct CommitCheckRulesBase<DR, NR, CR, SR, SLR> {
    /// Body must end with a full stop
    pub body_full_stop: DR,
    /// Body must start with a black line
    pub body_leading_blank: DR,
    /// Body must be empty
    pub body_empty: DR,
    /// Body must be longer than the value
    pub body_max_length: NR,
    /// Body must be shorter than the value
    pub body_min_length: NR,
    /// Body must be longer than the value
    pub body_max_line_length: NR,
    /// Body must be shorter than the value
    pub body_min_line_length: NR,
    /// Body must be longer than the value
    pub body_max_lines: NR,
    /// Body must be in the specified case
    pub body_case: CR,

    /// Footer must begin with a black line
    pub footer_leading_blank: DR,
    /// Footer must be empty
    pub footer_empty: DR,
    /// Footer must be longer than the value
    pub footer_max_length: NR,
    /// Footer must be longer than the value
    pub footer_max_line_length: NR,

    /// Header must be in the specified case
    pub header_case: CR,
    /// Header must end with a full stop
    pub header_full_stop: DR,
    /// Header must be shorter than the value
    pub header_max_length: NR,
    /// Header must be longer than the value
    pub header_min_length: NR,

    /// References has at least one entry
    pub references_empty: DR,

    /// Scope must be found in the value
    pub scope_enum: SLR,
    /// Scope must be in the specified case
    pub scope_case: CR,
    /// Scope must be empty
    pub scope_empty: DR,
    /// Scope must be shorter than the value
    pub scope_max_length: NR,
    /// Scope must be longer than the value
    pub scope_min_length: NR,

    /// Subject must be in the specified case
    pub subject_case: CR,
    /// Subject must be empty
    pub subject_empty: DR,
    /// Subject must end with a full stop
    pub subject_full_stop: DR,
    /// Subject must be shorter than the value
    pub subject_max_length: NR,
    /// Subject must be longer than the value
    pub subject_min_length: NR,
    /// Subject must have an exclamation mark before the : marker
    pub subject_exclamation: DR,

    /// Type must be found in the value
    pub type_enum: SLR,
    /// Type must be in the specified case
    pub type_case: CR,
    /// Type must be empty
    pub type_empty: DR,
    /// Type must be shorter than the value
    pub type_max_length: NR,
    /// Type must be longer than the value
    pub type_min_length: NR,

    /// Message must contain "Signed-off-by" or value
    pub signed_off_by: SR,
    /// Message must have trailer value
    pub trailer: SR,
}
