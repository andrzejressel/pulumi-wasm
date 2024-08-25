#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct AccessPolicyRequireExternalEvaluation {
    #[serde(rename = "evaluateUrl")]
    pub r#evaluate_url: Box<Option<String>>,
    #[serde(rename = "keysUrl")]
    pub r#keys_url: Box<Option<String>>,
}
