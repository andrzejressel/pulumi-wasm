#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetReceivedLicenseEntitlement {
    /// Indicates whether check-ins are allowed.
    #[builder(into)]
    #[serde(rename = "allowCheckIn")]
    pub r#allow_check_in: Box<bool>,
    /// Maximum entitlement count. Use if the unit is not None.
    #[builder(into)]
    #[serde(rename = "maxCount")]
    pub r#max_count: Box<i32>,
    /// The key name.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Indicates whether overages are allowed.
    #[builder(into)]
    #[serde(rename = "overage")]
    pub r#overage: Box<bool>,
    /// Entitlement unit.
    #[builder(into)]
    #[serde(rename = "unit")]
    pub r#unit: Box<String>,
    /// The value.
    #[builder(into)]
    #[serde(rename = "value")]
    pub r#value: Box<String>,
}
