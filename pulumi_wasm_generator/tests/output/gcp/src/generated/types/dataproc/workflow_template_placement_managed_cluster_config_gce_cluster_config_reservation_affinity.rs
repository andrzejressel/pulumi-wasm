#[derive(pulumi_wasm_rust::__private::serde::Deserialize, pulumi_wasm_rust::__private::serde::Serialize, pulumi_wasm_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct WorkflowTemplatePlacementManagedClusterConfigGceClusterConfigReservationAffinity {
    /// Type of reservation to consume Possible values: TYPE_UNSPECIFIED, NO_RESERVATION, ANY_RESERVATION, SPECIFIC_RESERVATION
    #[builder(into, default)]
    #[serde(rename = "consumeReservationType")]
    pub r#consume_reservation_type: Box<Option<String>>,
    /// Corresponds to the label key of reservation resource.
    #[builder(into, default)]
    #[serde(rename = "key")]
    pub r#key: Box<Option<String>>,
    /// Corresponds to the label values of reservation resource.
    #[builder(into, default)]
    #[serde(rename = "values")]
    pub r#values: Box<Option<Vec<String>>>,
}
