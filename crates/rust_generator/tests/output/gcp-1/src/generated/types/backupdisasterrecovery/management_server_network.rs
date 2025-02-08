#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ManagementServerNetwork {
    /// Network with format `projects/{{project_id}}/global/networks/{{network_id}}`
    #[builder(into)]
    #[serde(rename = "network")]
    pub r#network: Box<String>,
    /// Type of Network peeringMode
    /// Default value is `PRIVATE_SERVICE_ACCESS`.
    /// Possible values are: `PRIVATE_SERVICE_ACCESS`.
    #[builder(into, default)]
    #[serde(rename = "peeringMode")]
    pub r#peering_mode: Box<Option<String>>,
}
