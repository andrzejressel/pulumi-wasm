#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct CustomSslCustomSslPriority {
    /// The ID of this resource.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "priority")]
    pub r#priority: Box<Option<i32>>,
}
