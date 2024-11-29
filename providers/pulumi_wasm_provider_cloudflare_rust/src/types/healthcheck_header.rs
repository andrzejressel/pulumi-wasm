#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct HealthcheckHeader {
    /// The header name.
    #[builder(into)]
    #[serde(rename = "header")]
    pub r#header: Box<String>,
    /// A list of string values for the header.
    #[builder(into)]
    #[serde(rename = "values")]
    pub r#values: Box<Vec<String>>,
}
