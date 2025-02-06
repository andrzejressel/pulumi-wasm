#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct InterconnectCircuitInfo {
    /// (Output)
    /// Customer-side demarc ID for this circuit.
    #[builder(into, default)]
    #[serde(rename = "customerDemarcId")]
    pub r#customer_demarc_id: Box<Option<String>>,
    /// (Output)
    /// Google-assigned unique ID for this circuit. Assigned at circuit turn-up.
    #[builder(into, default)]
    #[serde(rename = "googleCircuitId")]
    pub r#google_circuit_id: Box<Option<String>>,
    /// (Output)
    /// Google-side demarc ID for this circuit. Assigned at circuit turn-up and provided by
    /// Google to the customer in the LOA.
    #[builder(into, default)]
    #[serde(rename = "googleDemarcId")]
    pub r#google_demarc_id: Box<Option<String>>,
}
