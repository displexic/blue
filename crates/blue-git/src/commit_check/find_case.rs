use blue_config::git::commit_check::Case;

pub fn find_case(text: &String, cases: &Vec<Case>) -> bool {
    for case in cases {
        match case {
            Case::Lower => {
                if Case::Lower.compare(text) {
                    return true;
                }
            }
            Case::Upper => {
                if Case::Upper.compare(text) {
                    return true;
                }
            }
            Case::Camel => {
                if Case::Camel.compare(text) {
                    return true;
                }
            }
            Case::Kebab => {
                if Case::Kebab.compare(text) {
                    return true;
                }
            }
            Case::Pascal => {
                if Case::Pascal.compare(text) {
                    return true;
                }
            }
            Case::Sentence => {
                if Case::Sentence.compare(text) {
                    return true;
                }
            }
            Case::Snake => {
                if Case::Snake.compare(text) {
                    return true;
                }
            }
            Case::Start => {
                if Case::Start.compare(text) {
                    return true;
                }
            }
        }
    }

    false
}
