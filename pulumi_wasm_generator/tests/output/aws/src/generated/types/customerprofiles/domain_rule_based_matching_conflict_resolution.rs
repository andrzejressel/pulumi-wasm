#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct DomainRuleBasedMatchingConflictResolution {
    /// How the auto-merging process should resolve conflicts between different profiles. Valid values are `RECENCY` and `SOURCE`
    #[builder(into)]
    #[serde(rename = "conflictResolvingModel")]
    pub r#conflict_resolving_model: Box<String>,
    /// The `ObjectType` name that is used to resolve profile merging conflicts when choosing `SOURCE` as the `ConflictResolvingModel`.
    #[builder(into, default)]
    #[serde(rename = "sourceName")]
    pub r#source_name: Box<Option<String>>,
}
