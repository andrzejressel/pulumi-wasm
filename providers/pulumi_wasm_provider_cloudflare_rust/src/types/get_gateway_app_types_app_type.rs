#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct GetGatewayAppTypesAppType {
    /// The identifier for the application type of this app.
    #[builder(into)]
    #[serde(rename = "applicationTypeId")]
    pub r#application_type_id: Box<i32>,
    /// A short summary of the app type.
    #[builder(into)]
    #[serde(rename = "description")]
    pub r#description: Box<String>,
    /// The identifier for this app type. There is only one app type per ID.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Box<i32>,
    /// The name of the app type.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
}