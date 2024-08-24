#[derive(serde::Serialize)]
pub struct ListItem {
    /// An optional comment for the item.
    #[serde(rename = "comment")]
    pub r#comment: Box<Option<String>>,
    #[serde(rename = "value")]
    pub r#value: Box<crate::types::ListItemValue>,
}
