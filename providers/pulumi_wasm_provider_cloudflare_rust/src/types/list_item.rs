#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct ListItem {
    /// An optional comment for the item.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "comment")]
    pub r#comment: Box<Option<String>>,
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Box<crate::types::ListItemValue>,
}
