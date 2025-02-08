#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation)]
pub struct GetManagementServerNetwork {
    /// Network with format 'projects/{{project_id}}/global/networks/{{network_id}}'
    #[builder(into)]
    #[serde(rename = "network")]
    pub r#network: Box<String>,
    /// Type of Network peeringMode Default value: "PRIVATE_SERVICE_ACCESS" Possible values: ["PRIVATE_SERVICE_ACCESS"]
    #[builder(into)]
    #[serde(rename = "peeringMode")]
    pub r#peering_mode: Box<String>,
}
