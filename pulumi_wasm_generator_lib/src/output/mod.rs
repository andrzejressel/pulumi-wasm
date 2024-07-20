use regex::Regex;

pub(crate) mod provider;
pub(crate) mod rust;
pub(crate) mod wit;

fn get_main_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
}

fn get_main_version_stringify() -> String {
    env!("CARGO_PKG_VERSION")
        .replace(".", "")
        .replace("0", "ZERO")
        .replace("1", "ONE")
        .replace("2", "TWO")
        .replace("3", "THREE")
        .replace("4", "FOUR")

}

fn replace_multiple_dashes(s: &str) -> String {
    let re = Regex::new("-+").unwrap();
    let result = re.replace_all(s, "-");
    result.to_string()
}
