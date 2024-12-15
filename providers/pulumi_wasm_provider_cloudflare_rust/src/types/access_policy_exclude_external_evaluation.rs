#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
pub struct AccessPolicyExcludeExternalEvaluation {
    /// The API endpoint containing your business logic.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "evaluateUrl")]
    pub r#evaluate_url: Box<Option<String>>,
    /// The API endpoint containing the key that Access uses to verify that the response came from your API.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "keysUrl")]
    pub r#keys_url: Box<Option<String>>,
}
