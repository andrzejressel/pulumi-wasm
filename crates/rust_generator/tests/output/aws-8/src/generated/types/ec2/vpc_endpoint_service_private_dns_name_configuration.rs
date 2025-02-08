#[derive(pulumi_gestalt_rust::__private::serde::Deserialize, pulumi_gestalt_rust::__private::serde::Serialize, pulumi_gestalt_rust::__private::bon::Builder, Debug, PartialEq, Clone)]
#[builder(finish_fn = build_struct)]
#[allow(dead_code)]
#[allow(clippy::doc_lazy_continuation, clippy::tabs_in_doc_comments)]
pub struct VpcEndpointServicePrivateDnsNameConfiguration {
    /// Name of the record subdomain the service provider needs to create.
    #[builder(into, default)]
    #[serde(rename = "name")]
    pub r#name: Box<Option<String>>,
    /// Verification state of the VPC endpoint service. Consumers of the endpoint service can use the private name only when the state is `verified`.
    #[builder(into, default)]
    #[serde(rename = "state")]
    pub r#state: Box<Option<String>>,
    /// Endpoint service verification type, for example `TXT`.
    #[builder(into, default)]
    #[serde(rename = "type")]
    pub r#type_: Box<Option<String>>,
    /// Value the service provider adds to the private DNS name domain record before verification.
    #[builder(into, default)]
    #[serde(rename = "value")]
    pub r#value: Box<Option<String>>,
}
