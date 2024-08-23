#[derive(serde::Serialize)]
pub struct ListItem {
    #[serde(rename = "comment")]
    pub r#comment: Box<Option<String>>,
    #[serde(rename = "value")]
    pub r#value: Box<crate::types::ListItemValue>,
}
