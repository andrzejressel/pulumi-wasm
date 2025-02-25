use serde::Deserialize;

#[derive(Debug, Deserialize)]
enum ChangelogType {
    Added,
    Changed,
    Deprecated,
    Removed,
    Fixed,
    Security,
}

#[derive(Debug, Deserialize)]
struct ChangelogEntry {
    r#type: ChangelogType,
    title: String,
}
