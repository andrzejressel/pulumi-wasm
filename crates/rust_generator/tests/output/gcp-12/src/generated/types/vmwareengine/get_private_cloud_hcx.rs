#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct GetPrivateCloudHcx {
    /// Fully qualified domain name of the appliance.
    #[builder(into)]
    #[serde(rename = "fqdn")]
    pub r#fqdn: Box<String>,
    /// Internal IP address of the appliance.
    #[builder(into)]
    #[serde(rename = "internalIp")]
    pub r#internal_ip: Box<String>,
    /// State of the appliance. Possible values: ["ACTIVE", "CREATING"]
    #[builder(into)]
    #[serde(rename = "state")]
    pub r#state: Box<String>,
    /// Version of the appliance.
    #[builder(into)]
    #[serde(rename = "version")]
    pub r#version: Box<String>,
}
