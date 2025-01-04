#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct AutoscaleSettingProfile {
    /// A `capacity` block as defined below.
    #[builder(into)]
    #[serde(rename = "capacity")]
    pub r#capacity: Box<super::super::types::monitoring::AutoscaleSettingProfileCapacity>,
    /// A `fixed_date` block as defined below. This cannot be specified if a `recurrence` block is specified.
    #[builder(into, default)]
    #[serde(rename = "fixedDate")]
    pub r#fixed_date: Box<Option<super::super::types::monitoring::AutoscaleSettingProfileFixedDate>>,
    /// Specifies the name of the profile.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// A `recurrence` block as defined below. This cannot be specified if a `fixed_date` block is specified.
    #[builder(into, default)]
    #[serde(rename = "recurrence")]
    pub r#recurrence: Box<Option<super::super::types::monitoring::AutoscaleSettingProfileRecurrence>>,
    /// One or more (up to 10) `rule` blocks as defined below.
    #[builder(into, default)]
    #[serde(rename = "rules")]
    pub r#rules: Box<Option<Vec<super::super::types::monitoring::AutoscaleSettingProfileRule>>>,
}
