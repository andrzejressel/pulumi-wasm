#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct CustomHostnameSslValidationError {
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "message")]
    pub r#message: Box<Option<String>>,
}
