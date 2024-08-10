use regex::Regex;

pub(crate) fn replace_multiple_dashes(s: &str) -> String {
    let re = Regex::new("-+").unwrap();
    let result = re.replace_all(s, "-");
    result.to_string()
}

pub(crate) fn escape_wit_identifier<'a>(s: &'a str) -> &'a str {
    match s {
        "result" => "%result",
        s => s,
    }
}
