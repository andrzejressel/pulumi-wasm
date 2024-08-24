#[derive(serde::Serialize)]
pub struct CustomSslCustomSslPriority {
    /// The ID of this resource.
    #[serde(rename = "id")]
    pub r#id: Box<Option<String>>,
    #[serde(rename = "priority")]
    pub r#priority: Box<Option<i32>>,
}
