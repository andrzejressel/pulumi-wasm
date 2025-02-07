#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct VMwareNodePoolConfigTaint {
    /// Available taint effects.
    /// Possible values are: `EFFECT_UNSPECIFIED`, `NO_SCHEDULE`, `PREFER_NO_SCHEDULE`, `NO_EXECUTE`.
    #[builder(into, default)]
    #[serde(rename = "effect")]
    pub r#effect: Box<Option<String>>,
    /// Key associated with the effect.
    #[builder(into)]
    #[serde(rename = "key")]
    pub r#key: Box<String>,
    /// Value associated with the effect.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}
