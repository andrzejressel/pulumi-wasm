#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
pub struct GetAccountGeoLocation {
    #[builder(into)]
    #[serde(rename = "failoverPriority")]
    pub r#failover_priority: Box<i32>,
    /// The ID of the virtual network subnet.
    #[builder(into)]
    #[serde(rename = "id")]
    pub r#id: Box<String>,
    /// The name of the Azure region hosting replicated data.
    #[builder(into)]
    #[serde(rename = "location")]
    pub r#location: Box<String>,
}
