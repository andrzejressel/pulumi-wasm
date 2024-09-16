#[derive(serde::Deserialize, serde::Serialize, bon::Builder, Debug)]
#[builder(finish_fn = build_struct)]
pub struct GetRulesetsRulesetRuleActionParametersSni {
    /// Value to define for SNI.
    #[builder(into, default = Box::new(None))]
    #[serde(rename = "value")]
    pub r#value: Box<Option<String>>,
}
