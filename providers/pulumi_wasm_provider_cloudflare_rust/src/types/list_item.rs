#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct ListItem {
    /// An optional comment for the item.
    #[builder(into, default)]
    #[serde(rename = "comment")]
    pub r#comment: Box<Option<String>>,
    #[builder(into, default)]
    #[serde(rename = "value")]
    pub r#value: Box<Option<crate::types::ListItemValue>>,
}
