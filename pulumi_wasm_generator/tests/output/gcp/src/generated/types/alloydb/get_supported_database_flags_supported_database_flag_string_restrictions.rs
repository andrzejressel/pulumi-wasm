#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetSupportedDatabaseFlagsSupportedDatabaseFlagStringRestrictions {
    /// The list of allowed values, if bounded. This field will be empty if there is a unbounded number of allowed values.
    #[builder(into)]
    #[serde(rename = "allowedValues")]
    pub r#allowed_values: Box<Vec<String>>,
}
