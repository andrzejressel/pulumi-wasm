#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct ProfileContainerNetworkInterfaceIpConfiguration {
    /// Specifies the name of the IP Configuration.
    #[builder(into)]
    #[serde(rename = "name")]
    pub r#name: Box<String>,
    /// Reference to the subnet associated with the IP Configuration.
    #[builder(into)]
    #[serde(rename = "subnetId")]
    pub r#subnet_id: Box<String>,
}
