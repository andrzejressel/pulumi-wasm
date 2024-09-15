#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct AccessPolicyIncludeExternalEvaluation {
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "evaluateUrl")]
    pub r#evaluate_url: Box<Option<String>>,
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "keysUrl")]
    pub r#keys_url: Box<Option<String>>,
}
