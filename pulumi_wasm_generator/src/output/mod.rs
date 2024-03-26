use regex::Regex;
use crate::model::ElementId;

pub(crate) mod wit;
pub(crate) mod provider;

fn replace_multiple_dashes(s: &String) -> String {
    let re = Regex::new("-+").unwrap();
    let result = re.replace_all(s, "-");
    result.to_string()
}