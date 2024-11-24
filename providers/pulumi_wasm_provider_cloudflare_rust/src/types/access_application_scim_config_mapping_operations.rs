#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct AccessApplicationScimConfigMappingOperations {
    /// Whether or not this mapping applies to create (POST) operations.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "create")]
    pub r#create: Box<Option<bool>>,
    /// Whether or not this mapping applies to DELETE operations.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "delete")]
    pub r#delete: Box<Option<bool>>,
    /// Whether or not this mapping applies to update (PATCH/PUT) operations.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "update")]
    pub r#update: Box<Option<bool>>,
}
